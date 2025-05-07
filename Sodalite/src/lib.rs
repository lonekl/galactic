#![ no_std]
#![ feature( abi_x86_interrupt, const_trait_impl)]
#![ allow( static_mut_refs)]

#[ cfg( target_arch = "x86_64")]
pub mod x86_64;
#[ cfg( target_arch = "x86_64")]
pub use crate::x86_64 as arch;

#[ cfg( not( any( target_arch = "x86_64")))]
compile_error!( "Unsupported architecture, only x86_64 is supported.");

pub mod video;

use core::panic::PanicInfo;

#[ no_mangle]
pub extern "C" fn rust_start() -> ! {

	unsafe {
		arch::initialize();
		video::initialize();
	}

	//unsafe { (0xffffffffffusize as *mut u8).write(1);}

	unsafe { arch::end_exec()}
}

#[ panic_handler]
pub fn panic( _panic_info: &PanicInfo) -> ! {

	unsafe { arch::end_exec()}
}
