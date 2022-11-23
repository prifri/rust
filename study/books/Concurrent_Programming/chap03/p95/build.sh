mkdir target
aarch64-linux-gnu-gcc -Wall -O2 -o target/good_mutex good_mutex.c -lpthread
