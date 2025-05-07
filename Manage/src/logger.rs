pub struct Logger {

	color: bool,
	tab_size: u8,
	current_tab: u8,

}

impl Logger {

	pub fn new() -> Self {

		Self {
			color: true,
			tab_size: 4,
			current_tab: 0,
		}
	}

	pub fn tab_up(&mut self) {

		self.current_tab += 1;

	}

	pub fn tab_down(&mut self) {

		self.current_tab -= 1;

	}

	pub fn info(&self, message: &str) {
		self.write_message(MessageType::Info, message);
	}

	pub fn warn(&self, message: &str) {
		self.write_message(MessageType::Warn, message);
	}

	pub fn err(&self, message: &str) {
		self.write_message(MessageType::Err, message);
	}

	fn write_message(&self, message_type: MessageType, message: &str) {

		for _ in 0..self.current_tab {
			for _ in 0..self.tab_size {
				print!(" ");
			}
		}

		print!(
			"{message_type}: {message}",
			message_type = message_type.as_str(self.color),
		);

		if message_type != MessageType::UserInput {
			println!();
		}

	}

}

#[derive(PartialEq)]
enum MessageType {

	Info,
	Warn,
	Err,
	UserInput,

}

impl MessageType {

	fn as_str(&self, color: bool) -> &'static str {
		use MessageType::*;
		match color {
			true => match self {
				Info => "[0;1;34minfo[0m",
				Warn => "[0;1;32mwarning[0m",
				Err => "[0;1;31merror[0m",
				UserInput => "[0;1;33muser input[0m",
			},
			false => match self {
				Info => "info",
				Warn => "warn",
				Err => "err",
				UserInput => "user input",
			},
		}
	}

}
