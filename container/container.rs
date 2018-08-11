/* container.rs
 * An implementation of a WASM-based container system in Rust
 * Written by Hayden Curfman, 2018
 */

#![no_std]

extern crate bobbin_wasm;
extern crate cpuio;
#[macro_use]
extern crate lazy_static;
extern crate rand;

pub mod crates;
#[macro_use]
pub mod drivers;
pub mod fs;
#[macro_use]
pub mod memory;
pub mod misc;
#[macro_use]
pub mod vga;

use bobbin_wasm::inter::Interp as interpreter;
use crates::hashmap_core;
use misc::rand;

pub static mut MAX_CONTAINERS: i8 = 12;
pub static mut MEM: i32 = get_mem!();

pub static mut CONTAINERS: i32 = 0;
pub static mut CONTAINER_IDS: vec![0]
pub static mut CONTAINER_MEM: i32 = MEM / CONTAINERS;

lazy_static! {
    static ref CONTAINER_TASKS: Vec<u64> = {
        let mut m = Hashmap::new();
        m.insert(0, "INACTIVE");
        m.insert(1, "INACTIVE");
        m.insert(2, "INACTIVE");
        m.insert(3, "INACTIVE");
        m.insert(4, "INACTIVE");
        m.insert(5, "INACTIVE");
        m.insert(6, "INACTIVE");
        m.insert(7, "INACTIVE");
        m.insert(8, "INACTIVE");
        m.insert(9, "INACTIVE");
        m.insert(10, "INACTIVE");
        m.insert(11, "INACTIVE");
        m.insert(12, "INACTIVE");
    }
}

macro_rules! spawn_container {
    () => {{
        // determine the container ID based on our custom Rand solution
        let rg = Rand::new(0);
        let container_id: u16 = random();
        
        unsafe {
            CONTAINERS += 1;
            CONTAINER_IDS.push(container_id);
        }
    
        let max_mem = CONTAINER_MEM;
    }};
}

macro_rules! assign {
    ($id:expr, $wast:expr) => {{
        try!(let mut container_one = CONTAINER_IDS[1]);

        if CONTAINERS > 2 {
            try!(let mut container_two = CONTAINER_IDS[2]);
            try!(let mut container_three = CONTAINER_IDS[3]);

            if CONTAINERS > 4 {
                try!
            } else if CONTAINERS = 4 {
                try!(let mut container_four = CONTAINER_IDS[4]);
            }
        } else if CONTAINERS = 2 {
            try!(let mut container_two = CONTAINER_IDS[2]);
        }

        let mut status = match $id {
            container_one => "one: ACTIVE",
            container_two => "two: ACTIVE",
            container_three => "three: ACTIVE",
            container_four => "four: ACTIVE",
            container_five => "five: ACTIVE",
            container_six => "six: ACTIVE",
            container_seven => "seven: ACTIVE",
            container_eight => "eight: ACTIVE",
            container_nine => "nine: ACTIVE",
            container_ten => "ten: ACTIVE",
            container_eleven => "eleven: ACTIVE",
            container_twelve => "twelve: ACTIVE",
            _ => "ERROR",
        }

        if status == "ERROR" {
            panic!("Error matching container ID!");
        }
    }};
}

macro_rules! run_wasm {
    ($wast:expr) => {{
        for e in mi.exports() {
            // println!("export: {:?}", e);
            if let ExportDesc::Func(index) = e.export_desc {
                let id = &e.name;            
                match &mi.functions()[index as usize] {
                    &FuncInst::Local { type_index: _, function_index } => {
                        // println!("Calling Local Function {}", function_index);
                        match interp.call(&env, &mi, function_index as usize) {
                            Ok(Some(value)) => {
                                println!("{}() => {:?}", id, value);
                            },
                            Ok(None) => {
                                println!("{}() =>", id);
                            },
                            Err(wasm::Error::Unreachable) => {
                                println!("{}() => error: unreachable executed", id);
                            },
                            Err(wasm::Error::UndefinedTableIndex { id: _ }) => {
                                println!("{}() => error: undefined table index", id);
                            },
                            Err(wasm::Error::SignatureMismatch) => {
                                println!("{}() => error: indirect call signature mismatch", id);
                            },
                            Err(e) => {
                                println!("Error: {:?}", e);
                                println!("---- Stack Dump ----");

                                let mut i = 0;
                                while let Ok(value) = interp.pop() {
                                    println!("{}: {:?}", i, value);
                                    i += 1;
                                }
                                println!("---- END ----");
                            }
                        }
                    },
                    f @ _ => {
                        println!("Unable to call {:?}", f);
                    }
                }

            }
        }
    }
    }};
}

macro_rules! kill_container {
    ($id:expr) => {{
        unsafe {
            CONTAINERS -= 1;
        }
    }};
}
