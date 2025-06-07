use std::env;
use std::path::PathBuf;
use crate::AbortControl;

use crate::{ Abort, Continue};

pub struct Opt {

	pub abort_level: AbortLevel,
	pub home_directory: PathBuf,
	pub cargo_bin: PathBuf,
	pub tasks: Vec< Task>,

}

impl Opt {

	/// Here spaghetti begins.
	pub fn new() -> Result< Self, Vec< String>> {
		#[ derive( PartialEq)]
		enum ArgState {

			Task,
			Config { flag_part: bool /* false: name, true: value */ },

		}
		let mut argument_state = ArgState::Task;
		let mut tasks = Vec::new();
		let mut current_task = Task::EMPTY_GENERIC;
		let mut flag_name = String::new();
		let mut flag_value = String::new();
		let mut errors = Vec::new();
		let mut help_was_used = false;

		let home_directory = match env::var( "HOME") {
			Ok( home) => home.into(),
			Err( _error) => PathBuf::new(),
		};
		let mut cargo_bin: PathBuf = home_directory.clone();
		cargo_bin.push( ".cargo/bin/cargo");
		if ! cargo_bin.is_file() {
			cargo_bin = format!( "/bin/cargo").into();
		}

		let mut arch = Arch::current();
		let mut release = false;

		for mut arg in env::args().skip( 1) {
			arg.push( ' ');
			for arg_char in arg.chars() {

				match &mut argument_state {

					ArgState::Task => {
						let error_count_before = errors.len();
						let current_task_before = current_task.clone();

						match arg_char {
							'h' => { current_task = Task::Help; help_was_used = true; },
							'b' => { current_task = Task::Build { release: false, arch }; release = false; },
							'i' => current_task = Task::Image { size: 1024 * 1024 * 4, partition_table: PartitionTable::Mbr, arch, release },
							'r' => current_task = Task::RunQemu { memory: 1024 * 1024 * 64, arch },
							'g' => current_task = Task::EMPTY_GENERIC,
							'[' => argument_state = ArgState::Config { flag_part: false },
							' ' => {},
							_ => errors.push( format!( "There is no such task as \'{arg_char}\'.")),
						}

						if error_count_before == errors.len() && argument_state == ArgState::Task {

							tasks.push( current_task_before);

						}

					},
					ArgState::Config { flag_part } => match arg_char {
						'=' => *flag_part = true,
						',' | ']' => {

							if !( flag_name.is_empty() && flag_value.is_empty()) {

								if flag_name.is_empty() {

									errors.push( format!( "Flag or value is empty."));

								}

								// Here shitty checking begins.
								match &mut current_task {
									Task::Help => errors.push( String::from( "Help task does not accept any flags.")),
									Task::Build { release: build_release, arch: build_arch } => match flag_name.as_str() {
										"release" => if *flag_part {
											errors.push( format!( "Build task: release flag does not accept a value."));
										} else {
											*build_release = true;
											release = true;
										},

										"arch" | "architecture" => if *flag_part {
											match Arch::from_str( &flag_value) {
												Some( v) => {
													*build_arch = v;
													arch = v;
												},
												None => errors.push( format!( "Build Task: \"{flag_value}\" architecture is not supported.")),
											}
										} else {
											errors.push( format!( "Build task: \"{flag_name}\" should contain a value."));
										},

										_ => errors.push( format!( "Build task: \"{flag_name}\" is not an accepted flag.")),
									},

									Task::Image { size: _image_size, partition_table: _image_partition_table, arch: _image_arch, release: _image_release } => {},
									Task::RunQemu { memory: _run_memory, arch: _run_arch } => {},
									Task::Generic { abort_level } => match flag_name.as_str() {
										"abort" => if *flag_part {
											match AbortLevel::from_flag_value( &flag_value) {
												Some( v) => {
													*abort_level = Some( v);
												},
												None => errors.push( format!( "Generic task: \"{flag_value}\" abort hint is invalid.")),
											}
										} else {
											errors.push( format!( "Generic task: \"abort\" flag should contain a value."));
										}
										_ => errors.push( format!( "Generic task: \"{flag_name}\" is not an accepted flag.")),
									},
								}

							}

							*flag_part = false;
							flag_name = String::new();
							flag_value = String::new();

							if arg_char == ']' {

								argument_state = ArgState::Task;
								tasks.push( current_task);
								current_task = Task::EMPTY_GENERIC;

							}

						},
						_ => if *flag_part {
							flag_value.push( arg_char);
						} else {
							flag_name.push( arg_char);
						},
					},

				}

			}

		}

		if help_was_used && tasks.len() != 2 {

			errors.push( format!( "Help task can only be used once and alone."));

		}

		match argument_state {
			ArgState::Config { .. } => errors.push( format!( "Brackets were not closed")),
			_ => {},
		}

		if errors.len()	!= 0 {
			Err( errors)
		} else {

			Ok( Self {
				abort_level: AbortLevel::FailAll,
				home_directory,
				cargo_bin,
				tasks,
			})
		}
	}

}

