use core::convert::Into;
use algorithms::video::color::Rgb8;
use vga::{ VgaDevice};
use x86_64::SerialPortWait;

pub static mut VGA: VgaDevice< SerialPortWait> = VgaDevice::new( SerialPortWait::new());

//const DEFAULT_PALETTE: &[VgaColor; 16] = ;

pub unsafe fn initialize() {

	VGA.set_color_palette( 0, &[
		Rgb8::new(  66,  56, 180).into(), // 0000 Blue, background.
		Rgb8::new(   0,   0,   0).into(), // 0001 Black nigger.
		Rgb8::new(   9, 160,  89).into(),	// 0010 Dark green.
		Rgb8::new(   8,   8,  64).into(), // 0011 Darker blue.
		Rgb8::new( 188,  21,   6).into(),	// 0100 Dark red, fatal error.
		Rgb8::new( 105,  10, 154).into(), // 0101 Violet.
		Rgb8::new( 211, 191,  86).into(), // 0110 Brown TODO fix it.
		Rgb8::new(  85, 220, 242).into(), // 0111 Cyan, foreground.
		Rgb8::new(  29,  17, 158).into(), // 1000 Dark blue, highlight background.
		Rgb8::new( 200, 200, 200).into(), // 1001 Grey.
		Rgb8::new(   6, 150, 106).into(), // 1010 Light green, number.
		Rgb8::new( 255, 255, 255).into(), // 1011 White man.
		Rgb8::new( 250,  93,  79).into(), // 1100 Red, error.
		Rgb8::new( 183,  44, 255).into(), // 1101 Pink, correct options.
		Rgb8::new( 255, 235, 129).into(), // 1110 Yellow, quote string.
		Rgb8::new( 191, 233, 253).into(), // 1111 Light cyan, highlight foreground.
	]);

}
