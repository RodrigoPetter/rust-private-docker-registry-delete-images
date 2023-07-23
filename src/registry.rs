use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

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

        let thread_pool = ThreadPoolBuilder::new().num_threads(0).build().unwrap();

        let spb = ScanProgressBar::new(catalog.len().try_into().unwrap());

        return thread_pool.install(|| {
            return catalog
                .par_iter()
                .map(|repo| {
                    let sub_bar = spb.add_bar(rayon::current_thread_index().unwrap());

                    let tags_grouped_by_digest =
                        self.client.get_tags_grouped_by_digest(&repo, &sub_bar);

                    spb.total_bar.inc(1);
                    spb.remove_bar(sub_bar);

                    return ScanElement {
                        repository: repo.clone(),
                        tags_grouped_by_digest,
                    };
                })
                .collect::<Vec<_>>();
        });
    }
}
