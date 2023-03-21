use tabled::{builder::Builder, Style};

use crate::{
    registry::{ScanElement, ScanResult},
    std_in_out::read_input,
};

pub struct MainMenu {}
pub struct TagsMenu {}

impl MainMenu {
    pub fn print(scan_result: &ScanResult) {
        let mut builder = Builder::default();

        builder.set_columns(vec![
            "repository",
            "tags",
            "disk usage",
            "total layers size",
        ]);

        //TODO: Sort before printing
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

    pub fn select(scan_result: &ScanResult) -> &ScanElement {
        loop {
            let selected = read_input::<usize>("Select an option:");

            if selected > scan_result.elements.len() {
                println!("Not a valid option.");
                continue;
            } else {
                return scan_result.elements.get(selected).unwrap();
            }
        }
    }
}

impl TagsMenu {
    pub fn print(repository: &ScanElement) {
        println!("{}", repository.repository);

        if repository.tags_grouped_by_digest.len() <= 0 {
            println!("No tags found...");
            todo!("Go back to the repository list instead of exiting");
        }

        let mut builder = Builder::default();

        builder.set_columns(vec!["tags", "created", "digest"]);

        //TODO: Sort before printing
        for group in repository.tags_grouped_by_digest.iter() {
            builder.add_record(vec![
                group
                    .tags
                    .iter()
                    .map(|t| t.name.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
                group.created.clone(),
                group.digest.clone(),
            ]);
        }

        println!(
            "\nAvaliable tags for the repository [{}]\n",
            repository.repository
        );
        println!(
            "{}",
            builder.index().build().with(Style::markdown()).to_string()
        );
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