#[ derive( Clone)]
pub enum Task {

	/// Makes parsing of arguments easier.
	Generic {
		abort_level: Option< AbortLevel>
	},
	Help,
	Build {
		release: bool,
		arch: Arch,
	},
	Image {
		size: u64,
		partition_table: PartitionTable,
		arch: Arch,
		release: bool,
	},
	RunQemu {
		memory: usize,
		arch: Arch,
	},

}

impl Task {

	const EMPTY_GENERIC: Self = Self::Generic { abort_level: None };

}



#[ derive( Clone, Copy)]
pub enum Arch {

	X86_64,

}

use Arch::*;

impl Arch {

	#[ cfg( target_arch = "x86_64")]
	pub fn current() -> Self {

		X86_64
	}

	#[ cfg( not( any( target_arch = "x86_64")))]
	pub fn current() -> Self {

		X86_64
	}

	pub fn from_str( name: &str) -> Option< Self> {

		match name {
			"X86_64" => Some( X86_64),
			_ => None,
		}
	}

	pub fn as_str( self) -> &'static str {

		match self {
			X86_64 => "X86_64",
		}
	}

	pub fn rust_target( self) -> &'static str {

		match self {
			X86_64 => "x86_64-unknown-none",
		}
	}

	pub fn core_utils_target( self) -> &'static str {

		match self {
			X86_64 => "x86_64-linux-gnu",
		}
	}

	pub fn qemu_command( self) -> &'static str {

		match self {
			X86_64 => "qemu-system-x86_64",
		}
	}

	pub fn core_target( self, release: bool) -> String {

		if release {
			format!( "target/{}/core-release/", self.rust_target())
		} else {
			format!( "target/{}/core-dev/", self.rust_target())
		}
	}

}



#[ derive( Clone, Copy)]
pub enum AbortLevel {

	/// Error aborts everything.
	FailAll,
	/// Error aborts current task, continuing others.
	FailTask,
	/// Finishes off the task after an error, aborting afterwards.
	FinishTask,
	/// Does never abort.
	None,

}

impl AbortLevel {

	pub fn from_flag_value( value: &str) -> Option< Self> {

		match value {
			"all"   => Some( Self::FailAll),
			"task"  => Some( Self::FailTask),
			"rest"  => Some( Self::FinishTask),
			"never" => Some( Self::None),
			_       => None,
		}
	}

	pub fn abort_control( &self, continue_afterwards: &mut bool) -> AbortControl {

		match self {
			AbortLevel::None => Continue,
			AbortLevel::FinishTask => {

				*continue_afterwards = false;
				Continue
			},
			AbortLevel::FailTask => Abort,
			AbortLevel::FailAll => {

				*continue_afterwards = false;
				Abort
			},
		}
	}

}

#[ derive( Clone, Copy)]
pub enum PartitionTable {

	Mbr,

}
