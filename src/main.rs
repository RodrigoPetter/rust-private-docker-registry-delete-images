use std::{io, process::exit};

const SCAN_ID: u16 = 998;
const GC_ID: u16 = 998;
const EXIT_ID: u16 = 0;
const FIXED_OPTIONS: [(u16, &str); 3] = [
    (
        SCAN_ID,
        "Scan all repositories size (this task can take several minutes)",
    ),
    (GC_ID, "Run Garbage Collection"),
    (EXIT_ID, "Exit"),
];

fn main() {
    let avaliable_repositories = get_repositories_from_registry();

    loop {
        println!("List of avaliable repositories and options:");

        //TODO: Check how to concat/extend the Vec with the array so two for loops are not needed
        for (id, text) in avaliable_repositories.iter() {
            print_option(&id, &text);
        }
        for (id, text) in FIXED_OPTIONS {
            print_option(&id, &text);
        }

        let selected = read_input("Select an option:");

        match selected {
            EXIT_ID => exit(0),
            _ => {
                println!("Not a valid option. Try again.");
                continue;
            }
        }
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

fn get_repositories_from_registry() -> Vec<(u16, String)> {
    //TODO: get real repositories from https://xxx.xx/v2/_catalog
    return vec![
        (1, String::from("Placeholder 1")),
        (2, String::from("Placeholder 2")),
        (3, String::from("Placeholder 3")),
        (4, String::from("Placeholder 4")),
    ];
}
