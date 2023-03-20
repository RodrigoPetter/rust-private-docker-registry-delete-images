use tabled::{builder::Builder, Style};

const INDEX_COLUMN_NAME: &str = " ";

pub struct Menu {
    pub header: Vec<String>,  //TODO: Remove this pub
    pub itens: Vec<MenuItem>, //TODO: Remove this pub
}

pub struct MenuItem {
    pub force_code: Option<u16>,
    pub values: Vec<String>,
}

impl Menu {
    pub fn new(header: Vec<String>, itens: Vec<MenuItem>) -> Menu {
        Menu {
            header: [INDEX_COLUMN_NAME.to_string()]
                .into_iter()
                .chain(header.into_iter())
                .collect::<Vec<_>>(),
            itens,
        }
    }
}

impl ToString for Menu {
    fn to_string(&self) -> String {
        let mut builder = Builder::default();

        builder.set_columns(&self.header);
        for (idx, element) in self.itens.iter().enumerate() {
            let code = element.force_code.unwrap_or(idx as u16).to_string();
            let row = [code]
                .into_iter()
                .chain(element.values.clone().into_iter())
                .collect::<Vec<_>>();
            builder.add_record(row);
        }

        return builder.build().with(Style::markdown()).to_string();
    }
}
