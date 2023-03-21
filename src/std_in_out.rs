use std::io::{self, Read};


pub fn press_enter_to_continue(message: &str) {
    println!("{}", message);
    println!("Press enter to continue...");
    io::stdin().read(&mut [0u8]).expect("Failed to read input");
}

pub fn read_input<T>(message: &str) -> T where T: std::str::FromStr {
    loop {
        println!("{message}");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin");

        return match input.trim().parse::<T>() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid input. Try again.");
                continue;
            }
        };
    }
}