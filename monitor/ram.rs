#![no_std}

extern crate average;

#[macro_use]
mod drivers;
mod memory;

pub const LAST_ACTIVITY: String = "none";

pub const KUBERNETES: i32;
pub const BACKGROUND: i32;

pub const TOTAL_RAM: i32;
pub const RAM_BEFORE: i32;
pub const RAM_AFTER: i32;

pub const ONE_CONTAINER_TARGET: i32;
pub const MULTI_CONTAINER_TARGET: i32 = ONE_CONTAINER_TARGET * CONTAINERS + KUBERNETES + BACKGROUND;

macro_rules! set_last_activity {
	($last:expr) => {{
	
	}};
}

macro_rules! update_mem {
	() => {{
		let mut total_ram = TOTAL_RAM;
		
		let mut avg_kubernetes = KUBERNETES;
		let mut avg_docker = ONE_CONTAINER_TARGET;
		let mut avg_multi_docker = MULTI_CONTAINER_TARGET;
	}};
}
