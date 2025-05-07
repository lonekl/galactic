pub trait ColorDraw< C> {

	fn draw_over( &self, origin: &mut C);

}



#[ derive( Clone, Copy)]
pub struct Rgb8 {

	pub r: u8,
	pub g: u8,
	pub b: u8,

}

impl Rgb8 {

	pub const fn new( r: u8, g: u8, b: u8) -> Self {

		Self { r, g, b }
	}

}

impl ColorDraw< Rgb8> for Rgb8 {
	fn draw_over( &self, origin: &mut Rgb8) {

		*origin = *self;
	}
}

impl ColorDraw< Rgba8> for Rgb8 {
	fn draw_over( &self, origin: &mut Rgba8) {
		let over_draw = ( *self).into();

		*origin = over_draw;
	}
}



#[ derive( Clone, Copy)]
pub struct Rgba8 {

	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,

}

impl Rgba8 {

	pub const fn new( r: u8, g: u8, b: u8, a: u8) -> Self {

		Self { r, g, b, a }
	}

}

/* TODO
impl ColorDraw< Rgb8> for Rgba8 {
	fn draw_over( &self, origin: &mut Rgb8) {

	}
}
*/

impl From< Rgb8> for Rgba8 {
	fn from( value: Rgb8) -> Self {
		Self {
			r: value.r,
			g: value.g,
			b: value.b,
			a: 255,
		}
	}
}
