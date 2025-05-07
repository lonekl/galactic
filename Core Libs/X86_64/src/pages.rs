pub type Page = [ PagePointer; 512];

pub struct PagePointer ( u64);

impl PagePointer {

	const PRESENT: u64 =        1 << 0;
	const WRITABLE: u64 =       1 << 1;
	const USER_ACCESS: u64 =    1 << 2;
	const WRITE_THROUGH: u64 =  1 << 3;
	const CACHE_DISABLED: u64 = 1 << 4;
	const ACCESSED: u64 =       1 << 5;
	const SIZE: u64 =           1 << 7;

	pub const fn empty() -> Self {

		Self( 0)
	}

	pub const fn new( address: usize) -> Self {

		if Self::check_for_flag_bits( address) {

			panic!( "First 12 bits of address weren't zeroed.")
		}

		unsafe { Self::new_unchecked( address)}
	}

	pub const fn new_option( address: usize) -> Option< Self> {

		if Self::check_for_flag_bits( address) {

			return None;
		}

		Some( unsafe { Self::new_unchecked( address)})
	}

	// Checks if first 12 bits of address aren't zeroed.
	// Returns true if they aren't.
	const fn check_for_flag_bits( address: usize) -> bool {

		address & 0b1111_1111_1111 != 0
	}

	pub const unsafe fn new_unchecked( address: usize) -> Self {

		Self ( address as u64)
	}

	pub const fn set_present( mut self) -> Self {

		self.0 |= Self::PRESENT;
		self
	}

	pub const fn set_writable( mut self) -> Self {

		self.0 |= Self::WRITABLE;
		self
	}

	pub const fn set_user_access( mut self) -> Self {

		self.0 |= Self::USER_ACCESS;
		self
	}

	pub const fn set_write_through( mut self) -> Self {

		self.0 |= Self::WRITE_THROUGH;
		self
	}

	pub const fn set_cache_disabled( mut self) -> Self {

		self.0 |= Self::CACHE_DISABLED;
		self
	}

	pub const fn set_accessed( mut self) -> Self {

		self.0 |= Self::ACCESSED;
		self
	}

	pub const fn set_size( mut self) -> Self {

		self.0 |= Self::SIZE;
		self
	}

}
