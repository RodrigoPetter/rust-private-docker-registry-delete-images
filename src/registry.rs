use self::client::{RegistryClient, Tag};
use std::collections::HashMap;

mod client;

pub struct RegistryScanner {
    client: RegistryClient,
}

pub struct ScanResult {
    pub repository: String,
    pub tags_grouped_by_digest: HashMap<String, Vec<Tag>>,
    pub size: usize,
    pub size_dedup_repo: usize,
    pub size_dedup_global: usize,
}

impl RegistryScanner {
    pub fn new() -> RegistryScanner {
        RegistryScanner {
            client: RegistryClient::new(),
        }
    }

    pub fn scan(&mut self) -> Vec<ScanResult> {
        vec![
            ScanResult {
                repository: "aaa".to_string(),
                tags_grouped_by_digest: HashMap::new(),
                size: 50000,
                size_dedup_repo: 50000,
                size_dedup_global: 50000,
            },
            ScanResult {
                repository: "bbbb".to_string(),
                tags_grouped_by_digest: HashMap::new(),
                size: 990000,
                size_dedup_repo: 990000,
                size_dedup_global: 990000,
            },
        ]
    }
}
