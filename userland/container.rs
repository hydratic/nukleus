/* container.rs
 * An implementation of a WASM-based container system in Rust
 * Written by Hayden Curfman, 2018
 */

/*
 * Building:
 *     Requirements:
 *         emscripten
 *         rustc/cargo
 *         python/pip
 *         git
 *
 *    From there, just run build.py to build nukelus or 
 *    build_container.py to build only the containers.
 */

/*
 * TODO:
 *     Add shell API
 *     Compile/test the WebAssembly stdlib that we glued together
 *     Add calls to and from scheme_wasm.ss
 */

#[allow(unused_extern_crates)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate bobbin_wasm;
extern crate cpuio;
extern crate lazy_static;
extern crate libc;

#[cfg(feature = "std"))]
extern crate rand;

#[cfg(not(feature = "std"))]
pub mod crates;
pub mod exec;
pub mod fs;
pub mod misc;

#[cfg(not(feature = "std"))]
#[macro_use]
pub mod drivers;
#[macro_use]
pub mod memory;
#[macro_use]
pub mod vga;

#[cfg(not(feature = "std"))]
use crates::hashmap_core;
use misc::rand;

// #[cfg(feature = "std"))]
// mod wasm_std;

#[cfg(feature = "std"))]
use std::collections::Hashmap;
use std::fs::File;
use std::thread;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    WasmError(wasm::Error),
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IoError(other)
    }
}

impl From<wasm::Error> for Error {
    fn from(other: wasm::Error) -> Self {
        Error::WasmError(other)
    }
}

impl HostHandler for Handler {
    fn import(&self, _module: &str, export: &str, _import_desc: &ImportDesc) -> Result<usize, wasm::Error> {
        Ok({ let function = match export {
               "print" => print_STD,
                _ => return Err(wasm::Error::InvalidImport),
            }
        })
    }

    fn dispatch(&self, interp: &mut Interp, _mem: &MemoryInst, _type_index: usize, index: usize) -> Result<(), wasm::Error> {
        Ok({ 
            match index {
                print_STD => print!("{}", std_out),
                _ => return Err(wasm::Error::InvalidFunction { id: index as u32 }),
            }
        })
    }
}

lazy_static! {
    pub static ref STD_OUT: HashMap<u64, &'static str> = {
        let mut out = Hashmap::new()
        m.insert("0, BEGINNING");
        m
    }
}

#[cfg(std)]
static STD: i8 = 1;

#[cfg(not(std))]
static STD: i8 = 0;

static mut MAX_CONTAINERS: i8 = 12;
static mut MEM: i32 = get_mem!();

static mut THREAD_IDS: Vec<u64< = vec![0];

static mut CONTAINERS: i32 = 0;
static mut CONTAINER_IDS: Vec<u64> = vec![0];
static mut CONTAINER_MEM: i32 = MEM / CONTAINERS;

static mut CONTAINER_ONE: u8 = 0;
static mut CONTAINER_TWO: u8 = 0;
static mut CONTAINER_THREE: u8 = 0;
static mut CONTAINER_FOUR: u8 = 0;
static mut CONTAINER_FIVE: u8 = 0;
static mut CONTAINER_SIX: u8 = 0;
static mut CONTAINER_SEVEN: u8 = 0;
static mut CONTAINER_EIGT: u8 = 0;
static mut CONTAINER_NINE: u8 = 0;
static mut CONTAINER_TEN: u8 = 0;
static mut CONTAINER_ELEVEN: u8 = 0;
static mut CONTAINER_TWELVE: u8 = 0;

macro_rules! spawn_container {
    () => {{
        // determine the container ID based on our custom Rand solution
        let rg = Rand::new(0);
        let container_id: u16 = random();

        unsafe {
            CONTAINERS += 1;
            CONTAINER_IDS.push(container_id);

            let max_mem = CONTAINER_MEM;
        }    
    }};
}

// TODO: Rename CONTAINERS to $containers
macro_rules! give_ids {
    ($containers:expr) => {{
         try!(let mut container_one = CONTAINER_IDS[1]);

        if $containers > 2 {
            try!(let mut container_two = CONTAINER_IDS[2]);
            try!(let mut container_three = CONTAINER_IDS[3]);

            if $containers > 4 {
                try!(let mut container_four = CONTAINER_IDS[4]);
                try!(let mut container_five = CONTAINER_IDS[5]);

                if $containers > 6 {
                    try!(let mut container_six = CONTAINER_IDS[6]);
                    try!(let mut container_seven = CONTAINER_IDS[7]);

                    if $containers > 8 {
                        try!(let mut container_eight = CONTAINER_IDS[8]);
                        try!(let mut container_nine = CONTAINER_IDS[9]);

                        if $containers > 10 {
                            try!(let mut container_ten = CONTAINER_IDS[10]);
                            try!(let mut container_eleven = CONTAINER_IDS[11]);

                            if $containers > 12 {
                                panic!("Maximum container limit reached. Aborting!");
                            }

                            if $containers == 12 {
                                try!(let mut container_twelve = CONTAINER_IDS[12]);
                            }

                        } else if $containers == 10 {
                            try!(let mut container_ten = CONTAINER_IDS[10]);
                        }

                    } else if $containers == 8 {
                        try!(let mut container_eight = CONTAINER_IDS[8]);
                    }

                } else if $containers == 6 {
                    try!(let mut container_six = CONTAINER_IDS[6]);
                }

            } else if $containers == 4 {
                try!(let mut container_four = CONTAINER_IDS[4]);
            }

        } else if $containers == 2 {
            try!(let mut container_two = CONTAINER_IDS[2]);
        }
    }};
}

