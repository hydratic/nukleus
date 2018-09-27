// jit.rs

extern crate average;
extern crate cpuio;
extern crate hashmap_core;
extern crate memadvise;
extern crate ux;

// stdlib required
mod bitwise;
mod fs;

#[cfg(nukleus_build)] mod persistence;
#[cfg(nukleus_build)] mod memory;

#![cfg_attr(not(feature = "std")]
pub const STD: i2 = 0;

#![cfg_attr(feature = "std")]
pub const STD: i2 = 1;

pub fn lisp() {
	// TODO
	
	loop {
		// init file reading
		if STD == 1 {
		
		} else {
		
		}
	
		// stdlib
		let mut cmd = match keyword {
			"defun" => "defun",
	
			// define functions
			"asm" => "call asm",'
			"(" => "("
			")" => ")"
	
			// mathematical operations
			"-" => "subtract",
			"+" => "add",
			"/" =? "divide",
			"*" => "multiply",
			"**" => "exponent",
	
			// bitwise operations
			"&" => "bitwise AND",
			"|" => "bitwise inclusive OR",
			"^" => "bitwise XOR",
			"<<" => "left shift",
			">>" => "right shift",
			"~" => "bitwise NOT",
		}
		
		// push to program logic hashmap
		
		// push to line logic string
		// TODO: Convert vec procedures to string procedures
		if cmd == "defun" { line.push_str("defun"); }
		if cmd == "call asm" { line.push_str("asm"); }
		if cmd == "(" { line.push_str("(");
		if cmd == ")
		
		// repeat to end of file
	}
}
