#[cfg(windows)]
use kernel32;
#[cfg(not(windows))]
use libc;
#[cfg(windows)]
use winapi;

use byteorder::{BigEndian, ReadBytesExt};
use chrono:: {
    DateTime, Duration as ChronoDuration, TimeZone, Timelike,
};
use chrono::{Local, Utc};
use clap::{App, Arg};
use std::mem::zeroed;
use std::net::UdpSocket;
use std::time::Duration;

/*
 * PRIFRI, 2022.10.07:
 * - 4byte * 12
 */
const NTP_MESSAGE_LENGTH: usize = 4 * 12;
const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;
/*
 * PRIFRI, 2022.10.07:
 * - ntp 기본포트 12300
 */
const LOCAL_ADDR: &'static str = "0.0.0.0:12300";

#[derive(Default, Debug, Copy, Clone)]
struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

struct NTPMessage {
    data: [u8; NTP_MESSAGE_LENGTH],
}

#[derive(Debug)]
struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

impl NTPResult {
/*
 * PRIFRI, 2022.10.07:
 * - delay = (T4 - T1) - (T3 - T2)
 *          = T4 - T1 - T3 + T2
 *          = T2 - T1 + T4 - T3
 *          = (T2 - T1) + (T4 - T3)
 *  즉 delay에 /2만 해주면 offset이 나온다.
 */
    fn offset(&self) -> i64 {
        let delta = self.delay();
        delta.abs() / 2
    }

    fn delay(&self) -> i64 {
/*
 * PRIFRI, 2022.10.07:
 *    vsend ~ recv까지의 시간  vrecv ~ send까지의 시간
 * - (client 소비시간) - (server 소비 시간) = clock간 차이에 대한 추정치
 *  + 네트워크 트래픽 및 처리로 인한 지연
 * - (send 시간 + recv시간 + client 처리 시간) - (server 처리시간)
 */
        let duration = (self.t4 - self.t1) - (self.t3 - self.t2);
        duration.num_milliseconds()
    }
}
/*
 * PRIFRI, 2022.10.07:
 * - chrono는 10^(-9), ntp는 250 * 10^(-12)의 정밀도를 가진다.
 *   그 정밀도에 대하여 연산해준다.
 */
impl From<NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: NTPTimestamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        let mut nanos = ntp.fraction as f64;
        nanos *= 1e9;
        nanos /= 2_f64.powi(32);

        Utc.timestamp(secs, nanos as u32)
    }
}

impl From<DateTime<Utc>> for NTPTimestamp {
    fn from(utc:DateTime<Utc>) -> Self {
        let secs = utc.timestamp() + NTP_TO_UNIX_SECONDS;
        let mut fraction = utc.nanosecond() as f64;
        fraction *= 2_f64.powi(32);
        fraction /= 1e9;

        NTPTimestamp {
            seconds: secs as u32,
            fraction: fraction as u32,
        }
    }
}

impl NTPMessage {
    fn new() -> Self {
        NTPMessage {
            data: [0; NTP_MESSAGE_LENGTH],
        }
    }

    fn client() -> Self {
        const VERSION: u8 = 0b00_011_000;
        const MODE: u8    = 0b00_000_011;

        let mut msg = NTPMessage::new();

        msg.data[0] |= VERSION | MODE;
        msg
    }
/*
 * PRIFRI, 2022.10.07:
 * - 인자 i byte부터 8byte를 가져온다.
 */
    fn parse_timestamp(
        &self,
        i: usize,
        ) -> Result<NTPTimestamp, std::io::Error> {
        let mut reader = &self.data[i..i + 8];
        let seconds = reader.read_u32::<BigEndian>()?;
        let fraction = reader.read_u32::<BigEndian>()?;

        Ok(NTPTimestamp {
            seconds,
            fraction,
        })
    }

    fn rx_time(
        &self
        ) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(32)
    }

    fn tx_time(
        &self
        ) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(40)
    }
}

/*
 * PRIFRI, 2022.10.07:
 * - weight비중을 넣는다. 시간 * weight / weight 이니 최종적으로 weight가
 *   고려된 평균 시간이 return.
 */
fn weighted_mean(values: &[f64], weights: &[f64]) -> f64 {
    let mut result = 0.0;
    let mut sum_of_weights = 0.0;

/*
 * PRIFRI, 2022.10.07:
 * - zip : 한쌍으로 iter를 얻어올수있다.
 * - weight값이 작았던건 v,w 쌍은 v*w값도 작게 나올것이다.
 *   나눌때에도 영향을 적게 받을것이다.
 */
    for (v, w) in values.iter().zip(weights) {
        result += v * w;
        sum_of_weights += w;
    }
/*
 * PRIFRI, 2022.10.07:
 * - (v1w1 + v2w2 + v3w3 + v4w4) / (w1 + w2 + w3 + w4)
 * - ex) w1이 작고, w2,w3,w4가 크다면
 *   (v1w1 + v2w2 + v3w3 + v4w4) / (w1 + w2 + w3 + w4)
 *    (10    + 10000+  11000 + 12000) / (1 + 100 + 1000 + 1000)
 *    이런식으로 영향을 적게받는다.
 */
    result / sum_of_weights
}

