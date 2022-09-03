use std::collections::HashMap;

fn test(s: &String) -> usize {
    s.len()
}

fn main() {
    //let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    //let doest_not_exist = &v[100];
    let doest_not_exist = v.get(100);

    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    for i in &mut v {
        *i += 1;
        println!("{} {}", i, *i);
    }

    let data = "init content";
    let s = data.to_string();

    println!("{}", s);

    let mut s = s.to_string();
    s.push_str("abc");
    s.push('1');

    println!("{}", s);

    let s2 = String::from(" s2");
    let s3 = s + &s2;
    let s4 = s2;
    let a: u32 = 1;
    let b = a;

    //println!("{}", test(&s2));
    println!("{}", test(&s4));
    //println!("{} {} {} {} {}", s2, s3, s4, a, b);
    println!("{} {} {} {}", s3, s4, a, b);


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),  10);
    scores.insert(String::from("Red"), 50);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    for i in &teams {
        println!("teams {}", i);
    }

    for i in &teams {
        println!("teams {}", i);
    }

    for i in &initial_scores {
        println!("scores {}", i);
    }

    let socres: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for i in &teams {
        println!("teams {}", i);
    }

    for i in &initial_scores {
        println!("scores {}", i);
    }

    let mut socres: HashMap<String, u32> = HashMap::new();

    let field_name = String::from("fa");
    let field_name2 = 3;

    println!("{} {}", field_name, field_name2);

    scores.insert(field_name, field_name2);

    //println!("{} {}", field_name, field_name2);
    
    let mut socres = HashMap::new();

    socres.insert(String::from("Blue"), 10);
    socres.insert(String::from("Red"), 20);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name);

    let mut i = 0;
    for (key, value) in &socres {
        i += 1;
        println!("{} {} {}", i, key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (text, value) in map {
        println!("{} {}", text, value);
    }
}
