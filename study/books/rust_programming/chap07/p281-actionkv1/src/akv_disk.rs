use libactionkv::ActionKV;
use std::collections::HashMap;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
    ";

#[cfg(target_os = "linux")]
const USAGE: &str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
    ";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = std::collections::HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {

/*
 * IAMROOT, 2022.10.06:
 * - INDEX_KEY
 *   database 내 인덱스의 숨겨진 내부 명칭
 */
    const INDEX_KEY: &ByteStr = b"+index";

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut a = ActionKV::open(path).expect("unable to open file");

    a.load().expect("unable to load data");

    match action {
        "get" => {

/*
 * prifri, 2022.10.06:
 * - a.index는 Option을 반환하는 hashMap이고 값 자체는 Option내에 저장되서
 * unwrap을 두번썻다는데 뭔말인지는 모르겠다.
 */
            let index_as_bytes = a.get(&INDEX_KEY)
                .unwrap()
                .unwrap();

            let index_decoded = bincode::deserialize(&index_as_bytes);
            let index: HashMap<ByteString, u64> = index_decoded.unwrap();

            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&i) => {
                    let kv = a.get_at(i).unwrap();
                    println!("{:?}", kv.value);
                }
            }
        },

        "delete" => a.delete(key).unwrap(),

        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.insert(key, value).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        },

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.update(key, value).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        },
        _ => eprintln!("{}", &USAGE),
    }
}

