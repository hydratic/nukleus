#![no_std]

extern crate smoltcp;
extern crate ux;

#[macro_use]
mod drivers;
mod memory;
mod network;

pub mod injection;
pub mod reader;

pub const SOURCE: String = "pakSQL";

pub fn download(package: String) {

}
