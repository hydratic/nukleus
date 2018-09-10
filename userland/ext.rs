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

// global constants
pub const SUDO: i8 = 0;

// security functions
pub fn no_sudo() { 
  security::disable_sudo();
  unsafe {
    
  }
}

// terminal functions
pub fn init() { terminal::shell_init(); }
pub fn switch() { terminal::switch(); }
