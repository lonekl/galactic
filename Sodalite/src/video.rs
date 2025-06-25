use core::convert::Into;
use algorithms::video::{ Display, ImageBufferPainter, color::Rgb8};
use vga::{ VgaDisplay};
use x86_64::SerialPortWait;

pub static mut VGA: VgaDisplay< SerialPortWait> = VgaDisplay::new( SerialPortWait::new());

//const DEFAULT_PALETTE: &[VgaColor; 16] = ;

pub unsafe fn initialize() {

	VGA.device.set_color_palette( 0, &[
		Rgb8::new(  66,  56, 180).into(), // 0000 Blue – background.
		Rgb8::new(   0,   0,   0).into(), // 0001 Total black.
		Rgb8::new(   9, 160,  89).into(),	// 0010 Dark green.
		Rgb8::new(   8,   8,  64).into(), // 0011 Darker blue.
		Rgb8::new( 140,  24,   8).into(),	// 0100 Dark red – fatal error.
		Rgb8::new(  84,  10,  93).into(), // 0101 Dark violet.
		Rgb8::new( 184, 160,  86).into(), // 0110 Brown. (Or what it is meant to be.)
		Rgb8::new(  85, 220, 242).into(), // 0111 Cyan – foreground.
		Rgb8::new(  29,  17, 158).into(), // 1000 Dark blue – highlight background.
		Rgb8::new( 164, 164, 164).into(), // 1001 Grey.
		Rgb8::new(   0, 195, 125).into(), // 1010 Light green – number.
		Rgb8::new( 255, 255, 255).into(), // 1011 Light of absolute.
		Rgb8::new( 240, 105, 150).into(), // 1100 Red – error.
		Rgb8::new( 183,  44, 255).into(), // 1101 Light violet – correct options.
		Rgb8::new( 200, 180,  80).into(), // 1110 Yellow – „quote string”.
		Rgb8::new( 191, 233, 253).into(), // 1111 Light cyan – highlight foreground.
	]);

	let mut screen_buffer = algorithms::video::ConstSizeImageBuffer::< Rgb8, 640, 400>::new( Rgb8::new( 9, 160, 89));
	let screen_painter: &mut ImageBufferPainter< Rgb8> = screen_buffer.image_painter();
	screen_painter.draw_pixel( Rgb8::new( 6, 150, 106), 10, 10);
	VGA.draw( screen_painter);

}
