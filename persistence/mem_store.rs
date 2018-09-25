#![no_std]

// local
extern crate hashmap_core;
extern crate ux;

// external
extern crate memadvise;

mod memory;

macro_rules! start_hashmap {
	($init:expr) => {{
		if $init == 0 {
			unsafe {
				pub const MEM_HASH: Hashmap<u64> = HashMap::New();
			}
		}
	}};
}

macro_rules! add_to_mem {
	($location:expr, $msg:expr) => {{
	
	}};
}
