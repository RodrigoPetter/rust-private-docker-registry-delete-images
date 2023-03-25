use std::ops::RangeInclusive;

use crate::{
    registry::{RegistryClient, ScanElement},
    std_in_out::read_input,
};
use chrono::{Days, Utc};
use regex::Regex;
use tabled::{builder::Builder, Style};

pub struct TagsMenu {}
impl TagsMenu {
    pub fn open(repository: &mut ScanElement) {
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
            
            if selected.end() == &repository.tags_grouped_by_digest.len() {
                //is the return option
                return;
            }else {
                for tag_group in repository.tags_grouped_by_digest.splice(selected, std::iter::empty()) {
                    registry_client.delete(&tag_group);                    
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

        builder.add_record(vec!["RETURN"]);

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
                read_input::<String>("Select a tag for deletion (Can be a range like `Y..X`)");

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
                    //max-1 to not accept a range where the "Return" option is present
                    if end <= &(max-1) && start < end {
                        return start.to_owned()..=end.to_owned();
                    }
                }
            }

            println!("Not a valid option.");
            continue;
        }
    }

    fn print_delete_suggestion(repository: &ScanElement) -> () {
        let tag_patter = Regex::new(r"v\d\.").unwrap();
        let mut tag_patter_counter = 0; //Represent Releases that went to staging and production
        let mut unknown_patter_counter = 0; //Represent versions deployed to the test environment
        let mut suggestion = None;


        //This will only work with an ordered Vec<>
        for i in (0..(repository.tags_grouped_by_digest.len())).rev() {
            let tg = repository.tags_grouped_by_digest.get(i).unwrap();

            //Should keep the 3 latest versions that start with `v.` (ex: "v1.3.2", "v1.2.0", "v1.0.0")
            if tg.tags.iter().any(|t| tag_patter.is_match(&t.name)) {
                tag_patter_counter += 1;
            } else {
                //Should keep the 2 latest versions that are not versions/realeas (ex: "37813", "19121", "random_name")
                unknown_patter_counter += 1;
            }

            //Should keep all images younger than 2 weeks
            if tg
                .created
                .gt(&Utc::now().checked_sub_days(Days::new(14)).unwrap())
            {
                continue;
            }

            if (tag_patter_counter > 3 && unknown_patter_counter >= 2) || (tag_patter_counter >= 3 && unknown_patter_counter > 2) {
                //now that we know the top number of the suggestion. The lower part will always be 0;
                suggestion = Some(format!("0..{}", i));
                break;
            }
        }

        match suggestion {
            Some(suggestion) => println!("\nOur delete suggestion is: {}", suggestion),
            None => println!("\nThis repository SGTM. We suggest you to delete nothing."),
        }
    }
}
