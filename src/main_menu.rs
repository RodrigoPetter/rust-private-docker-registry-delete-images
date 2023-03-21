use std::process::exit;

use self::tags_menu::TagsMenu;
use crate::{
    registry::{ScanElement, ScanResult},
    std_in_out::read_input,
};
use tabled::{builder::Builder, Style};

mod tags_menu;

enum Command {
    GC,
    EXIT,
}

pub struct MainMenu {}

impl MainMenu {
    const COMMANDS: [Command; 2] = [Command::GC, Command::EXIT];

    pub fn open(scan_result: &ScanResult) {
        loop {
            MainMenu::print(&scan_result);
            let selected = MainMenu::select(&scan_result);
            match selected {
                Some(repo) => TagsMenu::print(&repo),
                None => (),
            }
        }
    }

    fn print(scan_result: &ScanResult) {
        let mut builder = Builder::default();

        builder.set_columns(vec![
            "repository",
            "tags",
            "disk usage",
            "total layers size",
        ]);

        for element in scan_result.elements.iter() {
            builder.add_record(vec![
                element.repository.clone(),
                element
                    .tags_grouped_by_digest
                    .iter()
                    .map(|g| g.tags.len())
                    .sum::<usize>()
                    .to_string(),
                format_size(&element.size_dedup_global),
                format_size(&element.size),
            ]);
        }

        for cmd in MainMenu::COMMANDS {
            match cmd {
                Command::GC => builder.add_record(vec!["Run Garbage Collection"]),
                Command::EXIT => builder.add_record(vec!["Exit"]),
            };
        }

        println!("\nApproximate size used by the compressed images (gzip) in the registry:\n");
        println!(
            "{}",
            builder.index().build().with(Style::markdown()).to_string()
        );

        println!(
            "\nTotal Dedup: {}",
            format_size(&scan_result.total_dedup_size)
        );
        println!("Total: {:>15}\n", format_size(&scan_result.total_size));
    }

    fn select(scan_result: &ScanResult) -> Option<&ScanElement> {
        loop {
            let selected = read_input::<usize>("Select an option:");

            match selected {
                selected if selected < scan_result.elements.len() => {
                    return Some(scan_result.elements.get(selected).unwrap())
                }
                selected if selected == scan_result.elements.len() => todo!("Call GC"),
                selected if selected == scan_result.elements.len() + 1 => exit(0),
                _ => {
                    println!("Not a valid option.");
                    continue;
                }
            }
        }
    }
}

fn format_size(size: &usize) -> String {
    let mega = byte_to_mega(size);
    if mega < 1000.0 {
        return format!("{:<7.2}MB", mega);
    } else {
        return format!("{:<7.2}GB", mega_to_giga(&mega));
    }
}

fn byte_to_mega(bytes: &usize) -> f64 {
    return (bytes.clone() as f64 / 1024.0) / 1024.0;
}

fn mega_to_giga(megas: &f64) -> f64 {
    return megas / 1024.0;
}
