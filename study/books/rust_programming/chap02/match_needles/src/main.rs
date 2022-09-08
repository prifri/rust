fn main() {
    //let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in haystack {
        let result = match item {
            42 | 132 | 1430 => "hit!",
            /*
            * prifri, 2022.09.08:
            * - wildcard
            */
            _ => "miss",
        };

        if result == "hit" {
            println!("{} : {}", item, result);
        }
    }
}
