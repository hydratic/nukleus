// ext.rs
// provides a call t every function in nukleus for sudo level hacking on the kernel
// --------------------------------------------------------------------------------
// implemented:
//
// terminal
#[warn(dead_code)]
#![no_std]

pub mod terminal;

// terminal functions
pub fn init() { terminal::shell_init(); }
pub fn switch() { terminal::switch(); }
