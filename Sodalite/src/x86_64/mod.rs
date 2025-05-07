//! Code directly related to X86_64 architecture.

pub use x86_64::{ halt, end_exec, inb, inw, ind, outb, outw, outd, simple_wait};

use x86_64::ChainedPics;
use x86_64::tables::{ Gdt, Idt, IdtEntry, InterruptHandler, IO_DEFAULT, Tss};

mod interrupts;

#[ no_mangle]
static mut GDT: Gdt = {
	use x86_64::tables::gdf::*;
	let mut gdt = Gdt::new();

	let _ = gdt.push( KERNEL_CODE); // 0x08
	let _ = gdt.push( KERNEL_DATA); // 0x10

	gdt
};

#[ no_mangle]
static mut IDT: Idt = {
	let idt = Idt::new( [ IdtEntry::empty(); 256]);

	idt
};

static mut TSS: Tss = {
	let tss = Tss::new( [ 0, 0, 0], [ 0x1000; 7], 0);

	tss
};

static PICS: ChainedPics = ChainedPics::new( 32, 32 + 8);

/// Should be called only once.
pub unsafe fn initialize() {

	IDT.table[ 3] = IdtEntry::new( interrupts::breakpoint as * const InterruptHandler as u64, 8, IO_DEFAULT);
	IDT.table[ 14] = IdtEntry::new( interrupts::page_fault as * const InterruptHandler as u64, 8, IO_DEFAULT);
	IDT.table[ PICS.irq_index( 0) as usize] = IdtEntry::new( interrupts::timer as * const InterruptHandler as u64, 8, IO_DEFAULT);
	IDT.table[ PICS.irq_index( 1) as usize] = IdtEntry::new( interrupts::keyboard as * const InterruptHandler as u64, 8, IO_DEFAULT);

	let tss_segment = GDT.push_tss( &TSS);
	tss_segment.load_tss();
	x86_64::make_breakpoint();

	PICS.reinitialize();
	x86_64::enable_interrupts();

}
