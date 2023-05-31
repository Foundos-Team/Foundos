cargo build --release
nasm asm/mbr.asm -f bin -o mbr.bin
nasm asm/kernel_entry.asm -f elf -o kernel_entry.o
C:\i686-elf-tools-windows\bin\i686-elf-ld -m elf_i386 -o kernel.bin -Ttext 0x1000 kernel_entry.o libfoundos_kernel.o --oformat binary