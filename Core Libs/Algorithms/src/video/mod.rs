use crate::video::color::{ ColorDraw, Rgb8};
use crate::video::mode::Resolutions;

pub mod color;
pub mod mode;



pub trait Display {

	fn draw( &mut self, a: ImageBufferPainter< Rgb8>);
	fn available_resolutions( &self) -> Resolutions;

}



pub struct ConstSizeImageBuffer<
	'a,
	Pixel,
	const WIDTH: usize,
	const HEIGHT: usize,
> {

	#[ allow( dead_code)]
	image: [ [ Pixel; WIDTH]; HEIGHT],
	image_painter: ImageBufferPainter< 'a, Pixel>,

}

impl<
	'a,
	Pixel: Copy,
	const WIDTH: usize,
	const HEIGHT: usize,
> ConstSizeImageBuffer< 'a, Pixel, WIDTH, HEIGHT> {

	pub fn new( filler: Pixel) -> Self {
		let mut image = [ [ filler; WIDTH]; HEIGHT];

		Self {
			image,
			image_painter: ImageBufferPainter::new( unsafe {
					core::slice::from_raw_parts_mut( image.as_mut_ptr() as * mut Pixel, WIDTH * HEIGHT)
				},
				WIDTH,
			),
		}
	}


	pub fn image_painter( &'a mut self) -> &'a ImageBufferPainter< 'a, Pixel> {

		&self.image_painter
	}

}



pub struct ImageBufferPainter< 'a, Pixel> {

	width: usize,
	height: usize,
	pub pixel_array: &'a mut [ Pixel],

}

impl< 'a, Pixel> ImageBufferPainter< 'a, Pixel> {

	pub fn new( buffer: &'a mut [ Pixel], width: usize) -> Self {

		if buffer.len() % width != 0 {
			panic!( "Buffer has wrong length.")
		}

		Self {
			width,
			height: buffer.len() / width,
			pixel_array: buffer,
		}
	}


	pub fn clear< Filler: ColorDraw< Pixel>>( &mut self, filler: Filler) {

		for pixel in self.pixel_array.into_iter() {

			filler.draw_over( pixel);

		}

	}

	pub fn draw_pixel_index< Filler: ColorDraw< Pixel>>( &mut self, filler: Filler, index: usize) {

		filler.draw_over( &mut self.pixel_array[ index]);

	}

	pub fn draw_pixel< Filler: ColorDraw< Pixel>>( &mut self, filler: Filler, x: usize, y: usize) {

		filler.draw_over( &mut self.pixel_array[ x + y * self.width]);

	}


	pub fn width( &self) -> usize {

		self.width
	}

	pub fn height( &self) -> usize {

		self.height
	}

}
