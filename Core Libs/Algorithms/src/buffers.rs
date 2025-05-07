use core::mem::MaybeUninit;

/// Buffers values on static memory.
#[ repr( C)]
pub struct StaticBuffer< T: Clone, const LENGTH: usize> {

	buffer: [ T; LENGTH],
	start: usize,
	end: usize,
	zero: bool,

}

impl< T: Clone + Copy, const LENGTH: usize> StaticBuffer< T, LENGTH> {

	pub const fn new() -> Self {

		Self {
			buffer: [ unsafe { MaybeUninit::zeroed().assume_init()}; LENGTH],
			start: 0,
			end: 0,
			zero: true,
		}
	}

	/// Pushes value on top of buffer.
	/// Outputs `Err(())` if overflowed.
	pub fn push( &mut self, v: T) -> Result< (), ()> {

		if self.start != self.end || self.zero {

			self.zero = false;
			self.buffer[ self.end] = v;
			self.end += 1;

			if self.end == LENGTH {

				self.end = 0;

			}

			Ok( ())
		} else {

			Err( ())
		}

	}

	/// Pops value from bottom of buffer.
	/// Outputs `None` if there are no more variables to pop.
	pub fn pop( &mut self) -> Option< T> {

		if self.zero {

			None
		} else {
			let result = self.buffer[ self.start].clone();

			unsafe {
				self.reject_non_zero();
			}

			Some( result)
		}
	}

	/// Rejects value from bottom of buffer.
	pub fn reject( &mut self) {

		if !self.zero {

			unsafe {
				self.reject_non_zero();
			}

		}

	}

	/// Rejects value from bottom of buffer assuming it isn't empty.
	pub unsafe fn reject_non_zero( &mut self) {

		self.start += 1;

		if self.start == LENGTH {

			self.start = 0;

		}

		if self.start == self.end {

			self.zero = true;

		}

	}

	/// Returns pointer to value from bottom of buffer without removing it.
	pub fn head_mut( &mut self) -> &mut T {

		&mut self.buffer[ self.start]
	}

	pub fn head( &self) -> &T {

		&self.buffer[ self.start]
	}

	/// Outputs current length of the buffer.
	pub fn len( &self) -> usize {

		if self.start > self.end {

			LENGTH - self.start + self.end
		} else {

			if !self.zero && self.end == self.start {

				LENGTH
			} else {

				self.end - self.start
			}
		}
	}

	pub fn is_full( &self) -> bool {

		self.start == self.end && !self.zero
	}

	pub fn is_empty( &self) -> bool {

		self.zero
	}

}
