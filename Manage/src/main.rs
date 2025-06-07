#![ feature( try_trait_v2, try_blocks)]

pub mod opt;
pub mod logger;
pub mod image;
//pub mod units;

pub use opt::{ Opt, Task, Arch, AbortLevel, PartitionTable as SelectedPartitionTable};
pub use logger::Logger;

use std::mem::transmute;
use std::ops::{ ControlFlow, FromResidual, Try};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::process::{
	exit,
	Command,
};

use Arch::X86_64;
use image::{ MBR, PartitionTable};

fn main() {
	let mut logger = Logger::new();
	let mut opt = match Opt::new() {
		Ok( opt) => opt,
		Err( errors) => {
			logger.err( "Argument syntax error:");

			logger.tab_up();
			for error in errors {
				logger.err( &error);
			}

			exit( 1)
		},
	};

	let tasks = opt.tasks;
	opt.tasks = Vec::new();

	for task in tasks {
		let mut cont = true;

		match task {
			Task::Help => print!( "{}", include_str!( "help message.txt")),
			Task::Build { release, arch } => if Abort == try {


				//let arch_target = format!( "target/{}/", arch.rust_target_name());
				let core_target = arch.core_target( release);

				let core_profile = if release {
					"core-release"
				} else {
					"core-dev"
				};

				logger.info( "Building workspace.");
				logger.tab_up();
				logger.info( "Building Sodalite.");
				logger.tab_up();
				run_command(
						opt.cargo_bin.clone(),
						[ "build", "--target", arch.rust_target(), "--profile", core_profile, "--package", "sodalite"].iter(),
						&logger,
						&opt,
						&mut cont,
						"Rust is not installed.",
						"Main part compilation failed,",
					);

					match arch {
						X86_64 => {
							let core_utils_target_name = arch.core_utils_target();
							run_command(
								"nasm",
								[ "-f", "bin", "-o", &format!( "{core_target}bootsector.bin"), "Sodalite/X86_64/bootsector.asm"].iter(),
								&logger,
								&opt,
								&mut cont,
								"Nasm is not installed.",
								"Bootsector compilation failed,",
							)?;
							run_command(
								"nasm",
								[ "-f", "elf64", "-o", &format!( "{core_target}sodalite entry.o"), "Sodalite/X86_64/rust-entry.asm"].iter(),
								&logger,
								&opt,
								&mut cont,
								"Nasm is not installed.",
								"Entrypoint compilation failed,",
							)?;
							run_command(
								&format!( "{core_utils_target_name}-ld"),
								[ "--gc-sections", "-Ttext=0x7e00", "-o", &format!( "{core_target}sodalite.elf"), "-T", "Sodalite/X86_64/link.ld", &format!( "{core_target}sodalite entry.o"), &format!( "{core_target}libsodalite.a")].iter(),
								&logger,
								&opt,
								&mut cont,
								&format!( "{core_utils_target_name} core-utils toolchain is not installed."),
								"Failed to link Sodalite,",
							)?;
							run_command(
								&format!( "{core_utils_target_name}-objcopy"),
								[ "-O", "binary", &format!( "{core_target}sodalite.elf"), &format!( "{core_target}sodalite.bin")].iter(),
								&logger,
								&opt,
								&mut cont,
								&format!( "{core_utils_target_name} core-utils toolchain is not installed."),
								"Failed to create flat binary of Sodalite,",
							)?;
						},
					}

					logger.tab_down();
					logger.tab_down();


				} {
					exit( 1)
				},
				Task::Image { size, partition_table, arch, release } => if Abort == try {


					let core_target = arch.core_target( release);

					logger.info( "Creating disk image.");
					logger.tab_up();

					let bootsector = match read_file( &logger, &opt, &mut cont, format!( "{core_target}bootsector.bin").into(), "Sodalite bootsector") {
						Ok( bytes) => bytes,
						Err( abort_control) => {
							abort_control?;

							vec![ 0; 512]
						},
					};
					
					if bootsector.len() != 512 {

						logger.err( "Sodalite bootsector doesn't match 512 byte length.");
						opt.abort_level.abort_control( &mut cont)?;
						logger.err( "This error is unskippable.");
						exit( 1)
					}
					// I was too lazy to make safe variant of this code. But hey, it works!
					let ( bootsector, _) = unsafe { transmute::< &[u8], ( &[ u8; 512], usize)>( &bootsector[ 0..512])};
					let bootsector = *bootsector;

					let main_sodalite = match read_file( &logger, &opt, &mut cont, format!( "{core_target}sodalite.bin").into(), "Sodalite") {
						Ok( bytes) => bytes,
						Err( abort_control) => {
							abort_control?;

							vec![]
						},
					};

					let image_file = match File::create( "target/disk.img") {
						Ok( file) => file,
						Err( error) => {

							logger.err( &format!( "Os error {error}. Failed to create disk image file."));
							return opt.abort_level.abort_control( &mut cont)?;
						},
					};

					let mut image = match image::Image::create( image_file, size) {
						Ok( image) => image,
						Err( error) => {

							logger.err( &error);
							return opt.abort_level.abort_control( &mut cont)?;
						},
					};

					match partition_table {
						SelectedPartitionTable::Mbr => match image.write_partition_table( PartitionTable::MBR( MBR::empty()), bootsector) {
							Ok( _) => {},
							Err( error) => {

								logger.err( &error);
								opt.abort_level.abort_control( &mut cont)?
							},
						},
					}

					match image.write_sodalite( &main_sodalite) {
						Ok( _) => {},
						Err( error) => {

							logger.err( &error);
							opt.abort_level.abort_control( &mut cont)?
						},
					}

					logger.tab_down();

				} {
					exit( 1)
				},
				Task::RunQemu { memory, arch } => if Abort == try {



					logger.info( "Running Qemu.");
					logger.tab_up();

					match Command::new( arch.qemu_command())
						.args([
							"-m",
							&format!( "{}B", memory),
							"-drive",
							"format=raw,file=target/disk.img",
							"-display",
							"gtk",
						])
						.spawn() {

							Ok( mut qemu) => {

								let _ = qemu.wait();

							},
							Err( error) => {

								logger.err( &format!( "Failed to start Qemu, {error}."));
								opt.abort_level.abort_control( &mut cont)?
							},

						}
					logger.tab_down();

				} {
					exit( 1)
				},
					Task::Generic { abort_level } => drop( abort_level.inspect( |v| opt.abort_level = *v)),
			}

		if !cont {
			break;
		}

	}

	logger.info("Management program has ended.");
}

