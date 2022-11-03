#https://livebook.manning.com/book/rust-in-action/chapter-11/v-14/11
rustc --print sysroot
find $(rustc --print sysroot) -type f -name 'llvm-*' -printf '%f\n'
cargo xbuild --help
