#[allow(unused_extern_crates)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate cpuio;
extern crate libc;
extern crate smoltcp;
extern crate ux;

#[macro_use]
mod ansi;

#[cfg(not(feature = "std"))]
pub mod crates;
#[macro_use]
pub mod container;
#[macro_use]
pub mod crypto;
#[macro_use]
pub mod drivers;
pub mod fs;
#[macro_use]
pub mod memory;
pub mod misc;
