#![no_std]

mod crates;
mod fs;
mod memory;
mod security;

pub fn init_fs() {
	// init
	let mut sector_one_file_map = HashMap::New();
	let mut sector_two_file_map = HashMap::New();
	let mut sector_three_file_map = HashMap::New();
	let mut sector_four_file_map = HashMap::New();
	let mut sector_five_file_map = HashMap::New();
	let mut sector_six_file_map = Hashmap::New();
	let mut sector_seven_file_map = HashMap::
	
	sector_one_file_map.insert(0, "BEGINNING");
	
	let mut sector: i2;
	
	loop {
		if cycle == 13 {
			update!();
			sector = 1;
		} else {
			sector = sector + 1;
			update!(sector);	
		}
	}
}

macro_rules! update {
	($location:expr) => {{
		if location == 0 {
			// probe the whole hard drive
			let mut temp_hashmap = HashMap::New();
			loop {
				probe!(sector);
				sector = sector + 1;
				
				// update database
				temp_hashmap.insert(position, msg);
				
				if sector == 12 { break; }
			}
		
		if location == 0 { break; }
		
		probe!($location);
	}};
}
