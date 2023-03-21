use crate::{registry::{ScanElement, TagGroup}, std_in_out::read_input};
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
            let selected = TagsMenu::select(&repository);
            //TODO: Selected should be a Range<>
            match selected {
                Some(_) => print!("TODO: implement delete"),
                None => todo!(),
            }                    
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

        //TODO: Print return to main menu
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

    fn select(repository: &ScanElement) -> Option<&TagGroup> {
        loop {
            let selected = read_input::<usize>("Select a tag for deletion:");

            match selected {
                selected if selected < repository.tags_grouped_by_digest.len() => {
                    return Some(repository.tags_grouped_by_digest.get(selected).unwrap())
                }
                _ => {
                    println!("Not a valid option.");
                    continue;
                }
            }
        }
    }
}
