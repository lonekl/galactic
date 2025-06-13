//! Video Graphics Adapter driver.
//! This library should be used only with `0xa0000 - 0xbffff` pointing to Vga buffers and on `ring 0`.
#![ no_std]
#![ feature( const_trait_impl)]

use algorithms::sync::DriverDelay;
use algorithms::video::color::{ ColorDraw, Rgb8};
use algorithms::video::{ Display, ImageBufferPainter};
use algorithms::video::mode::{ Resolution, Resolutions};

pub mod screen;
pub mod register_access;



/// General Vga display.
pub struct VgaDisplay< Delay: DriverDelay> {

	#[ allow( dead_code)]
	pub device: VgaDevice< Delay>,

}

impl< Delay: DriverDelay> VgaDisplay< Delay> {

	pub const fn new( wait: Delay) -> Self {

		Self {
			device: VgaDevice::< Delay>::new( wait),
		}
	}

}

impl< Delay: DriverDelay> Display for VgaDisplay< Delay> {
	fn draw( &mut self, _a: &ImageBufferPainter< Rgb8>) {


	}

	fn available_resolutions( &self) -> Resolutions {

		const RESOLUTIONS: Resolutions = &[
			Resolution::new( 640, 480),
			Resolution::new( 320, 240),
		];
		RESOLUTIONS
	}
}



/// Bindings to Vga.
pub struct VgaDevice< Delay: DriverDelay> {

	wait: Delay,

	#[ allow( dead_code)]
	mode: VgaMode,

}

impl< Wait: DriverDelay> VgaDevice< Wait> {

	pub const fn new( mut wait: Wait) -> Self {

		wait.increment_delay();

		Self {
			wait,
			mode: VgaMode::Unknown,
		}
	}

	pub unsafe fn set_color_palette( &mut self, start_index: u8, palette: &[ VgaPaletteColor]) {
		let raw_palette = unsafe { core::mem::transmute( palette) }; // By transmute is only way I see it would go, fastly enough at least.

		register_access::write_dac( start_index, raw_palette, &mut self.wait);

	}

}



#[ derive( Clone, Copy)]
pub enum VgaMode {

	Graphic640x480x4,
	Graphic320x240x8,
	Unknown,

}

impl VgaMode {

	pub unsafe fn set_mode< Delay: DriverDelay>( self, delay: &mut Delay) {

		register_access::write_graphics_indexed( 0x6, 0b0001, delay);

		match self {
			Self::Graphic640x480x4 => {},
			Self::Graphic320x240x8 => {},
			Self::Unknown => panic!( "Vga mode should be defined."),
		}

	}

}



#[ derive( Clone, Copy)]
#[ repr( C, packed)]
pub struct VgaPaletteColor {

	r: u8,
	g: u8,
	b: u8,

}

impl VgaPaletteColor {

	pub const fn new( r: u8, g: u8, b: u8) -> Option< Self> {

		if
			r & 0b11000000 != 0
			||
			g & 0b11000000 != 0
			||
			b & 0b11000000 != 0
		{
			return None;
		}

			Some( Self { r, g, b })
	}

}

impl Into< [u8; 3]> for VgaPaletteColor {
	fn into( self) -> [ u8; 3] {

		[
			self.r,
			self.g,
			self.b,
		]
	}
}

impl Into< VgaPaletteColor> for Rgb8 {
	fn into( self) -> VgaPaletteColor {

		VgaPaletteColor {
			r: self.r >> 2,
			g: self.g >> 2,
			b: self.b >> 2,
		}
	}
}

impl Into< Rgb8> for VgaPaletteColor {
	fn into( self) -> Rgb8 {

		Rgb8::new(
			self.r << 2,
			self.g << 2,
			self.b << 2,
		)
	}
}




#[ derive( Clone, Copy)]
pub struct Vga4Color ( u8);

impl Vga4Color {

	pub fn new( value: u8) -> Self {

		if value & 0b11110000 != 0 {
			panic!( "Vga4Color value is not 4-bit compatible.");
		}

		Self ( value)
	}

}

impl ColorDraw< Vga4Color> for Vga4Color {

	fn draw_over( &self, origin: &mut Vga4Color) {

		*origin = *self;

	}

}

impl ColorDraw< Vga8Color> for Vga4Color {

	fn draw_over( &self, origin: &mut Vga8Color) {

		origin.0 = self.0;

	}

}



#[ derive( Clone, Copy)]
pub struct Vga8Color ( u8);

impl Vga8Color {

	pub fn new( value: u8) -> Self {

		Self ( value)
	}

}

impl ColorDraw< Vga8Color> for Vga8Color {

	fn draw_over( &self, origin: &mut Vga8Color) {

		*origin = *self;

	}

}
