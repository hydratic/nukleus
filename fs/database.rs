#![no_std]

extern crate hashmap_core;
extern crate nukleus_core;
extern crate nukleus_security;

mod probe;

pub fn init_fs() {
	// init
	let mut sector_one_file_map = HashMap::New();
	let mut sector_two_file_map = HashMap::New();
	let mut sector_three_file_map = HashMap::New();
	let mut sector_four_file_map = HashMap::New();
	let mut sector_five_file_map = HashMap::New();
	let mut sector_six_file_map = Hashmap::New();
	let mut sector_seven_file_map = HashMap::New();
	
	sector_one_file_map.insert(0, "BEGINNING");
	sector_two_file_map.insert(0, "BEGINNING");
	sector_three_file_map.insert(0, "BEGINNING");
	sector_five_file_map.insert(0, "BEGINNING");
	sector_six_file_map.insert(0, "BEGINNING");
	sector_seven_file_map.insert(0, "BEGINNING");
	sector_eight_file_map.insert(0, "BEGINNING");
	sector_nine_file_map.insert(0, "BEGINNING");
	sector_ten_file_map.insert(0, "BEGINNING");
	sector_eleven_file_map.insert(0, "BEGINNING");
	sector_twelve_file_map.insert(0, "BEGINNING");
	
	let mut sector: i2;
	
	loop {
		if given == 1 {
			update!(location);
			sector = 1;
		} else {
			sector = sector + 1;
			update!(0);
			
			match sector {
				1 => {
					sector_one_file_map.insert(size, filename);
				}
				2 => {
					sector_two_file_map.insert(size, filename);
				}
				3 => {
					sector_three_file_map.insert(size, filename);
				}
				4 => {
					sector_four_file_map.insert(size, filename);
				}
				5 => {
					sector_five_file_map.insert(size, filename);
				}
				6 => {
					sector_six_file_map.insert(size, filename);
				}
				7 => {
					sector_seven_file_map.insert(size, filename);
				}
				8 => {
					sector_eight_file_map.insert(size, filename);
				}
				9 => {
					sector_nine_file_map.insert(size, filename);
				}
				10 => {
					sector_ten_file_map.insert(size, filename);
				}
				11 => {
					sector_eleven_file_map.insert(size, filename);
				}
				12 => {
					sector_twelve_file_map.insert(size, filename);
				}
				_ => {
					panic!("Invalid sector read!");
				}
			}
		}
	}
}

macro_rules! update {
	($location:expr) => {{
		if $location == 0 {
			// probe the whole hard drive
			let mut temp_hashmap = HashMap::New();
			loop {
				probe!(sector);
				sector = sector + 1;
				
				// update database
				temp_hashmap.insert(position, msg);
				
				if sector == 12 { break; }
			}
		
		if $location == 0 { break; }
		
		probe!($location);
	}};
}
