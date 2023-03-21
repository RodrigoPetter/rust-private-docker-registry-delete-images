use tabled::{builder::Builder, Style};

use crate::registry::ScanResult;

pub struct MainMenu {}

impl MainMenu {
    pub fn print(itens: &Vec<ScanResult>) {
        let mut builder = Builder::default();

        builder.set_columns(vec![
            "repository",
            "tags",
            "disk usage",
            "total layers size",
        ]);

        //TODO: Sort before printing
        for element in itens.iter() {
            builder.add_record(vec![
                element.repository.clone(),
                element.tags_grouped_by_digest.len().to_string(),
                format_size(&element.size_dedup_global),
                format_size(&element.size),
            ]);
        }

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
