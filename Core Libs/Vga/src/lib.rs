//! Video Graphics Adapter driver.
//! This library should be used only with `0xa0000 - 0xbffff` pointing to Vga buffers and on `ring 0`.
#![ no_std]
#![ feature( const_trait_impl)]

use algorithms::sync::DriverDelay;
use algorithms::video::color::Rgb8;
use algorithms::video::{ Display, ImageBufferPainter};
use algorithms::video::mode::{ Resolution, Resolutions};

pub mod screen;
pub mod register_access;



/// General Vga display.
pub struct VgaDisplay< Delay: DriverDelay> {

	#[ allow( dead_code)]
	device: VgaDevice< Delay>,

}

impl< Delay: DriverDelay> VgaDisplay< Delay> {

}

impl< Delay: DriverDelay> Display for VgaDisplay< Delay> {
	fn draw( &mut self, _a: ImageBufferPainter< Rgb8>) {


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

}

impl< Wait: DriverDelay> VgaDevice< Wait> {

	pub const fn new( wait: Wait) -> Self {

		Self {
			wait,
		}
	}

	pub unsafe fn set_color_palette( &mut self, start_index: u8, palette: &[ VgaColor]) {
		let raw_palette = unsafe { core::mem::transmute( palette)}; // By transmute is only way I see it would go, fastly enough at least.

		register_access::write_dac( start_index, raw_palette, &mut self.wait);

	}

}



#[ derive( Clone, Copy)]
pub enum VgaMode {

	Graphic640x480x4,
	Graphic320x240x8,
	Unknown,

}



#[ derive( Clone, Copy)]
#[ repr( C, packed)]
pub struct VgaColor {

	r: u8,
	g: u8,
	b: u8,

}

impl VgaColor {

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

impl Into< [u8; 3]> for VgaColor {
	fn into( self) -> [ u8; 3] {

		[
			self.r,
			self.g,
			self.b,
		]
	}
}

impl Into< VgaColor> for Rgb8 {
	fn into( self) -> VgaColor {

		VgaColor {
			r: self.r >> 2,
			g: self.g >> 2,
			b: self.b >> 2,
		}
	}
}

impl Into< Rgb8> for VgaColor {
	fn into( self) -> Rgb8 {

		Rgb8::new(
			self.r << 2,
			self.g << 2,
			self.b << 2,
		)
	}
}
