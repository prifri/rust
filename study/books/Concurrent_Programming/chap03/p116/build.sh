mkdir target

OPT=-D$1=true
echo $OPT
gcc -Wall ${OPT} -O2 -o target/test test.c -lpthread