/// Worst function declaration I have ever done so far.
fn run_command
<
	P: AsRef< OsStr> + std::fmt::Debug,
	A: Iterator,
> (
	program: P,
	arguments: A,
	logger: &Logger,
	opt: &Opt,
	cont: &mut bool,
	not_exists_error: &str,
	failed_status_error: &str,
)
	->
	AbortControl
	where < A as Iterator>::Item: AsRef< OsStr>
{

	match Command::new( &program).args( arguments).spawn() {
		Ok( mut child) => match child.wait() {
			Ok( status) => {
				if status.success() {
					Continue
				} else {
					logger.err( &format!( "{failed_status_error} {status}."));

					opt.abort_level.abort_control( cont)
				}
			},
			Err( error) => {
				logger.err( &format!( "Could not wait until the end of child process. Program: {program:#?}, error: {error}."));

				opt.abort_level.abort_control( cont)
			},
		},
		Err( error) => {
			logger.err( &format!( "{not_exists_error}. Error: {error}."));

			opt.abort_level.abort_control( cont)
		},
	}
}

pub fn read_file( logger: &Logger, opt: &Opt, cont: &mut bool, file_path: PathBuf, file_object_name: &str) -> Result< Vec< u8>, AbortControl> {

	match File::open( file_path) {
		Ok( mut file) => match file.metadata() {
			Ok( file_metadata) => {
				let mut read_buffer = vec![ 0; file_metadata.len() as usize];

				match file.read( &mut read_buffer) {
					Ok( _) => Ok( read_buffer),
					Err( error) => {

						logger.err( &format!( "Os error {error}. Failed to read {file_object_name}."));
						Err( opt.abort_level.abort_control( cont))
					},
				}

			},
			Err( error) => {

				logger.err( &format!( "Os error: {error}. Failed to read {file_object_name} metadata."));
				Err( opt.abort_level.abort_control( cont))
			},
		},
		Err( error) => {

			logger.err( &format!( "Os error: {error}. Failed to open {file_object_name}."));
			Err( opt.abort_level.abort_control( cont))
		},
	}
}

#[derive( PartialEq)]
pub enum AbortControl {

	Continue,
	Abort,

}

pub use AbortControl::*;

impl FromResidual for AbortControl {

	fn from_residual( _residual: < Self as Try>::Residual) -> Self {

		Abort
	}
}

impl Try for AbortControl {
	type Output = ();
	type Residual = ();

	fn from_output( _output: Self::Output) -> Self {

		Continue
	}

	fn branch( self) -> ControlFlow< Self::Residual, Self::Output> {

		match self {
			Continue => ControlFlow::Continue( ()),
			Abort => ControlFlow::Break( ()),
		}
	}
}
