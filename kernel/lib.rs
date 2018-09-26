extern crate memadvise;

// jit
pub mod go;
pub mod rust;
pub mod lisp;

// jit_std
pub mod go_lib;
pub mod rust_lib;
pub mod lisp_lib;

// extras
pub mod translator;

// required
pub mod memory;
