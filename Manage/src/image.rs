use std::mem::{ transmute};
use std::fs::File;
use std::io::{ Seek, SeekFrom, Write, Error as IOError};

pub struct Image {

	image_file: File,

	/// Size is in bytes. Must be divisible by `512`.
	pub size: u64,
	partition_table: PartitionTable,

}

impl Image {

	pub fn create( image_file: File, size: u64) -> Result< Self, String> {

		match image_file.set_len( size) {
			Ok( _) => {},
			Err( error) => return Err( format!( "Os error {error}. Could not set proper file length.")),
		}

		Ok( Self {
			image_file,
			size,
			partition_table: PartitionTable::None,
		})
	}

	pub fn write_partition_table( &mut self, partition_table: PartitionTable, bootsector: [ u8; 512]) -> Result< (), String> {

		fn bootsector_write_error( error: IOError) -> Result< (), String> {

			Err( format!( "Os error {error}. Bootsector writing failed"))
		}

		match self.image_file.seek( SeekFrom::Start( 0)) {
			Ok( _) => {},
			Err( error) => return Err( format!( "Os error {error}. Pointing to file's beggining failed")),
		}

		match &partition_table {
			PartitionTable::MBR( mbr) => match self.image_file.write( &mbr.combine_with_bootsector( bootsector)) {
				Ok( _) => {},
				Err( error) => return bootsector_write_error( error),
			},
			PartitionTable::None => match self.image_file.write( &bootsector) {
				Ok( _) => {},
				Err( error) => return bootsector_write_error( error),
			},
		}

		self.partition_table = partition_table;

		Ok(())
	}

	pub fn write_sodalite( &mut self, sodalite: &[ u8]) -> Result< (), String> {
		let sodalite_begin_sector = self.partition_table.partition_head_end();
		let sodalite_begin_sector_chs = sodalite_begin_sector.as_chs();

		match self.image_file.seek( SeekFrom::Start( 0xd3)) {
			Ok( _) => {},
			Err( error) => return Err( format!( "Os error {error}. Pointing to Sodalite's low settings failed")),
		}

		match self.image_file.write( &[
			sodalite_begin_sector_chs.sector(),
			sodalite_begin_sector_chs.head,
			( sodalite.len() / 512 / 63) as u8 + 1 + sodalite_begin_sector_chs.head
		]) {
			Ok( _) => {},
			Err( error) => return Err( format!( "Os error {error}. Saving Sodalite's low settings failed")),
		}

		match self.image_file.seek( SeekFrom::Start( sodalite_begin_sector.into())) {
			Ok( _) => {},
			Err( error) => return Err( format!( "Os error {error}. Pointing at Sodalite beggining sector failed")),
		}

		match self.image_file.write( sodalite) {
			Ok( _) => {},
			Err( error) => return Err( format!( "Os error {error}. Writing Sodalite failed")),
		}

		Ok( ())
	}

}

pub enum PartitionTable {

	MBR ( MBR),
	None,

}

impl PartitionTable {

	/// First free for bootloader sector.
	pub const fn partition_head_end( &self) -> LBA {

		match self {
			PartitionTable::MBR( _) => LBA::from_value( 1),
			PartitionTable::None => LBA::from_value( 1),
		}
	}

}

pub struct MBR {

	/// Random disk signature.
	pub disk_signature: u32,
	pub primary_partitions: [ MbrPartition; 4],

}

impl MBR {

	pub fn empty() -> Self {

		Self {
			disk_signature: 420,
			primary_partitions: [
				MbrPartition::empty(),
				MbrPartition::empty(),
				MbrPartition::empty(),
				MbrPartition::empty(),
			],
		}
	}

	fn combine_with_bootsector( &self, bootsector: [ u8; 512]) -> [ u8; 512] {
		//let mut bootsector = bootsector;

		// I don't care it's `unsafe`.
		unsafe {
			let main_mbr = ( transmute::< &[u8; 512], usize>( &bootsector) + 0x1b8) as * mut ( u32, u16, [ MbrPartition; 4]);

			main_mbr.write( ( self.disk_signature, 0x0000, self.primary_partitions.clone()));

		}

		bootsector
	}

}

#[repr( C, packed)]
pub struct MbrPartition {

	status: u8,
	first_sector_chs: CHS,
	partition_type: u8,
	last_sector_chs: CHS,
	first_sector: LBA,
	sector_size: LBA,

}

impl MbrPartition {

	pub fn empty() -> Self {

		Self {
			status: 0,
			first_sector_chs: CHS::from_values( 0, 0, 0),
			partition_type: 0,
			last_sector_chs: CHS::from_values( 0, 0, 0),
			first_sector: LBA::from_value( 0),
			sector_size: LBA::from_value( 0),
		}
	}

	pub fn new( partition_type: u8, first_sector: LBA, last_sector: LBA) -> Self {
		let sector_size = last_sector - first_sector;

		Self {
			status: 0x80,
			first_sector_chs: first_sector.as_chs(),
			partition_type,
			last_sector_chs: last_sector.as_chs(),
			first_sector,
			sector_size,
		}
	}

}

impl Clone for MbrPartition {
	fn clone( &self) -> Self {

		Self {
			status: self.status,
			first_sector_chs: self.first_sector_chs.clone(),
			partition_type: self.partition_type,
			last_sector_chs: self.last_sector_chs.clone(),
			first_sector: self.first_sector.clone(),
			sector_size: self.sector_size.clone(),
		}
	}
}

#[repr( C, packed)]
pub struct CHS {

	head: u8,
	sector_cylinder: u16,

}

impl CHS {

	pub fn from_values( sector: u8, head: u8, cylinder: u16) -> Self {

		Self {
			head,
			sector_cylinder: ( sector & 0b00111111) as u16 | ( cylinder << 6)
		}
	}

	pub fn sector( &self) -> u8 {

		self.sector_cylinder as u8 & 0b00111111
	}


}

impl Clone for CHS {

	fn clone( &self) -> Self {

		Self {
			head: self.head,
			sector_cylinder: self.sector_cylinder,
		}
	}

}

#[repr( C, packed)]
pub struct LBA {

	sector: u32,

}

impl LBA {

	pub const fn from_value( sector: u32) -> Self {

		Self { sector }
	}

	pub fn as_chs( &self) -> CHS {
		let mut sector = ( self.sector % 63 + 1) as u8;
		let total_head = self.sector / 63;
		let mut head = ( total_head % 16) as u8;
		let mut cylinder = total_head / 16;
		if cylinder > 1023 {

			sector = 63;
			head = 15;
			cylinder = 1023;

		}

		CHS::from_values( sector, head, cylinder as u16)
	}


}

impl Into< u64> for LBA {

	fn into( self) -> u64 {

		self.sector as u64 * 512
	}

}

impl Clone for LBA {
	fn clone( &self) -> Self {

		Self {
			sector: self.sector,
		}
	}
}

impl Copy for LBA {}

impl std::ops::Sub for LBA {
	type Output = LBA;

	fn sub( self, rhs: Self) -> Self::Output {

		Self { sector: self.sector - rhs.sector}
	}
}
