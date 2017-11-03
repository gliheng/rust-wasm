fn main() {
}

#[no_mangle]
pub fn rust_pow(number: i32, power: i32) -> i32 {
    number.pow(power as u32)
}

#[no_mangle]
pub fn hello() {
    println!("Hello, world!");
}
