fn main() {

/*
 * prifri, 2022.08.28:
 * - \ : 뒤에오는 줄바꿈을 피한다.
 */
    let pengun_data = "\
    common name, length (cm)
    little penguin, 33
    Yellow-eyed penguin, 75
    Fiordland pengun, 60
    invalid, data";

    let records = pengun_data.lines();

    for (i, record) in records.enumerate() {
/*
 * prifri, 2022.08.28:
 * - i == 0 : coomon name, length skip
 * - record.trim().len() 공백이 있는거 스킵. 여기선 i == 0 만 스킵될것.
 */
        if cfg!(debug_assertions) {
            println!("i {} | record {} | record.trim() {} | record.trim().len {}\n",
            i, record, record.trim(), record.trim().len());
        }
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

/*
 * prifri, 2022.08.28:
 * - record를 field로 나눈다.
 */
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();

/*
 * prifri, 2022.08.28:
 * - program release version일때는 미포함 된다.
 */
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}",
                      record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}
