use clap::{App, Arg};
use smoltcp::phy::TapInterface;
use url::Url;

mod dns;
mod http;
mod ethernet;

macro_rules! trace {
    ($($args: expr),*) => {
        print!("TRACE: file: {}, line: {}", file!(), line!());
        $(
            print!(", {}: {}", stringify!($args), $args);
        )*
        println!(""); // to get a new line at the end
    }
}

fn main() {
    let app = App::new("mget")
        .about("GET a webpage, manually")
        .arg(Arg::with_name("url").required(true))
        .arg(Arg::with_name("tap-device").required(true))
        .arg(
            Arg::with_name("dns-server")
            .default_value("1.1.1.1")
            )
        .get_matches();

    let url_text = app.value_of("url").unwrap();
    let dns_server_text =
        app.value_of("dns-server").unwrap();
    let tap_text = app.value_of("tap-device").unwrap();

    let url = Url::parse(url_text)
        .expect("error: unable to parse <url> as a URL");

    trace!();

    if url.scheme() != "http" {
        eprintln!("error: only HTTP protocol supported");
        return;
    }

    trace!();

    let tap = TapInterface::new(&tap_text)
        .expect(
            "error: unable to use <tap-device> \
            as a network interface"
            );

    trace!();

    let domain_name =
        url.host_str()
        .expect("domain name required");

    trace!();

    let _dns_server: std::net::Ipv4Addr =
        format!("{}", dns_server_text)
        .parse()
        .expect(
            "error: unable to parse <dns-server> \
            as an IPv4 address"
            );

    trace!("_dns_server {}", _dns_server);

    let addr =
        dns::resolve(dns_server_text, domain_name)
        .unwrap()
        .unwrap();

    trace!();

    let mac = ethernet::MacAddress::new().into();

    trace!();

    http::get(tap, mac, addr, url).unwrap();

    trace!();
}
