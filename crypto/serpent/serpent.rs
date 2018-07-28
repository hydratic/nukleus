#![no_std]

extern crate libm;

mod memory;

// Constants
pub const DIR_ENCRYPT: i8 = 0;
pub const DIR_DECRYPT: i8 = 1;
pub const MODE_ECB: i8 = 1;
pub const MODE_CBC: i8 = 2;
pub const MODE_CFB1: i8 = 3;
pub const TRUE: i8 = 1;
pub const FALSE: i8 = 0;

// Error Codes
pub const BAD_KEY_DIR: i8 = -1;
pub const BAD_KEY_MAT: i8 = -2;
pub const BAD_KEY_INSTANCE: i8 = -3;
pub const BAD_CIPHER_MODE: i8 = -4;
pub const BAD_CIPHER_STATE: i8 = -5;

pub const MAX_KEY_SIZE: i8 = 64;
pub const MAX_IV_SIZE: i8 = 32;

// Structs
pub struct keyInstance {
    direction: str,
    keyLen: i32,
    keyMaterial[MAX_KEY_SIZE+1]: char,
    key[8]: u64,
    subkeys[33][4]: u64
}

pub struct cipherInstance {
    mode: str,
    IV[MAX_IV_SIZE]: char,
    blockSize: i32
}
