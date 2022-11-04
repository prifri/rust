#https://github.com/rust-osdev/bootimage
#cargo xrun --target your_custom_target.json [other_args] -- [qemu args]
#cargo xrun bootimage --target=fledge.json
#cargo xrun --target=fledge.json
 qemu-system-x86_64 -drive format=raw,file=target/fledge/debug/bootimage-p458-fledgeos-0.bin
