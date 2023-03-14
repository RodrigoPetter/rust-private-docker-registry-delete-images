use std::io;

fn main() {
    const SCAN_ID: u16 = 998;
    const GC_ID: u16 = 998;
    const EXIT_ID: u16 = 0;
    const FIXED_OPTIONS: [(u16, &str); 3] = [
        (SCAN_ID, "Scan all repositories size (this task can take several minutes)"),
        (GC_ID, "Run Garbage Collection"),
        (EXIT_ID, "Exit"),
    ];

    loop {
        for (id, text) in FIXED_OPTIONS {
            print_option(&id, &text);
        }

        let _test = read_input("Select one repository:");
    }
}

fn print_option(id: &u16, text: &str) {
    println!("{:<4}- {text}", id);
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
            }
        };
    }
}
