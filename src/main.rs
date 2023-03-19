use registry::RegistryClient;
use std::{io, process::exit};

mod formats;
mod registry;

const COMMANDS: [Command; 3] = [Command::Scan, Command::GC, Command::Exit];
enum Command {
    Scan,
    GC,
    Exit,
}

impl TryFrom<usize> for Command {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            value if value == 998 => Ok(Command::Scan),
            value if value == 999 => Ok(Command::GC),
            value if value == 0 => Ok(Command::Exit),
            _ => Err(()),
        }
    }
}

fn main() {
    let registry_client: RegistryClient = RegistryClient::new();
    let avaliable_repositories = registry_client.get_catalog();

    loop {
        println!("List of avaliable repositories and options:");

        for (id, text) in avaliable_repositories.iter() {
            print_option(id, &text);
        }
        for command in COMMANDS {
            match command {
                Command::Scan => print_option(
                    &998,
                    "Scan all repositoriessize (this task can take several minutes)",
                ),
                Command::GC => print_option(&999, "Run Garbage Collection"),
                Command::Exit => print_option(&0, "Exit"),
            }
        }

        let selected = read_input("Select an option:");

        if selected == 0 || selected > (avaliable_repositories.len()) {
            match Command::try_from(selected) {
                Ok(cmd) => match cmd {
                    Command::Scan => registry_client.scan(&avaliable_repositories),
                    Command::GC => run_gc(),
                    Command::Exit => exit(0),
                },
                Err(_) => {
                    println!("Not a valid option. Try again.");
                    continue;
                }
            }
        } else {
            let repo_selected = &avaliable_repositories[selected - 1].1;
            println!("{}", repo_selected);

            let tags = registry_client.get_tags(&repo_selected);
            if tags.len() <= 0 {
                println!("Nenhuma tag encontrada...");
                todo!("Go back to the repository list instead of exiting");
            }
            println!("\n ==> Avaliable tags gouped by digest <==");

            for (idx, digest, created, tags) in
                tags.into_iter().enumerate().map(|(idx, (digest, tags))| {
                    (
                        idx,
                        digest,
                        registry_client.get_created(tags.first().unwrap()),
                        tags.iter()
                            .map(|tag| tag.name.clone())
                            .collect::<Vec<String>>()
                            .join(", "),
                    )
                })
            {
                //TODO: Order tags by creation date
                println!(
                    "{:<3} - {:<35} | {} | {:25.25}...",
                    idx.to_string(),
                    tags,
                    created,
                    digest
                );
            }

            read_input("\nSelect a tag to delete:");
        }
    }
}

fn print_option(id: &u16, text: &str) {
    println!("{:<4}- {text}", id);
}

fn read_input(message: &str) -> usize {
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

fn run_gc() -> () {
    std::process::Command::new("bin/registry")
        .arg("garbage-collect")
        .arg("--delete-untagged")
        .arg("/etc/docker/registry/config.yml")
        .spawn()
        .unwrap()
        .wait()
        .expect("Error while waiting for GC command to finish...");
}
