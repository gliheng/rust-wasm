extern crate num;

use num::Complex;

fn main() {}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex {re: .0, im: .0};

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}