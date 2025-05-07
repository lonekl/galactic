use core::mem::transmute;
use x86_64::tables::IntStackFrame;

use super::PICS;

pub extern "x86-interrupt" fn breakpoint( _stack_frame: IntStackFrame) {

	unsafe {
		let str: &mut [ u8; 22] = transmute( 0xb8000_usize);
		*str = *b"b r e a k   p o i n t ";
	}

}

pub extern "x86-interrupt" fn page_fault( _stack_frame: IntStackFrame) -> ! {

	unsafe {
		let str: &mut [ u8; 20] = transmute( 0xb8000_usize);
		*str = *b"p a g e   f a u l t ";
	}

	loop {}
}

pub extern "x86-interrupt" fn timer( _stack_frame: IntStackFrame) {

	unsafe {
		let byte: &mut u8 = transmute( 0xb8000_usize);
		*byte += 1;
	}

	unsafe {
		PICS.end_of_interrupt( 0);
	}
}

pub extern "x86-interrupt" fn keyboard( _stack_frame: IntStackFrame) {


	unsafe {
		PICS.end_of_interrupt( 1);
	}
}
