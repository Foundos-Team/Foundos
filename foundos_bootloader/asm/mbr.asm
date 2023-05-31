; Use 16bit real mode
[bits 16]
[org 0x7c00]

KERNEL_OFFSET equ 0x1000

mov [BOOT_DRIVE], dl

mov bp, 0x9000
mov sp, bp

; Load the kernel into KERNEL_OFFSET
call load_kernel
call switch_to_32bit

jmp $

%include "asm/disk.asm"
%include "asm/gdt.asm"
%include "asm/switch_to_32bit.asm"

[bits 16]
load_kernel:
    mov bx, KERNEL_OFFSET
    mov dh, 2
    mov dl, [BOOT_DRIVE]
    call disk_load
    ret

[bits 32]
BEGIN_32BIT:
    call KERNEL_OFFSET
    jmp $

BOOT_DRIVE db 0

times 510 - ($-$$) db 0

; magic number
dw 0xaa55