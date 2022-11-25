mkdir target
aarch64-linux-gnu-gcc -S -Wall -O2 -o target/semaphore.s semaphore.c -lpthread
aarch64-linux-gnu-gcc -Wall -O2 -o target/semaphore semaphore.c -lpthread
