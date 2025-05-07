use std::mem::transmute;

#[derive(Clone, Copy)]
pub struct DataSize (u64);

impl DataSize {

	pub fn full_512_sectors(self) -> bool {

		self.0 % 512 == 0
	}

	pub fn parse(str: &str) -> Result<Self, &'static str> {
		use NumberFormat::*;
		enum NumberFormat {
			Decimal,
			Hexadecimal,
			Octal,
			Binary,
		}
		let mut number_format = Decimal;
		let bytes = str.as_bytes();

		let number_start = if bytes.len() < 2 {
			0
		} else if &bytes[0..2] == b"0x" || &bytes[0..2] == b"0h" {
			number_format = Hexadecimal;
			2
		} else if &bytes[0..2] == b"0b" {
			number_format = Binary;
			2
		} else if &bytes[0..2] == b"0o" {
			number_format = Octal;
			2
		} else if &bytes[0..2] == b"0d" {
			2
		} else {
			0
		};

		let mut index_counter = number_start;

		while index_counter < bytes.len() {

			match bytes[index_counter] {
				b'0'..b'9' => {},
				_ => {index_counter += 1; break},
			}

			index_counter += 1;
		}

		let unit = &bytes[index_counter..];
		let number_bytes = &bytes[number_start..index_counter];
		let number_str: &str = unsafe {transmute(number_bytes)};

		let mut number: u64 = match number_format {
			Decimal => match number_str.parse() {
				Ok(num) => num,
				Err(_) => return Err("wrong number format"),
			},
			_ => 0,
		};

		number *= match unit {
			b"b" | b"B" => 1,
			b"kb" | b"Kb" => 1000 / 8,
			b"kB" | b"KB" => 1000,
			b"kib" | b"Kib" | b"kIb" | b"KIb" => 1024 / 8,
			b"kiB" | b"KiB" | b"kIB" | b"KIB" => 1024,

			b"mb" | b"Mb" => 1000_000 / 8,
			b"mB" | b"MB" => 1000_000,
			b"mib" | b"Mib" | b"mIb" | b"MIb" => 1024 * 1024 / 8,
			b"miB" | b"MiB" | b"mIB" | b"MIB" => 1024 * 1024,

			b"gb" | b"Gb" => 1000_000_000 / 8,
			b"gB" | b"GB" => 1000_000_000,
			b"gib" | b"Gib" | b"gIb" | b"GIb" => 1024 * 1024 * 1024 / 8,
			b"giB" | b"GiB" | b"gIB" | b"GIB" => 1024 * 1024 * 1024,

			b"tb" | b"Tb" => 1000_000_000_000 / 8,
			b"tB" | b"TB" => 1000_000_000_000,
			b"tib" | b"Tib" | b"tIb" | b"TIb" => 1024 * 1024 * 1024 * 1024 / 8,
			b"tiB" | b"TiB" | b"tIB" | b"TIB" => 1024 * 1024 * 1024 * 1024,

			b"" => return Err("no unit"),
			_ => return Err("wrong unit"),
		};

		Ok(Self(number))
	}

}
