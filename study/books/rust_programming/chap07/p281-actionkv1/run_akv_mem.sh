cargo run --bin akv_mem ./akv_mem.test get abc
cargo run --bin akv_mem ./akv_mem.test insert abc 123
cargo run --bin akv_mem ./akv_mem.test get abc
cargo run --bin akv_mem ./akv_mem.test update abc 456
cargo run --bin akv_mem ./akv_mem.test get abc
cargo run --bin akv_mem ./akv_mem.test delete abc
cargo run --bin akv_mem ./akv_mem.test get abc
