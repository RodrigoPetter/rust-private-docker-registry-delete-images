pub use self::client::RegistryClient;
pub use self::client::TagGroup;
use self::progress_bar::ScanProgressBar;

mod client;
mod progress_bar;

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

        let spb = ScanProgressBar::new(catalog.len().try_into().unwrap());

        return catalog
            .into_iter()
            .map(|repo| {

                spb.total_bar.set_message(format!("Catalog: {}", repo));

                //TODO: this line should be async with multi-thread to reduce scan time
                let tags_grouped_by_digest = self.client.get_tags_grouped_by_digest(&repo);

                spb.total_bar.inc(1);

                return ScanElement {
                    repository: repo,
                    tags_grouped_by_digest,
                };
            })
            .collect::<Vec<_>>();
    }
}