fn ntp_roundtrip(
    host: &str,
    port: u16,
    ) -> Result<NTPResult, std::io::Error> {
    let destination = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(1);

    let request = NTPMessage::client();
    let mut response = NTPMessage::new();

/*
 * PRIFRI, 2022.10.07:
 * - t1이 ntp server에서 딱히 검사를 안하므로 message에 담지도 않는것이
 *   확인된다.
 */
    let message = request.data;

    let udp = UdpSocket::bind(LOCAL_ADDR)?;
    udp.connect(&destination).expect("unable to connect");

    let t1 = Utc::now();
/*
 * PRIFRI, 2022.10.07:
 * - 그냥 빈데이터를(헤더만) 보낸다.
 */
    udp.send(&message)?;
    udp.set_read_timeout(Some(timeout))?;
    udp.recv_from(&mut response.data)?;
    let t4 = Utc::now();

/*
 * PRIFRI, 2022.10.07:
 * - 위에 From impl구현으로 인해 into 호출이 가능하다.
 *   rx_time, tx_time은 NTPTimestamp을 return하지만 이 into로 DateTime으로
 *   변환되는게 그 때문이다.
 */
    let t2: DateTime<Utc> =
        response
        .rx_time()
        .unwrap()
        .into();
    let t3: DateTime<Utc> =
        response
        .tx_time()
        .unwrap()
        .into();

    Ok(NTPResult {
        t1, t2, t3, t4,
    })
}

/*
 * PRIFRI, 2022.10.07:
 * - weight값을 고려한 평균값을 return.
 */
fn check_time() -> Result<f64, std::io::Error> {
    const NTP_PORT: u16 = 123;

    let servers = [
        "time.nist.gov",
        "time.apple.com",
        "time.euro.apple.com",
        "time.google.com",
        "time2.google.com",
        //"time.windows.com",
    ];

    let mut times = Vec::with_capacity(servers.len());

    //check_error();

    for &server in servers.iter() {
        print!("{} =>", server);

        let calc = ntp_roundtrip(&server, NTP_PORT);

        match calc {
            Ok(time) => {
                println!(" {}ms away from local system time", time.offset());
                times.push(time);
            }
            Err(_) => {
                println!(" ? [response took too long]")
            }
        };
    }

    //check_error();

    let mut offsets = Vec::with_capacity(servers.len());
    let mut offset_weights = Vec::with_capacity(servers.len());

/*
 * PRIFRI, 2022.10.07:
 * - vec을 만들고 순서대로 offset과 delay를 push한다.
 */
    for time in &times {
        let offset = time.offset() as f64;
        let delay = time.delay() as f64;

/*
 * PRIFRI, 2022.10.07:
 * - 느릴수록 weight값이 작아진다.
 */
        let weight = 1_000_000.0 / (delay * delay);

        if weight.is_finite() {
            offsets.push(offset);
            offset_weights.push(weight);
        }
    }

    let avg_offset = weighted_mean(&offsets, &offset_weights); 
    Ok(avg_offset)
}

struct Clock;
/*
 * PRIFRI, 2022.10.07:
 * - p384랑 같다.
 */
impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use chrono::Weekday;
        use kernel32::SetSystemTime;
        use winapi::{SYSTEMTIME, WORD};

        let t = t.with_timezone(&Local);

        let mut systime: SYSTEMTIME = unsafe { zeroed() };

        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        };

        let mut ns = t.nanosecond();
        let is_leap_second = ns > 1_000_000_000;

        if is_leap_second {
            ns -= 1_000_000_000;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = t.second() as WORD;
        systime.wMilliseconds = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            SetSystemTime(systime_ptr);
        }
    }

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::{timeval, time_t, suseconds_t};
        use libc::{settimeofday, timezone};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        println!("123 t {}, u {}.{}", t, u.tv_sec, u.tv_usec);
        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
        println!("456");
    }
}

fn check_error() {
    let maybe_error =
        std::io::Error::last_os_error();
    let os_error_code =
        &maybe_error.raw_os_error();

    match os_error_code {
        Some(0) => (),
        None => (),
        Some(_) => eprintln!("Unable to set the time: {:?}",
                             maybe_error),
    }
}

fn main() {
    let app = App::new("clock")
        .version("0.1.2")
        .about("Gets and (aspirationally) sets the time.")
        .after_help(
            "Note: UNIX timestamps ar parsed as whole \
            seconds since 1st January 1970 0:00:00 UTC. \
            For more accuracy. use another format.",
            )
        .arg(
            Arg::with_name("action")
            .takes_value(true)
            .possible_values(&["get", "set", "check-ntp"])
            .default_value("get"),
            )
        .arg(
            Arg::with_name("std")
            .short("s")
            .long("use-standard")
            .takes_value(true)
            .possible_values(&[
                             "rfc2822",
                             "rfc3339",
                             "timestamp",
            ])
            .default_value("rfc3339"),
            )
        .arg(Arg::with_name("datetime").help(
                "When <action> is 'set', apply <datetime>. \
                Otherwise, ignore.",
                ));

    let args = app.get_matches();
    
    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();

    if action == "set" {
        let t_ = args.value_of("datetime").unwrap();

        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };

        let err_msg = format!(
            "Unalbe to parse {} according to {}",
            t_, std
            );
        let t = parser(t_).expect(&err_msg); 
        Clock::set(t);

    } else if action == "check-ntp" {
        let offset = check_time().unwrap() as isize;
/*
 * PRIFRI, 2022.10.07:
 * - 부호(offset) * min(abs(offset), 200) / 5.
 *   너무 한번에 안바뀌고 최대 40ms씩 바뀌라는 거..
 */
        let adjust_ms_ = offset.signum() * offset.abs().min(200) / 5;
        let adjust_ms = ChronoDuration::milliseconds(adjust_ms_ as i64);

        let now: DateTime<Utc> = Utc::now() + adjust_ms;

        Clock::set(now);
    }

    check_error();
    let now = Clock::get();

    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
