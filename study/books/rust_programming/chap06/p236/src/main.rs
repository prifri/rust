fn string_only(password: String) -> bool {
    password.len() > 5
}

/*
 * PRIFRI, 2022.10.04:
 * - AsRef<Str> 구현체를 가지고 있는 타입 이면 받는다는것.
 *   String, str둘다 있으므로 둘다 가능
 */
fn string_n_str<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}

/*
 * PRIFRI, 2022.10.04:
 * - String으로 변환할수있는 type만 받겟다는것.
 */
fn string_info<T: Into<String>>(passowrd: T) -> bool {
    passowrd.into().len() > 5
}

fn main() {
    let pw_string = String::from("justok");
    let string_only = string_only(pw_string);
    println!("string {}", string_only);
    let pw = "justok";
    let pw_string = String::from("justok");
    let string_n_str_string = string_n_str(pw_string);
    let string_n_str_str = string_n_str(pw);
    println!("string2 {} {}", string_n_str_string, string_n_str_str);
    let pw = "justok";
    let pw_string = String::from("justok");
    let string_n_str_string = string_info(pw_string);
    let string_n_str_str = string_info(pw);
    println!("string3 {} {}", string_n_str_string, string_n_str_str);
}
