/* container.rs
 * An implementation of a WASM-based container system in Rust
 * Written by Hayden Curfman, 2018
 */

/*
 * Notes:
 * Individual letter printing takes a lot of room and time;
 * 
 * Possibly use consts or vars in a macro that is called from
 * the assignment macro
 */
#[allow(unused_extern_crates)]
#![cfg_attr(not(std), no_std)]

#[cfg(nukleus)]
extern crate bobbin_wasm;
extern crate cpuio;

#[cfg(not(std))]
pub mod crates;
pub mod fs;
pub mod misc;

#[cfg(not(std))]
#[macro_use]
pub mod drivers;
#[macro_use]
pub mod memory;
#[macro_use]
pub mod vga;

#[cfg(not(std))]
use crates::hashmap_core;
use misc::rand;

#[cfg(std)]
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
        Ok({
            let function = match export {
                "a" => a_FN,
                "b" => b_FN,
                "c" => c_FN,
                "d" => d_FN,
                "e" => e_FN,
                "f" => f_FN,
                "g" => g_FN,
                "h" => h_FN,
                "i" => i_FN,
                "j" => j_FN,
                "k" => k_FN,
                "l" => l_FN,
                "m" => m_FN,
                "n" => n_FN,
                "o" => o_FN,
                "p" => p_FN,
                "q" => q_FN,
                "r" => r_FN,
                "s" => s_FN,
                "t" => t_FN,
                "u" => u_FN,
                "v" => v_FN,
                "w" => w_FN,
                "x" => x_FN,
                "y" => y_FN,
                "z" => z_FN,
                "A" => A_FN,
                "B" => B_FN,
                "C" => C_FN,
                "D" => D_FN,
                "E" => E_FN,
                "F" => F_FN,
                "G" => G_FN,
                "H" => H_FN,
                "I" => I_FN,
                "J" => J_FN,
                "K" => K_FN,
                "L" => L_FN,
                "M" => M_FN,
                "N" => N_FN,
                "O" => O_FN,
                "P" => P_FN,
                "Q" => Q_FN,
                "R" => R_FN,
                "S" => S_FN,
                "T" => T_FN,
                "U" => U_FN,
                "V" => V_FN,
                "W" => W_FN,
                "X" => X_FN,
                "Y" => Y_FN,
                "Z" => Z_FN,
                _ => return Err(wasm::Error::InvalidImport)
            }
        })
    }

    fn dispatch(&self, interp: &mut Interp, _mem: &MemoryInst, _type_index: usize, index: usize) -> Result<(), wasm::Error> {
        Ok({ 
            match index {
                a_FN => print!("a"),
                b_FN => print!("b"),
                c_FN => print!("c"),
                d_FN => print!("d"),
                e_FN => print!("e"),
                f_FN => print!("f"),
                g_FN => print!("g"),
                h_FN => print!("h"),
                i_FN => print!("i"),
                j_FN => print!("j"),
                k_FN => print!("k"),
                l_FN => print!("l"),
                m_FN => print!("m"),
                n_FN => print!("n"),
                o_FN => print!("o"),
                p_FN => print!("p"),
                q_FN => print!("q"),
                r_FN => print!("r"),
                s_FN => print!("s"),
                t_FN => print!("t"),
                u_FN => print!("u"),
                v_FN => print!("v"),
                w_FN => print!("w"),
                x_FN => print!("x"),
                y_FN => print!("y"),
                z_FN => print!("z"),
                A_FN => print!("A"),
                B_FN => print!("B"),
                C_FN => print!("C"),
                D_FN => print!("D"),
                E_FN => print!("E"),
                F_FN => print!("F"),
                G_FN => print!("G"),
                H_FN => print!("H"),
                I_FN => print!("I"),
                J_FN => print!("J"),
                K_FN => print!("K"),
                L_FN => print!("L"),
                M_FN => print!("M"),
                N_FN => print!("N"),
                O_FN => print!("O"),
                P_FN => print!("P"),
                Q_FN => print!("Q"),
                R_FN => print!("R"),
                S_FN => print!("S"),
                T_FN => print!("T"),
                U_FN => print!("U"),
                V_FN => print!("V"),
                W_FN => print!("W"),
                X_FN => print!("X"),
                Y_FN => print!("Y"),
                Z_FN => print!("Z"),
                _ => return Err(wasm::Error::InvalidFunction { id: index as u32 })
            }
        })
    }
}

#[cfg(std)]
pub static STD: i8 = 1;

#[cfg(not(std))]
pub static STD: i8 = 0;

pub static mut MAX_CONTAINERS: i8 = 12;
pub static mut MEM: i32 = get_mem!();

pub static mut CONTAINERS: i32 = 0;
pub static mut CONTAINER_IDS: vec![0]
pub static mut CONTAINER_MEM: i32 = MEM / CONTAINERS;

pub static mut CONTAINER_ONE: u8 = 0;
pub static mut CONTAINER_TWO: u8 = 0;
pub static mut CONTAINER_THREE: u8 = 0;
pub static mut CONTAINER_FOUR: u8 = 0;
pub static mut CONTAINER_FIVE: u8 = 0;
pub static mut CONTAINER_SIX: u8 = 0;
pub static mut CONTAINER_SEVEN: u8 = 0;
pub static mut CONTAINER_EIGT: u8 = 0;
pub static mut CONTAINER_NINE: u8 = 0;
pub static mut CONTAINER_TEN: u8 = 0;
pub static mut CONTAINER_ELEVEN: u8 = 0;
pub static mut CONTAINER_TWELVE: u8 = 0;

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

macro_rules! assign {
    ($id:expr, $wast:expr, $name:expr) => {{
        try!(let mut container_one = CONTAINER_IDS[1]);

        if CONTAINERS > 2 {
            try!(let mut container_two = CONTAINER_IDS[2]);
            try!(let mut container_three = CONTAINER_IDS[3]);

            if CONTAINERS > 4 {
                try!(let mut container_four = CONTAINER_IDS[4]);
                try!(let mut container_five = CONTAINER_IDS[5]);
                if CONTAINERS > 6 {
                
                } else if CONTAINERS = 5 {
                
                }
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

        unsafe {
            if status == "one: ACTIVE" {
                CONTAINER_ONE =
            }
        }

        if status == "ERROR" {
            panic!("Error matching container ID!");
        }

        match status {
            "one: ACTIVE" => print!("Container[1] given task: {}", $name);
            "two: ACTIVE" => print!("Container[2] given task: {}", )
        }

        unsafe {
            print!("with an alloca)ed {}", CONTAINER_MEM);
        }

        if STD == 1 {
            assign_to_thread!()
        }
    }};
}

#[cfg(std)]
macro_rules! assign_to_thread {
    ($id:expr, $thread:expr) => {{
         
    }};
}

#[cfg(std)]
macro_rules! run_wasm {
    ($wast:expr) => {{
        let path = Path::new(matches.value_of("path").unwrap());
        let mut file = File::open(&path)?;
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data)?;

        let path = path.file_stem().unwrap().to_str().unwrap();

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
    
    Ok(())
    }};
}

[cfg(is_kill_working)]
macro_rules! kill_container {
    ($id:expr) => {{
        unsafe {
            CONTAINERS -= 1;
            CONTAINER_MEM = CONTAINERS / MEM;
        }
    }};
}
