pub mod buffer;

#[ derive( Clone, Copy)]
#[ repr( C, packed)]
pub struct Char16Color {

	char: Char,
	color: Color16x16,

}

impl Char16Color {

	pub fn filler() -> Self {

		Self {
			char: Char::new(b' '),
			color: Color16x16::new(7, 0),
		}
	}

	pub fn new(char: Char, color: Color16x16) -> Self {

		Self {
			char,
			color,
		}
	}

}


#[ derive( Clone, Copy)]
#[ repr( C, packed)]
pub struct Char( u8);

impl Char {

	pub const fn new( byte: u8) -> Self {

		Self( byte)
	}

}


#[ derive( Clone, Copy)]
#[ repr( C, packed)]
pub struct Color16x16 ( u8);

impl Color16x16 {

	pub const fn new_raw( colors: u8) -> Self {

		Self( colors)
	}

	pub const fn new( primary: u8, secondary: u8) -> Self {

		Self( ( primary & 0b1111) | ( secondary << 4))
	}

}
