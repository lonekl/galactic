extern rust_start, GDT, IDT
global _start
bits 16

; Arguments: string segment, string address, text color.
%macro println 3

	mov ax, %1
	mov es, ax
	mov ax, %2
	mov dx, %3
	mov si, 0x7ce0
	call si
; Tried here to fix wrong thing, not this was problem (problem was `mov si, [println_address]` (which existed before) some stupid relocation problem).
;	dw 0x14ff ; call [si]
;	dw 0xd6ff ; call si
;	mov word [.%4], 0x14ff
;.%4:
;	dw 0

%endmacro

section .text.early_entry

; Here bootsector program jumps in real mode, this function doesn't return.
_start:
; Check if A20 line is already enabled by bios.
	mov dx, .after_a20
	call _test_a20

	mov ax, 0x2403 ; Some bios call, I don't know what it is.
	int 15h ; Bios utility interrupt.
	jb .after_15h ; 15h not supported.
	cmp ah, 0
	jnz .after_15h ; 15h second time not supported.

	mov ax, 0x2402 ; Second time I don't know what call it is, probably getting cpu status or something.
	int 15h
	jb .after_15h ; Failed to get status.
	cmp ah, 0
	jnz .after_15h ; Failed to get status.

	cmp al, 1
	jz .after_a20 ; Already enabled.

	mov ax, 0x2401 ; Enable A20.
	int 15h
	jb .after_15h ; Failed to enable.
	cmp ah, 0
	jnz .after_15h ; Failed to enable.

.after_15h:
	mov dx, .after_a20 ; Check A20 line.
	call _test_a20

	; TODO Keyboard and fast (and maybe 0xee) method.

	println 0, _no_a20_line, 0xc
	hlt

.after_a20:
; Check if processor is 8086.
	pushf
	mov bp, sp
	test word [bp], 1 << 15 ; Test 15th bit of flags register.
	jz .is_8086

	println 0, _cpu_not_8086, 0xc

	hlt

.is_8086:
	; Check if processor is at least 32-bit.
	xor word [bp], (1 << 12) | (1 << 13) ; Try to change these bits.
	mov ax, [bp] ; Save changed flags register.

	popf ; Reload flags register.
	pushf

	xor ax, [bp] ; Xor changed flags register with new one.
	cmp ax, 0 ; Check if these bits in flags register have changed.
	popf
	je .is_32bit

	println 0, _cpu_not_32bit, 0xc

	hlt

.is_32bit:
	; Now we know it's 32-bit cpu, so we finally can use extended registers!!!
	; Check for cpuid availability.
	pushfd ; code from osdev
	pushfd

	xor dword [esp], 0x00200000 ; Change 21th bit in eflags register.

	popfd ; Reload eflags.
	pushfd

	pop eax ; Save current eflags.
	xor eax, [esp] ; Compare it to old one.

	popfd

	and eax, 0x00200000 ; Check for 21th bit.
	cmp eax, 0
	jne .cpuid_available

	println 0, _no_cpuid, 0xc

	hlt

.cpuid_available:
	; Now we can check for availability of long mode.
	mov eax, 0x80000000 ; Check last cpuid number.
	cpuid
	cmp eax, 0x80000001
	jb .no_long_mode

	mov eax, 0x80000001 ; Check for long mode.
	cpuid
	test edx, 1 << 29
	jz .no_long_mode

.long_mode_exists:
	; Now we know there is long mode.
	; So we can jump directly to it skipping protected mode.

	xor ax, ax
	mov es, ax

	; Create empty buffer for pages.
	mov ecx, 0x4000 / 4
	xor eax, eax
	mov di, PAGE_ADDRESS
	cld
	rep stosd ; Will end at 0x1000 * 4 + 0x1000 = 0x5000
	mov edi, PAGE_ADDRESS
	mov cr3, edi ; Set page address.

	; Creating primitive page table.
	mov dword [PAGE_ADDRESS         ], PAGE_ADDRESS + 0x1003
	mov dword [PAGE_ADDRESS + 0x1000], PAGE_ADDRESS + 0x2003
	mov dword [PAGE_ADDRESS + 0x2000], PAGE_ADDRESS + 0x3003

	; Create page for first 2MIB size.
	mov edi, PAGE_ADDRESS + 0x3000
	mov ecx, 512
	mov ebx, 3

; I didn't made better name matching all others.
.create_2MIB_page:
	mov [di], ebx
	add ebx, 0x1000
	add edi, 8
	loop .create_2MIB_page

.pages_created:

	; Set PAE and PGE bits in control register.
	mov eax, (1 << 5) | (1 << 7)
	mov cr4, eax

	; Setting LM bit.
	mov ecx, 0xC0000080 ; Read from EFER MSR.
	rdmsr
	or eax, 1 << 8 ; Set LM bit.
	wrmsr

	; Entering compatibility mode
	mov eax, cr0
	or eax, (1 << 31) | (1 << 0) ; Set PG bit and enable protected mode (In which we will probably never be).
	mov cr0, eax


.compatibility_mode:
	; We should be now in long sub mode (compatibility mode).
	; But for some reason protected mode instructions don't work.
	; Println macro probably can't be used now.

	; Prepare gdt pointer.
	push dword GDT
	push word 8 * 8 - 1 ; Number of entries * qword in bytes - 1

	; Load it.
	lgdt [esp]
	add esp, 4 + 2 ; Deallocate gdt pointer from stack.

	; Do far jump to long mode function.
	jmp 0x8:.long_mode

.long_mode:
	bits 64
	; We are finally in long mode.

	; Disable interrupts again, I don't know do it is needed and why.
	cli

	; Now we set rest of the segment registers to data segment.
	mov ax, 0x10
	mov ds, ax
	mov ss, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	push qword IDT ; Prepare idt pointer.
	push word 256 * 16 - 1

	; Load it.
	lidt [rsp]
	add rsp, 8 + 2 ; Deallocate idt pointer from stack.

	jmp rust_start ; Finally jump to rust code.

.no_long_mode:
	bits 16

	println 0, _no_long_mode, 0xc
	hlt

; Returns if A20 line isn't enabled, jumps to dx (and clears return address in stack) if A20 line is enabled.
; Modifies: ax, es, di, si.
; Assumes that it can address 1MIB + 16KIB - 16B, so it will crash if you would like to run it in washer machine with 1MIB ram.
_test_a20:
	mov ax, 0xffff ; Set extra segment to wrap around.
	mov es, ax

	mov di, 0x500 ; Low di.
	mov si, 0x510 ; High es:si (wrap around).

	mov byte [di], 0xae ; Setting first usable memory byte.
	mov byte [es:si], 0xea ; Setting same thing if it wraps around.

	xor ax, ax ; Reset it just for `println` function.
	mov es, ax

	cmp byte [di], 0xae ; Compare 0:0x500 with 0xae.
	je .enabled ; Skip enabling a20 line if it's already enabled.

	ret

.enabled:
	pop ax
	mov si, dx
	jmp si

_no_a20_line db "A20 line is not supported, or cannot be enabled.", 10
_cpu_not_8086 db "Processor is older than 8086, x86_64 is required.", 10
_cpu_not_32bit db "Processor is 16-bit, x86_64 is required.", 10
_no_cpuid db "No cpuid, procesor is older than i486, x86_64 is required.", 10
_no_long_mode db "Processor is x86, x86_64 is required.", 10

PAGE_ADDRESS equ 0x1000

;PAGE_PRESENT equ 1 << 0
;PAGE_WRITABLE equ 1 << 1

section .note.GNU-stack ; Just leave me alone dear ld.