macro_rules! assign {
    ($id:expr, $wast:expr, $name:expr) => {{
        /*
         *  If you place this block of code in an unsafe wrapper, you can
         *  delete the one below it and make the match bit modify the container
         *  statics directly.
         */
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

        unsafe {
            if status == "one: ACTIVE" {
                CONTAINER_ONE = 1;
            }

            if status == "two: ACTIVE" {
                CONTAINER_TWO = 1;
            }

            if status == "three: ACTIVE" { 
                CONTAINER_THREE = 1;
            }

            if status == "four: ACTIVE" {
                CONTAINER_FOUR = 1;
            }

            if status == "five: ACTIVE" {
                CONTAINER_FIVE = 1;
            }

            if status == "six: ACTIVE" {
                CONTAINER_SIX = 1;
            }

            if status == "seven: ACTIVE" {
                CONTAINER_SEVEN = 1;
            }

            if status == "eight: ACTIVE" {
                CONTAINER_EIGHT = 1;
            }

            if status == "nine: ACTIVE" {
                CONTAINER_NINE = 1;
            }

            if status == "ten: ACTIVE" {
                CONTAINER_TEN = 1;
            }

            if status == "eleven: ACTIVE" {
                CONTAINER_ELEVEN = 1;
            }

            if status == "twelve: ACTIVE" {
                CONTAINER_TWELVE = 1;
            }

            if status == "ERROR" {
                panic!("Error matching container ID!");
            } 
        }

        match status {
            "one: ACTIVE" => println!("Container[1] given task: {}", $name),
            "two: ACTIVE" => println!("Container[2] given task: {}", $name),
            "three: ACTIVE" => println!("Container[3] given task: {}", $name),
            "four: ACTIVE" => println!("Container[4] given task: {}", $name),
            "five: ACTIVE" => println!("Container[5] given task: {}", $name),
            "six: ACTIVE" => println!("Container[6] given task: {}", $name),
            "seven: ACTIVE" => println!("Container[7] given task: {}", $name),
            "eight: ACTIVE" => println!("Container[8] given task: {}", $name),
            "nine: ACTIVE" => println!("Container[9] given task: {}", $name),
            "ten: ACTIVE" => println!("Container[10] given task: {}", $name),
            "eleven: ACTIVE" => println!("Container[11] given task: {}", $name),
            "twelve: ACTIVE" => println!("Container[12] given task: {}", $name),
        }

        unsafe {
            print!("with an allocated {}", CONTAINER_MEM);
        }

        if STD == 1 {
            assign_to_thread!()
        }
    }};
}

macro_rules! assign_to_thread {
    ($id:expr, $wast:expr) => {{
        // memory allocation is still a WIP 
        thread::spawn(move || {
            run_wasm!($wast);
            let mut thread_id = random();
            let mut thread_loop = 0;

            loop {

                if thread_id ==  
            }
        });
    }};
}

macro_rules! run_wasm {
    ($wast:expr) => {{
        if STD == 1 {
            let mut file = File::open($wast)?;
            let mut data: Vec<u8> = Vec::new();
            file.read_to_end(&mut data)?;
            
            let _out = String::new();

            let h = Handler {};

            let buf = &mut [0u8; 65536 * 2];
            let (buf, mut env) = Environment::new(buf, h);    

            // let math = load_file("local_test/math.wasm")?;
            // println!("loading {:?}", "math");
            // let (buf, _) = env.load_module("math", buf, math.as_ref())?;
            // println!("loading {:?}", path);

            let (buf, mi) = env.load_module(path, buf, data.as_ref())?;

            // Interpreter

            let mut interp = Interp::new(buf);

            if matches.is_present("run-all-exports") {
                for e in mi.exports() {
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

            Ok(())
        } else { break; }
    }};
}

macro_rules! kill_container {
    ($id:expr) => {{
        unsafe {
            CONTAINERS -= 1;
            CONTAINER_MEM = CONTAINERS / MEM;


        }
    }};
}

// these functions are used by the C bridge
pub fn new_container(name: ) {

}
