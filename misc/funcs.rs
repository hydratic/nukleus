// random functions that nukleus relies on
// ---------------------------------------
// List:
// number_to_vec
// ---------------------------------------
#![no_std]

mod memory;

pub fn number_to_vec(n: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}
