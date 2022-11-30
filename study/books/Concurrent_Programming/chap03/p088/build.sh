mkdir target
gcc -S -O2 -o target/asm_x86-64.s test.c
aarch64-linux-gnu-gcc -S -O2 -o target/asm_arm64.s test.c 
