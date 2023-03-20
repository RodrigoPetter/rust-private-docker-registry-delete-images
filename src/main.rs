use menu::{Menu, MenuItem};
use registry::RegistryClient;
use std::{io, process::exit};

mod formats;
mod menu;
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

    let main_menu = Menu::new(
        vec!["Options".to_string()],
        avaliable_repositories
            .into_iter()
            .map(|name| MenuItem {
                force_code: None,
                values: vec![name],
            })
            .collect(),
    );

    loop {
        println!("List of avaliable repositories and options:");

        //TODO: refactor Menu to have a print fn
        println!("{}", main_menu.to_string());
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

        if selected == 0 || selected > (main_menu.itens.len()) {
            match Command::try_from(selected) {
                Ok(cmd) => match cmd {
                    Command::Scan => registry_client.scan(&vec![(0, "placeholder".to_string())]), //TODO: use correct values
                    Command::GC => run_gc(),
                    Command::Exit => exit(0),
                },
                Err(_) => {
                    println!("Not a valid option. Try again.");
                    continue;
                }
            }
        } else {
            let repo_selected = main_menu.itens[selected - 1].values.first().unwrap();
            println!("{}", repo_selected);

            let tags = registry_client.get_tags(&repo_selected);
            if tags.len() <= 0 {
                println!("Nenhuma tag encontrada...");
                todo!("Go back to the repository list instead of exiting");
            }
            println!("\n ==> Avaliable tags gouped by digest <==");

            //TODO: Refactor this ordering. Maybe there is some data structure in rust that can help? BTreeMap?
            let mut display_ordered = tags
                .into_iter()
                .enumerate()
                .map(|(idx, (digest, tags))| {
                    (
                        idx,
                        digest,
                        registry_client.get_created(repo_selected, tags.first().unwrap()),
                        tags.iter()
                            .map(|tag| tag.name.clone())
                            .collect::<Vec<String>>()
                            .join(", "),
                    )
                })
                .collect::<Vec<_>>();
            display_ordered.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

            for (idx, digest, created, tags) in display_ordered {
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
