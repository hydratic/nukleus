// rust_jit.rs
// translates rust code to a Lisp implementation

#![no_std]

extern crate cpuio;

mod memory;
mod syscall;
