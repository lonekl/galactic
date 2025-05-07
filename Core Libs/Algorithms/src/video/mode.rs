pub struct Resolution {

	pub width: u32,
	pub height: u32,

}

impl Resolution {

	pub const fn new( width: u32, height: u32) -> Self {

		Self {
			width,
			height,
		}
	}

}



pub type Resolutions<'a> = &'a [ Resolution];
