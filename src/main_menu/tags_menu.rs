use std::ops::RangeInclusive;

use crate::{
    registry::{RegistryClient, ScanElement},
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

        let registry_client = RegistryClient::new();

        loop {
            TagsMenu::print(repository);
            TagsMenu::print_delete_suggestion(repository);
            let selected = TagsMenu::select_range(repository.tags_grouped_by_digest.len());
            for s in selected {
                if let Some(tag_group) = repository.tags_grouped_by_digest.get(s) {
                    registry_client.delete(tag_group);
                    //TODO: how remove the deleted element from the Application state? Passing the property as mutable all way down the call stack seems wrong
                } else {
                    return;
                }
            }
        }
    }

    fn print(repository: &ScanElement) {
        let mut builder = Builder::default();

        builder.set_columns(vec!["tags", "created", "digest"]);

        for group in repository.tags_grouped_by_digest.iter() {
            builder.add_record(vec![
                group
                    .tags
                    .iter()
                    .map(|t| t.name.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
                group.created.to_string(),
                group.digest.clone(),
            ]);
        }

        builder.add_record(vec!["Voltar"]);

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
            let input = input
                .split("..")
                .map(|x| x.trim().parse::<usize>())
                .collect::<Vec<_>>();

            if input.len() == 1 {
                if let Ok(value) = &input[0] {
                    if value <= &max {
                        return value.to_owned()..=value.to_owned();
                    }
                }
            } else if input.len() == 2 {
                if let (Ok(start), Ok(end)) = (&input[0], &input[1]) {
                    if end <= &max && start < end {
                        return start.to_owned()..=end.to_owned();
                    }
                }
            }

            println!("Not a valid option.");
            continue;
        }
    }

    fn print_delete_suggestion(repository: &ScanElement) -> () {
        let suggestion = Some("0..0");

        //Should keep all images younger than a week
        //Should keep the 3 latest versions that start with `V` (ex: "v1.3.2", "v1.2.0", "v1.0.0")
        //Should keep the 2 latest versions that are only numbers (ex: "37813", "19121")
        // repository.tags_grouped_by_digest.iter()
        // .filter(|t| t.created);


        match suggestion {
            Some(suggestion) => println!("\nOur delete suggestion is: {}", suggestion),
            None => println!("\nThis repository SGTM. We suggest you to delete nothing."),
        }
    }
}
