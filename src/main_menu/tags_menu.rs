use crate::registry::ScanElement;
use tabled::{builder::Builder, Style};

pub struct TagsMenu {}
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