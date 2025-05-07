#![ allow( unused)]

use core::arch::asm;
use core::mem::{ /*transmute, */size_of};

use crate::Segment;

pub mod page {

	pub const PRESENT: u32 = 1 << 0;
	pub const WRITABLE: u32 = 1 << 1;

}

/// global descriptor fields
pub mod gdf {

	pub const RING_POS: u64 = 45;

	pub const ACCESSED: u64 = 1 << 40;
	pub const WRITABLE: u64 = 1 << 41;
	pub const CONFORMING: u64 = 1 << 42;
	pub const EXECUTABLE: u64 = 1 << 43;
	pub const USER_SEGMENT: u64 = 1 << 44;

	pub const RING_0: u64 = 0 << RING_POS /* Left shift nothing. */;
	pub const RING_1: u64 = 1 << RING_POS;
	pub const RING_2: u64 = 2 << RING_POS;
	pub const RING_3: u64 = 3 << RING_POS;

	pub const PRESENT: u64 = 1 << 47;
	pub const AVAILABLE: u64 = 1 << 52;
	pub const LONG_MODE: u64 = 1 << 53;
	pub const COMPATIBILITY_MODE: u64 = 1 << 54;
	pub const GRANULARITY: u64 = 1 << 55;

	pub const LIMIT_0_15: u64 = 0xFFFF;
	pub const LIMIT_16_19: u64 = 0xF << 48;
	pub const BASE_0_23: u64 = 0xFFFFFF << 16;
	pub const BASE_24_31: u64 = 0xFF << 56;

	pub const MINIMAL: u64 = (
		USER_SEGMENT
			| PRESENT
			| ACCESSED
			| LIMIT_0_15
			| LIMIT_16_19
			| GRANULARITY
	);

	pub const KERNEL_CODE: u64 = MINIMAL | EXECUTABLE | LONG_MODE;
	pub const KERNEL_DATA: u64 = MINIMAL | WRITABLE | COMPATIBILITY_MODE;

}

pub const IO_MINIMAL: u16 = 0b0000111000000000;
pub const IO_PRESENT: u16 = 0b1000000000000000;
pub const IO_DEFAULT: u16 = IO_MINIMAL | IO_PRESENT;

#[ repr( C, packed)]
pub struct Gdt {

	pub table: [ u64; 8],
	pub length: usize,

}

impl Gdt {

	pub const fn new() -> Gdt {

		Gdt { table: [ 0; 8], length: 1 }
	}

	pub const fn push( &mut self, desc: u64) -> Segment {
		let index = self.length as u16;

		self.table[ self.length] = desc;
		self.length += 1;

		Segment::new( index as u16, desc)
	}

	pub const fn push_sys_seg( &mut self, desc0: u64, desc1: u64) -> Segment {
		let segment = self.push( desc0);
		let _ = self.push( desc1);

		segment
	}

	pub fn push_tss( &mut self, tss: &'static Tss) -> Segment {
		let ptr = tss as * const _ as u64;
		let desc1 = ptr >> 32;
		let mut desc0 = gdf::PRESENT;
		desc0 |= ptr << 16 & 0b1111111111111111111111110000000000000000;
		desc0 |= ptr & 0b11111111000000000000000000000000 << 32 /* 56 - 24 */;
		desc0 |= ( size_of::< Tss>() - 1) as u64 & 0b1111111111111111;
		desc0 |= 0b1001 << 40;

		self.push_sys_seg( desc0, desc1)
	}

	pub unsafe fn load( &'static self) {

		let pointer = Pointer::new( self as * const _ as u64, ( self.length * size_of::< u64>() - 1) as u16);
		pointer.load_gdt();

	}

}

#[ repr( C, align( 16))]
pub struct Idt {

	pub table: [ IdtEntry; 256],

}

impl Idt {

	pub const fn new( table: [ IdtEntry; 256]) -> Self {

		Idt { table }
	}

	pub unsafe fn load( &mut self) {

		let pointer = Pointer::new( self as * const _ as u64, ( self.table.len() * size_of::< IdtEntry>() - 1) as u16);
		pointer.load_idt();

	}

}

pub type InterruptHandler = extern "x86-interrupt" fn( IntStackFrame);
pub type DoubleFaultHandler = extern "x86-interrupt" fn( IntStackFrame) -> !;

#[ repr( C, packed)]
pub struct IdtEntry {

	offset_low: u16,
	seg_select: u16,
	options: u16,
	offset_med: u16,
	offset_high: u32,
	reserved: u32,

}

impl IdtEntry {

	pub const fn empty() -> Self {

		IdtEntry { offset_low: 0, seg_select: 0, options: 0, offset_med: 0, offset_high: 0, reserved: 0 }
	}

	pub const fn new( fn_addr: u64, seg_select: u16, options: u16) -> Self {

		IdtEntry {
			offset_low: fn_addr as u16,
			seg_select,
			options,
			offset_med: ( fn_addr >> 16) as u16,
			offset_high: ( fn_addr >> 32) as u32,
			reserved: 0
		}
	}

}

impl Clone for IdtEntry {
	fn clone( &self) -> Self {

		Self {
			offset_low: self.offset_low,
			seg_select: self.seg_select,
			options: self.options,
			offset_med: self.offset_med,
			offset_high: self.offset_high,
			reserved: 0,
		}
	}
}

impl Copy for IdtEntry {}

#[ repr( C, packed)]
pub struct Tss {

	reserved0: u32,
	pub privilege_stack_table: [ u64; 3],
	reserved1: u64,
	pub interrupt_stack_table: [ u64; 7],
	reserved2: u64,
	reserved3: u16,
	pub iomap_base: u16,

}

impl Tss {

	pub const fn new( privilege_stack_table: [ u64; 3], interrupt_stack_table: [ u64; 7], iomap_base: u16) -> Self {

		Tss {

			reserved0: 0,
			privilege_stack_table,
			reserved1: 0,
			interrupt_stack_table,
			reserved2: 0,
			reserved3: 0,
			iomap_base,
		}
	}

}

#[ repr( C, packed)]
pub struct Pointer {

	pub limit: u16,
	pub base: u64,

}

impl Pointer {

	pub fn new( table_addr: u64, limit: u16) -> Pointer {

		Pointer { limit, base: table_addr }
	}

	pub unsafe fn load_gdt( &self) {

		asm!( "lgdt [{}]", in( reg) self, options( nostack));

	}

	pub unsafe fn load_idt( &self) {

		asm!( "lidt [{}]", in( reg) self, options( nostack));

	}

}

#[ repr( C, packed)]
pub struct IntStackFrame {

	/// Code address to return (old %rip).
	pub instruction_pointer: u64,
	/// Code segment before interrupt.
	pub code_segment: u64,
	/// Flags register before interrupt.
	pub cpu_flags: u64,
	/// Stack pointer before interrupt.
	pub stack_pointer: u64,
	/// Stack segment before interrupt.
	pub stack_segment: u64,

}

pub unsafe fn enable_interrupts() {

	asm!( "sti");

}

pub unsafe fn disable_interrupts() {

	asm!( "cli");

}
