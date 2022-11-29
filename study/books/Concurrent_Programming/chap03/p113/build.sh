mkdir target
gcc -Wall -O2 -o target/test test.c ../../utils/thread_info/thread_info.c ../../utils/thread_info/thread_info.h -lpthread
