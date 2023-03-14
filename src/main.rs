use std::io;

fn main() {
    loop {
        let _test = read_input("Select one repository:");
    }
}

fn read_input(message: &str) -> u16 {
    loop {
        println!("{message}");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin");

        return match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid input. Try again.");
                continue;
            },
        };
    }
}
