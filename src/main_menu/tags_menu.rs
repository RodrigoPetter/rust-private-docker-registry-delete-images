use std::ops::{Range, RangeInclusive};

use crate::{
    registry::{ScanElement, TagGroup},
    std_in_out::read_input,
};
use tabled::{builder::Builder, Style};

pub struct TagsMenu {}
impl TagsMenu {
    pub fn open(repository: &ScanElement) {
        println!("{}", repository.repository);
        if repository.tags_grouped_by_digest.len() <= 0 {
            println!("No tags found...");
            todo!("Go back to the repository list instead of exiting");
        }

        loop {
            TagsMenu::print(repository);
            let selected = TagsMenu::select_range(repository.tags_grouped_by_digest.len());
            for s in selected.rev() {
                println!("[{}] TODO: Implement delete", s);
            }
            todo!();
        }
    }

    fn print(repository: &ScanElement) {
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

        builder.add_record(vec!["Voltar"]);
        //TODO: Print the sugestion for deletion

        println!(
            "\nAvaliable tags for the repository [{}]\n",
            repository.repository
        );
        println!(
            "{}",
            builder.index().build().with(Style::markdown()).to_string()
        );
    }

    fn select_range(max: usize) -> RangeInclusive<usize> {
        loop {
            let input =
                read_input::<String>("Select a tag for deletion (Can be a range like `1..23`)");

            //Try to create a range from input
            let input = input.split("..").collect::<Vec<_>>();
            if input.len() == 2 {
                let start = input[0].trim().parse::<usize>();
                let end = input[1].trim().parse::<usize>();

                match (start, end) {
                    (Ok(start), Ok(end)) if end <= max => return start..=end,
                    _ => ()
                }
            } else {
                match input[0].trim().parse::<usize>() {
                    Ok(value) if value <= max => return value..=value,
                    _ => ()
                }
            }

            println!("Not a valid option.");
            continue;
        }
    }
}
