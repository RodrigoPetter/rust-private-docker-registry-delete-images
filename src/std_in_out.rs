use std::io::{self, Read};


pub fn press_enter_to_continue(message: &str) {
    println!("{}", message);
    println!("Press enter to continue...");
    io::stdin().read(&mut [0u8]).expect("Failed to read input");
}