// vga.rs
// our VGA driver
// ------------------------

#![no_std]

extern crate libm;

// mod grid;
mod memory;
// mod vesa;

pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

pub const fg: Color::White;
pub const bg: Color::Black;

macro_rules! get_resolution {
    ($devices:expr) => {{
        
    }};
}

fn drawchar_8BPP(c: u8, x: i16, y: i16, fgcolor: i16, bgcolor: i16) {
    let mut dest;
    let mut dest32: u32;
    let mut src: u8;
    let mut row: i16;
    let mut fgcolor32: u32;
    let mut bgcolor32: u32;
    
    src = font + c * 16;
    dest = videoBuffer + y * bytes_per_line + x;
    for row 0..16 {
        // TODO
    }
}

fn drawchar_transparent_8B)P(c: u8, x: i16, y: i16, fgcolor: i16, bgcolor: i16) {
    let mut dest;
    let mut dest32: u32;
    let mut src: u8;
    let mut row: i16;
    let mut fgcolor32: u32;
    let mut bgcolor32: u32;
    
    src = font + c * 16;
    dest = videoBuffer + y * bytes_per_line + x;
    for row 0..16 {
        // TODO
    }
}

macro_rules! print {
    ($text:expr) => {{
        loop {
            // let grid = Grid::new(  );
            // drawchar_8BPP()
        }
    }};
}

macro_rules! print_transparent {
    ($text:expr) => {{
        // let grid = Grid::new();
        // drawchar_8bpp();
    }};
}
