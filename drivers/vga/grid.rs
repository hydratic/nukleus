#![no_std]

#[macro_use]
mod vga;
mod memory;

pub struct Grid {
    width: i32,
    height: i32,
}

macro_rules! grid_init {
    () => {{
        let grid: Grid = { width, height };
    }};
}

macro_rules! heatmap {
    ($pixel:expr) => {{
        
    }};
}

pub fn heatmap() {
    get_resolution!();
    grid_init!();
    loop {}
}
