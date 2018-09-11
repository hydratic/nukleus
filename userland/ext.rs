// ext.rs
// provides a call t every function in nukleus for sudo level hacking on the kernel
// --------------------------------------------------------------------------------
// implemented:
//
// terminal
#[warn(dead_code)]
#![no_std]

pub mod security;
pub mod terminal;

// SECURITY

pub enum NON_SUDO_TASKS {
	bare_functions,
	ssh,
}

pub mut const SUDO: i8 = 0;

pub fn no_sudo() { 
  	security::disable_sudo();
  	unsafe {
    	if SUDO == 1 {
			SUDO -= 1;	
		}
	}
}

pub fn sudo() {
	security::enable_sudo();
	unsafe {
		if SUDO == 0 {
			
		SUDO += 1;
	}
}

// TERMINAL

pub fn init() { terminal::shell_init(); }
pub fn switch() { terminal::switch(); }

