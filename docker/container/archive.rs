// archive.rs
// provides direct calling of all of archive.go's fucntions
// 
// TODO:
// types

#![no_std]

mod memory;

extern {
	fn ResolvePath();
	fn StatPath();
}
