#![ no_std]
#![ feature( const_trait_impl, abi_x86_interrupt)]

use core::arch::asm;

use algorithms::sync::DriverDelay;

pub mod tables;
pub mod pages;

#[ derive( Clone, Copy)]
pub struct Segment( u16);

impl Segment {

	pub const fn new( index: u16, desc: u64) -> Self {

		Self( index << 3 | ( desc >> tables::gdf::RING_POS & 0b11) as u16)
	}

	pub unsafe fn set_code( self) {

		asm!(
		"push {segment:x}",
		"lea {tmp}, [1f + rip]",
		"push {tmp}",
		"retfq",
		"2:",
		segment = in( reg) < Segment as Into< u16>>::into( self), tmp = lateout( reg) _,
		);

	}

	/// You must ensure that it actually points to valid tss segment.
	pub unsafe fn load_tss( self) {

		asm!( "ltr {0:x}", in( reg) self.0, options( nostack, nomem));

	}

}

impl Into<u16> for Segment {
	fn into( self) -> u16 {

		self.0
	}
}


pub struct Pic< const COMMAND: u16, const DATA: u16> {

	irq_offset: u8,

}

impl< const COMMAND: u16, const DATA: u16> Pic< {COMMAND}, {DATA}> {

	unsafe fn send_command( &self, command: u8) {

		outb( COMMAND, command);

	}

	unsafe fn send_data( &self, data: u8) {

		outb( DATA, data);

	}

	unsafe fn get_data( &self) -> u8 {

		inb( DATA)
	}

}

pub struct ChainedPics {

	pub master: Pic< 0x20, 0x21>,
	pub slave: Pic< 0xA0, 0xA1>,

}

impl ChainedPics {

	const INITIALIZE: u8 = 0x11;
	const END_OF_INTERRUPT: u8 = 0x20;

	const MODE_8086: u8 = 1;

	pub const fn new( irq_offset_master: u8, irq_offset_slave: u8) -> Self {

		ChainedPics {
			master: Pic {
				irq_offset: irq_offset_master,
			},
			slave: Pic {
				irq_offset: irq_offset_slave,
			},
		}
	}

	/// Irq value must be under 16 or it will output wrong value.
	pub const fn irq_index( &self, irq: u8) -> u8 {

		let index = if irq >= 8 {
			self.master.irq_offset + irq
		} else {
			self.slave.irq_offset + irq - 8
		};

		index
	}

	pub unsafe fn reinitialize( &self) {

		// Save default masks to reset them later.
		let master_mask = self.master.get_data();
		let slave_mask = self.slave.get_data();

		// We initialize pics so it will wait for 3 byte initialization code.
		self.master.send_command( Self::INITIALIZE);
		simple_wait();
		self.slave.send_command( Self::INITIALIZE);
		simple_wait();

		// Set irq offsets.
		self.master.send_data( self.master.irq_offset);
		simple_wait();
		self.slave.send_data( self.slave.irq_offset);
		simple_wait();

		// Set chaining between pics.
		self.master.send_data( 4);
		simple_wait();
		self.slave.send_data( 2);
		simple_wait();

		// Set pics mode.
		self.master.send_data( Self::MODE_8086);
		simple_wait();
		self.slave.send_data( Self::MODE_8086);
		simple_wait();

		// Reset default masks.
		self.master.send_data( master_mask);
		self.slave.send_data( slave_mask);

	}

	pub unsafe fn end_of_interrupt( &self, irq: u8) {

		if irq >= 8 {
			self.slave.send_command( Self::END_OF_INTERRUPT);
		}

		self.master.send_command( Self::END_OF_INTERRUPT);

	}

}

pub struct SerialPortWait {

	wait_times: u16,

}

impl SerialPortWait {

	pub const fn new() -> Self {

		Self {
			wait_times: 0,
		}
	}

}

impl DriverDelay for SerialPortWait {

	fn wait( &mut self) {

		for _ in 0..self.wait_times {

			unsafe { simple_wait();}

		}

	}

	fn increment_delay( &mut self) {
		self.wait_times += 1;
	}

}

pub unsafe fn run_interrupt_free< T: FnOnce() -> R, R>( code: T) -> R {

	disable_interrupts();
	let returned = code();
	enable_interrupts();

	returned
}

pub unsafe fn make_breakpoint() {
	asm!( "int 3h");
}

pub unsafe fn enable_interrupts() {
	asm!( "sti");
}

pub unsafe fn disable_interrupts() {
	asm!( "cli");
}

pub unsafe fn halt() {
	asm!( "hlt");
}

#[ no_mangle]
pub unsafe fn end_exec() -> ! {
	asm!( "hlt", "jmp end_exec", options( noreturn))
}

pub unsafe fn outb( port: u16, data: u8) {
	asm!( "out dx, al", in( "dx") port, in( "al") data);
}

pub unsafe fn outw( port: u16, data: u16) {
	asm!( "out dx, ax", in( "dx") port, in( "ax") data);
}

pub unsafe fn outd( port: u16, data: u32) {
	asm!( "out dx, eax", in( "dx") port, in( "eax") data);
}

pub unsafe fn inb( port: u16) -> u8 {
	let output: u8;

	asm!( "in al, dx", in( "dx") port, out( "al") output);

	output
}

pub unsafe fn inw( port: u16) -> u16 {
	let output: u16;

	asm!( "in ax, dx", in( "dx") port, out( "ax") output);

	output
}

pub unsafe fn ind( port: u16) -> u32 {
	let output: u32;

	asm!( "in eax, dx", in( "dx") port, out( "eax") output);

	output
}

/// Function to create delay without multi tasking setup or anything complicated.
/// Used for simple micro second delay.
pub unsafe fn simple_wait() {

	outb( 0x80, 0);

}
