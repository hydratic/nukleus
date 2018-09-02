#[allow(unused_extern_crates)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate cpuio;
extern crate libc;
extern crate smoltcp;
extern crate ux;

// TODO:
// #[macro_use]
// mod ansi;

#[cfg(not(feature = "std"))]
pub mod crates;
#[macro_use]
pub mod crypto;
#[macro_use]
pub mod drivers;
pub mod header;
pub mod fs;
#[macro_use]
pub mod memory;
pub mod misc;

pub mod commands;

pub fn shell_init(out: u8) {
	// get to the newline
	println!(" ");
	
	// print device name, shell prompt
	print!("{} {} ", host, out);
	
	// if running on nukleus, or without the stdlib
	if STD == 0 {
		// determine IO source
		if IO_DEVICE == "PS/2" {
			loop {
				// get IO
				// match command
				// call command
			}
		}
	
		if IO_DEVICE == "USB" {
			panic!("Input type not supported. Please try again with a PS/2 keyboard.");	
		}
	}
}
