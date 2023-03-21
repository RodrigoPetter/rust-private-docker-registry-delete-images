use self::client::{RegistryClient, TagGroup};

mod client;

pub struct RegistryScanner {
    client: RegistryClient,
}

pub struct ScanResult {
    pub total_size: usize,
    pub total_dedup_size: usize,
    pub elements: Vec<ScanElement>,
}

pub struct ScanElement {
    pub repository: String,
    pub tags_grouped_by_digest: Vec<TagGroup>,
    pub size: usize,
    pub size_dedup_global: usize,
}

impl RegistryScanner {
    pub fn new() -> RegistryScanner {
        RegistryScanner {
            client: RegistryClient::new(),
        }
    }

    pub fn scan(&mut self) -> ScanResult {
        let catalog = self.client.get_catalog();

        let mut total: usize = 0;
        let mut total_dedup: usize = 0;
        // When multiple images share the same base layer (such as Alpine Linux), we want to avoid
        // summing the layer size multiple times. However, we must sum it at least once. Therefore,
        // we aggregate the layer value for the first image that we check. This may result in a
        // misleading display size, such as displaying zero for the global deduplication if all
        // layers are shared between two distinct repositories.
        let mut global_digest_tracker: Vec<String> = vec![];

        let mut result = ScanResult {
            elements: catalog
                .into_iter()
                .map(|repo| {
                    //TODO: this line should be async with multi-thread to reduce scan time
                    let tags_grouped_by_digest = self.client.get_tags_grouped_by_digest(&repo);

                    let mut size: usize = 0;
                    let mut size_dedup_global: usize = 0;
                    for tag_group in tags_grouped_by_digest.iter() {
                        for tag in tag_group.tags.iter() {
                            for layer in tag.manifest.layers.iter() {
                                size += layer.size;
                                total += layer.size;
                                if !global_digest_tracker.contains(&layer.digest) {
                                    global_digest_tracker.push(layer.digest.clone());
                                    size_dedup_global += layer.size;
                                    total_dedup += layer.size;
                                }
                            }
                        }
                    }

                    return ScanElement {
                        repository: repo,
                        size,
                        size_dedup_global,
                        tags_grouped_by_digest,
                    };
                })
                .collect::<Vec<_>>(),
            total_size: total,
            total_dedup_size: total_dedup,
        };

        result.elements.sort_by(|a, b| {
            b.size_dedup_global
                .partial_cmp(&a.size_dedup_global)
                .unwrap()
        });

        return result;
    }
}
