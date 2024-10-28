use core::panic;
use std::env::args;

use cisco_decrypt::decrypt;

fn main() {
    let pw = args().skip(1).next();
    match pw {
        Some(pw) => println!("{}", decrypt(&pw).unwrap()),
        None => panic!("No password provided!"),
    }
}
