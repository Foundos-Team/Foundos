cargo build --release
nasm foundos_bootloader/asm/mbr.asm -f bin -o mbr.bin
nasm foundos_bootloader/asm/kernel_entry.asm -f elf -o kernel_entry.o
ld -m elf_i386 -o kernel.bin -Ttext 0x1000 kernel_entry.o target/x86_64_foundos_target/release/libfoundos_kernel.rlib --oformat binary
cat mbr.bin kernel.bin > os-image.bin
qemu-system-i386 -fda os-image.bin
