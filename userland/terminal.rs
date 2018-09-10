#[allow(unused_extern_crates)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate cpuio;
extern crate libc;
extern crate smoltcp;
extern crate ux;

pub mod misc;
pub mod docker;
pub mod commands;
pub mod fs;

// TODO:
// #[macro_use]
// mod ansi;

#[cfg(not(feature = "std"))]
pub mod crates;

#[macro_use]
pub mod crypto;

#[macro_use]
pub mod drivers;

#[macro_use]
pub mod memory;

pub shell_init(out: u8) {
	// get to the newline
	println!(" ");
	
	// print device name, shell prompt
	print!("{} {} ", host, out);
	
	// if running on nukleus, or without the stdlib
	if STD == 0 {
		// determine IO source
		if IO_DEVICE == "PS/2" {
			loop {
				if TYPE == "UNIX" {
					let in = get_io!();
					
					let cmd = match in {
						
					}
					
					// match command
					// call command
				} else {
					panic!("Terminal type not supported.")	
				}
			}
		}
	
		if IO_DEVICE == "USB" {
			panic!("Input type not supported. Please try again with a PS/2 keyboard.");	
		}
	} else if STD == 1 {
		// get IO normally
	}
}
