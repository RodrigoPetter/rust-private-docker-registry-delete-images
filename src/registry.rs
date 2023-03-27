pub use self::client::{RegistryClient};
pub use self::client::TagGroup;

mod client;

pub struct RegistryScanner {
    client: RegistryClient,
}

pub struct ScanElement {
    pub repository: String,
    pub tags_grouped_by_digest: Vec<TagGroup>,
}

impl RegistryScanner {
    pub fn new() -> RegistryScanner {
        RegistryScanner {
            client: RegistryClient::new(),
        }
    }

    pub fn scan(&mut self) -> Vec<ScanElement> {
        let catalog = self.client.get_catalog();

         return catalog
                .into_iter()
                .map(|repo| {
                    //TODO: this line should be async with multi-thread to reduce scan time
                    let tags_grouped_by_digest = self.client.get_tags_grouped_by_digest(&repo);
                    return ScanElement {
                        repository: repo,
                        tags_grouped_by_digest
                    };
                })
            .collect::<Vec<_>>();
    }
}
