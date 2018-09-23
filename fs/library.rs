// library.rs
// basically a database of every file on a hard drive

#![no_std]

extern crate hashmap_core;
#[macro_use] extern crate lazy_static;

mod compress;
mod memory;
mod sata;

pub const ENTRIES: i64 = 0;

pub struct ENTRY {
	sector: i64,
	name: &str,
	type: &str,
}

pub enum SECTOR {
	five_twelve_BYTES,
	twenty_fortyeight_BYTES,
	forty_ninetysix_BYTES,
}

lazy_static! {
	static ref 4096_SECTOR: Hashmap<u64, i64> = {
		let mut m = hashmap_core::New();
		
		m.insert(0, 0);
	}
	
	static ref 4096_SECTOR_CONTENTS: Hashmap<u64, &'static str> = {
		let mut m = hashmap_core::New();
		
		m.insert(0, 0);
	}
}
