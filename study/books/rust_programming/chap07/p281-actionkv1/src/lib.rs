use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, SeekFrom};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;

/*
 * prifri, 2022.10.06:
 * - serde_derive = "1", serde = "1" 추가
 */
use serde_derive::{Deserialize, Serialize};

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(
        path: &Path
        ) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::new();
        Ok(ActionKV { f, index })
    }

/*
 * PRIFRI, 2022.10.06:
 * - curr pos에서 checksum, key_len, val_len, data_len, key, value를 읽어서
 *   무결성 검사를 수행하고 curr pos를 갱신후 얻은 key, value를 return 한다.
 */
    fn process_record<R: Read>(
        f: &mut R
        ) -> io::Result<KeyValuePair> {
/*
 * PRIFRI, 2022.10.06:
 * - 4byte -> 4byte -> 4byte 로 읽는다. 읽을때마다 seek는 갱신된다.
 */
        let saved_checksum =
            f.read_u32::<LittleEndian>()?;
        let key_len =
            f.read_u32::<LittleEndian>()?;
        let val_len =
            f.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);

/*
 * PRIFRI, 2022.10.06:
 * - data_len만큼 읽으면서 seek를 갱신하는 방법. 정확한건 모르겠다.
 *   data = |key| + |val|
 *   checksum은 key, val 합쳐서 계산하게 되있으므로 한번에 계산한다.
 */
        {
            f.by_ref()
                .take(data_len as u64)
                .read_to_end(&mut data)?;
        }
        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!(
                "data corruption encounted ({:08x} 1= {:08x}",
                checksum, saved_checksum
                );
        }
/*
 * PRIFRI, 2022.10.06:
 * - key가 먼저 위치해있으므로 key를 자른다.
 */
        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value} )
    }

    pub fn seek_to_end(&mut self) -> io::Result<u64> {
        self.f.seek(SeekFrom::End(0))
    }

/*
 * PRIFRI, 2022.10.06:
 * - file에서
 */
    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);

        loop {
/*
 * PRIFRI, 2022.10.06:
 * - 물음표 연산에 대한 설명
 *   https://rinthel.github.io/rust-lang-book-ko/ch09-02-recoverable-errors-with-result.html
 *
 * - seek(SeekFrom::Current(X))
 *   curr pos에서 X번이후를 return. 0인경우 현재 위치.
 *
 * - key와 pos를 쌍으로 hash에 저장해놓는다.
 */
            let current_position = f.seek(SeekFrom::Current(0))?;

            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => {
                            break;
                        }
                        _ => return Err(err),
                    }
                }
            };

            self.index.insert(kv.key, current_position);
        }
        Ok(())
    }

/*
 * PRIFRI, 2022.10.06:
 * - hash에는 key, pos가 pair로 저장되있다. key로 pos를 퍼온다.
 */
    pub fn get(
        &mut self,
        key: &ByteStr
        ) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position,
        };

        let kv = self.get_at(position)?;

        Ok(Some(kv.value))
    }

/*
 * PRIFRI, 2022.10.06:
 * - pos로 file stream에서 key, value를 퍼온다.
 */
    pub fn get_at(
        &mut self,
        position: u64
        ) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut f)?;

        Ok(kv)
    }

/*
 * PRIFRI, 2022.10.06:
 * - stream에서 target을 loop돌며 찾는다. 찾기만 하는데도 process_record내부에서
 *   pass한 값들에 대해 일일히 checksum을 계산하니 별 좋은건 아닌듯..
 */
    pub fn find(
        &mut self,
        target: &ByteStr
        ) -> io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.f);

        let mut found: Option<(u64, ByteString)> = None;

        loop {
            let position = f.seek(SeekFrom::Current(0))?;
            
            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => {
                            break;
                        }
                        _ => return Err(err),
                    }
                }
            };

            if kv.key == target {
                found = Some((position, kv.value));
            }
/*
 * PRIFRI, 2022.10.06:
 * - key가 혹시나 동일한 경우가 있을경우 대비한 방어인듯 싶은데 그냥
 *   break쳐도 될거같다.
 */
            //키를 덮어쓸 경우를 대비해 파일의 끝가지 반복하는것이 중요.
        }

        Ok(found)
    }

/*
 * PRIFRI, 2022.10.06:
 * - |key_len| + |val_len|을 저장할 buffer를 하나 만들어서 넣어놓고,
 *   실제 stream엔 f.seek(..)부터 해서 넣기 시작한다.
 */
    pub fn insert_buf_ignore_index(
        &mut self,
        key: &ByteStr,
        value: &ByteStr
        ) -> io::Result<u64> {
        let mut f = BufWriter::new(&mut self.f);

        let key_len = key.len();
        let val_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + val_len);

        for byte in key {
            tmp.push(*byte);
        }

        for byte in value {
            tmp.push(*byte);
        }

        let checksum = crc32::checksum_ieee(&tmp);

        let next_byte = SeekFrom::End(0);
        let current_position = f.seek(SeekFrom::Current(0))?;
        //println!("next_byte {:?} checksun {:08x} key_len {} val_len {} tmp {:x?}",
        //         next_byte, checksum, key_len, val_len, tmp);
        f.seek(next_byte)?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(val_len as u32)?;
        f.write_all(&tmp)?;

        Ok(current_position)
    }

    pub fn insert(
        &mut self,
        key: &ByteStr,
        value: &ByteStr
        ) -> io::Result<()> {
        let position = self.insert_buf_ignore_index(key, value)?;

        //println!("insert key {:x?} value {:x?} position {:?}",
        //         key, value, position);
        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    #[inline]
    pub fn update(
        &mut self,
        key: &ByteStr,
        value: &ByteStr
        ) -> io::Result<()> {
        self.insert(key, value)
    }

    #[inline]
    pub fn delete(
        &mut self,
        key: &ByteStr
        ) -> io::Result<()> {
        println!("delete {:?}", key);
        self.insert(key, b"")
    }
}
