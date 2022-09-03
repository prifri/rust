fn main() {
    'outer : for x in 0.. {
        for y in 0..40 {
            for z in 0 ..10 {
                if x + y + z > 1000 {
                    println!("{} {} {}", x, y, z);
                    break 'outer
                }
            }
        }
    }

    println!("bye");
}
