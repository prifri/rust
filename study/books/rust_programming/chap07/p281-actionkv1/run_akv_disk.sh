cargo run --bin akv_disk ./akv_disk.test get abc
cargo run --bin akv_disk ./akv_disk.test insert abc 123
cargo run --bin akv_disk ./akv_disk.test get abc
cargo run --bin akv_disk ./akv_disk.test update abc 456
cargo run --bin akv_disk ./akv_disk.test get abc
cargo run --bin akv_disk ./akv_disk.test delete abc
cargo run --bin akv_disk ./akv_disk.test get abc
