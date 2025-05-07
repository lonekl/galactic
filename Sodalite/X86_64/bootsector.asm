[org 0x7c00]
bits 16

; Just because I don't remember which is which.
; column = x
; row = y

_start:
	cli ; Disabling interrupts so keyboard presses or pit in perfect moment won't eventually fuck it up, mainly to just use `hlt` to stop execution.
	jmp 0:.0 ; Some bioses jmp to 0x7c0:0 and others to 0:0x7c00, this really can't be same on all bioses? For some reason I need to care about it.

.0:
	xor ax, ax ; Reset extra segment.
	mov es, ax

	mov sp, stack_address
	mov [drive_number], dl ; Save booted drive number for reading later.

	mov ah, 0xf ; Get video mode.
	int 10h ; Bios call, al = video mode, ah = character columns, bh = current page.
	mov [text_page], bh ; Save output values.
	mov [text_columns], ah

	mov ah, 3h ; Get cursor position, column already in bl.
	int 10h ; Bios call, ch = start scam line, cl = end scam line, dh = row, dl = column.
	mov [cursor_row], dh
	mov [cursor_column], dl

	mov dx, 0xa; Highlight color.
	mov ax, hello_msg_str ; String address.
	call println


	; Now actually reading rest of the bootloader from disk.


	mov ah, 2 ; Read sectors.
	mov cl, [load_first_sector_offset] ; Sector to start reading.
	mov al, 64 ; Inverted sectors to read.
	sub al, cl ; Sectors to read.
	xor ch, ch ; Cylinder, probably always will be zero while reading rest of the bootloader from disk.
	mov dh, [load_head_offset] ; Head, zero on first reading.
	mov dl, [drive_number] ; Restore drive number.
	mov bx, main_bootloader_address ; Load boot runner here.
	int 13h ; Bios call, carry flag = error or not, ah = error code, al = actual sectors read.

	jc disk_read_error ; Display error if failed to read from disk.

	mov cl, 1 ; Sector to start reading, won't change during reading.

.disk_read_loop:
	mov ax, es ; Load es.
	add ax, (512 * 63) >> 4 ; Add segment.
	mov es, ax ; Get es back.
	mov ah, 2 ; Read sectors from disk.
	mov al, 62 ; Sectors to read.
	int 13h ; Bios call, carry flag = error or not, ah = error code, al = actual sectors read.

	jc disk_read_error ; Display error if failed to read from disk.

	inc dh ; Increment current head.
	cmp dh, [head_to_stop_reading] ; Compare it to all heads that are needed to be read.
	jl .disk_read_loop ; Repeat if current head is less than all heads we need to read.

	;xor ax, ax
	;mov es, ax
	;mov ax, 0x7e00
	;call println
	;hlt

	jmp main_bootloader_address


	text_page equ 0x7bff ; db, some variables
	text_columns equ 0x7bfe ; db
	cursor_row equ 0x7bfd ; db
	cursor_column equ 0x7bfc ; db
	drive_number equ 0x7bfb ; db

	stack_address equ 0x7bfa

	main_bootloader_address equ 0x7e00


times 211 - ($ - $$) db 0
load_first_sector_offset db 2 ; 0xd3, some Sodalite settings.
load_head_offset db 0 ; 0xd4
head_to_stop_reading db 0 ; 0xd5
blue_milk_signature db 0xac, 0xdc, 0x01, 0x02 ; 0xd6, can be used by other programs to detect Sodalite.
dw 0 ; Disk signature.
db 0, 0, 0, 0 ; Timestamp.
; Next part of code here:
; starting at 224, or 0xe0.


; Can be called in real (or virtual) mode at address 0x7ce0.
; Put values: text start address in es:ax, vga text color in dx.
; Text should end with enter (10 byte).
; Values changed are: ax, dx, bx, si, ds, ip, cx (maybe).
println:
	push ax ; Save arguments for later.
	push dx

	mov dl, [cursor_column] ; Get cursor column back.
	mov dh, [cursor_row] ; Get cursor row back.
	inc dh
	cmp dh, [text_columns] ; cursor_column + 1 < text_columns
	jl .0 ; Skip scrolling terminal if cursor isn't at the end.

	mov ah, 7h ; Scroll down.
	mov al, 1 ; One line.
	mov bh, 0x08 ; New line color.
	int 10h ; Bios call, probably none of the registers are modified.
	dec dh ; As it has scrolled one line, cursor column should also be deceased.

.0:
	mov ah, 2h ; Set cursor position.
	mov bh, [text_page]
	int 10h ; Bios call, probably none of the registers are modified.
	mov [cursor_row], dh

	dec dh ; Set correct column (that before cursor position has changed).
	xor bx, bx ; Zero bx.
	mov bl, dh ; Set bx to cursor row.

	mov ax, 80 ; Set ax to terminal width.
	mul bx ; Multiply ax by bx, output in ax (and overflow in dx) (column * 80).
	mov bx, ax ; Move multiplication output.

	xor ax, ax ; Zero ax.
	mov al, [cursor_column] ; Set ax to column.

	add bx, ax ; Add rows to columns (column * 80 + row).

	shl bx, 1 ; Shift one left as one vga character has 2 bytes not 1.

	pop dx ; Get arguments back.
	pop si ; Ax as si because ax can't be used as pointer in real mode.

.printlning_loop:

	mov cl, [si] ; Get current text character.
	cmp cl, 10 ; End loop if character is enter.
	je .return

	mov ax, 0xb8000 >> 4 ; Set data register.
	mov ds, ax

	mov [bx], cl ; Write text to vga.
	inc bx
	mov [bx], dl
	inc bx

	mov ax, es ; Set string data segment.
	mov ds, ax

	inc si ; And increase character pointer.

	jmp .printlning_loop ; Repeat.

.return:
	ret

disk_read_error:
	mov dx, 0xc
	mov ax, disk_read_error_str
	call println
	hlt


disk_read_error_str db "Failed to read booting disk.", 10
hello_msg_str db "Bios Sodalite booting!", 10

times 440 - ($ - $$) db 0
dd 0 ; Disk signature.
dw 0

dq 0, 0 ; Partitions.
dq 0, 0
dq 0, 0
dq 0, 0

dw 0xaa55 ; Bios bootable signature.
