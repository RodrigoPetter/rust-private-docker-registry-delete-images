use std::{io::{self, Read}, process::exit};

const COMMANDS: [Command; 3] = [Command::Scan, Command::GC, Command::Exit];
enum Command {
    Scan,
    GC,
    Exit,
}

impl TryFrom<u16> for Command {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            value if value == 998 => Ok(Command::Scan),
            value if value == 999 => Ok(Command::GC),
            value if value == 0 => Ok(Command::Exit),
            _ => Err(()),
        }
    }
}

fn main() {
    let avaliable_repositories = get_repositories_from_registry();

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

        match Command::try_from(selected) {
            Ok(cmd) => match cmd {
                Command::Scan => run_scan(&avaliable_repositories),
                Command::GC => run_gc(),
                Command::Exit => exit(0),
            },
            Err(_) => {
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

/// This function is currently experiencing some issues with accurately tracking the space used by Docker images.
/// Specifically, it's only using the digest of the tag to calculate the size,
/// which can result in inaccuracies when there are multiple layers that share the same blob.
/// In order to properly calculate the total size, layers that have the same SHA256 hash should
/// only be counted once towards the total.
/// 
/// For example, many images use the Alpine Linux image as their base, so if this image is included
/// in multiple layers, its size should only be counted once towards the total size.
/// However, if each layer is counted separately, it could result in an overestimation of the total size.
fn run_scan(repos: &Vec<(u16, String)>) -> () {
    struct RepoSize {
        index: u16,
        name: String,
        size: f64,
        tag_count: usize,
    }

    let mut repo_sizes: Vec<RepoSize> = vec![];

    for (index, repo) in repos {
        //TODO: get real tada about the tags inside a repository from https://xxx.xx/v2/_REPOSITORY_/tags/list"
        let tags_list = vec!["v3.1.2", "v2.9.8", "v1.2.1", "v1.0.0", "v0.1.0"];

        let mut repo_size_accumulator: f64 = 0.0;

        //TODO: This Vec should be stored outside this loop. Diferent repositories can share the same digest/blob for the tag. I'm keeping it here just for comparison with the GO version. Fit it when the program is ready.
        // Vector used to store digests whose size has already been queried to prevent
        // duplicate queries and avoid double-counting the size of tags that share the same blob.
        let mut digest_tracker: Vec<String> = vec![];

        for tag in tags_list.iter() {
            //TODO: get real tag digest from https://xxx.xx/v2/_REPOSITORY_/manifests/_TAG_ -> NEEDS: "Accept: application/vnd.docker.distribution.manifest.v2+json"
            let tag_digest = format!(
                "{}{}",
                tag, "sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b"
            );

            if !digest_tracker.contains(&tag_digest) {
                println!("[{}] Fetching [{}] repository size...", index, repo);
                digest_tracker.push(tag_digest);

                //TODO: get real tag size from https://xxx.xx/v2/_REPOSITORY_/manifests/_TAG_ -> same url than before, but without the Accept header
                repo_size_accumulator += 1.45354;
            }
        }

        repo_sizes.push(RepoSize {
            index: index.clone(),
            name: repo.clone(),
            size: repo_size_accumulator,
            tag_count: tags_list.len(),
        });
    }

    repo_sizes.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());

    println!("Approximate size used by the compressed images in the registry (note: this size does not take into account that the same layer can be shared between multiple Docker images):");

    let mut total : f64 = 0.0;
    for element in repo_sizes.into_iter() {
        println!("{:>10.2} MB - {:^3} tags - {} ({})", element.size, element.tag_count, element.name, element.index);
        total += element.size;
    }

    println!("Total: {:>7.3}GB", total/1024.0);
    println!("Press enter to continue...");
    io::stdin().read(&mut [0u8]).expect("Failed to read input");
    
    return ();
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
