use std::collections::HashMap;

use serde::Deserialize;

const BASE_URL: &str = "http://localhost:5000/v2/";

pub struct RegistryClient {
    http_client: reqwest::blocking::Client,
}

pub struct Tag {
    pub name: String,
    pub manifest: Manifest,
}
pub struct Manifest {
    pub digest: String,
    pub layers: Vec<Layer>,
}
#[derive(Deserialize)]
pub struct Layer {
    pub size: usize,
    pub digest: String,
}

impl RegistryClient {
    pub fn new() -> RegistryClient {
        RegistryClient {
            http_client: (reqwest::blocking::Client::new()),
        }
    }

    pub fn get_catalog(&self) -> Vec<String> {
        const CATALOG_PATH: &str = "_catalog";

        #[derive(Deserialize)]
        struct Catalog {
            repositories: Vec<String>,
        }

        let resp: Catalog = self
            .http_client
            .get(format!("{}{}", BASE_URL, CATALOG_PATH))
            .send()
            .expect(&format!("Unable to fetch the catalog. Check that the registry address [{}] is correct and that it is running.", BASE_URL))
            .json()
            .unwrap();

        return resp.repositories;
    }

    pub fn get_tags_grouped_by_digest(&self, repo_name: &str) -> HashMap<String, Vec<Tag>> {
        const TAGS_PATH: &str = "/tags/list";

        #[derive(Deserialize)]
        struct Tags {
            tags: Vec<String>,
        }

        let resp: Tags = self
            .http_client
            .get(format!("{}{}{}", BASE_URL, repo_name, TAGS_PATH))
            .send()
            .unwrap()
            .json()
            .unwrap();

        let tags: Vec<Tag> = resp
            .tags
            .into_iter()
            .map(|tag_name| Tag {
                manifest: self.get_manifest_v2(repo_name, &tag_name),
                name: tag_name,
            })
            .collect();

        let mut tags_group_by_digest: HashMap<String, Vec<Tag>> = HashMap::new();
        for tag in tags.into_iter() {
            tags_group_by_digest
                .entry(tag.manifest.digest.clone())
                .or_insert(Vec::new())
                .push(tag);
        }

        return tags_group_by_digest;
    }

    pub fn get_created(&self, repo_name: &str, tag: &Tag) -> String {
        const MANIFEST_PATH: &str = "/manifests/";

        #[derive(Deserialize)]
        struct ManifestV1 {
            history: Vec<ManifestHistoryV1>,
        }
        #[derive(Deserialize)]
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        struct ManifestHistoryV1 {
            v1Compatibility: String,
        }
        #[derive(Deserialize)]
        #[allow(non_camel_case_types)]
        struct v1Compatibility {
            created: String,
        }

        let resp: ManifestV1 = self
        .http_client
        .get(format!("{}{}{}{}",BASE_URL, repo_name, MANIFEST_PATH, tag.name))
        .send()
        .expect(&format!("Unable to fetch the catalog. Check that the registry address [{}] is correct and that it is running.", BASE_URL))
        .json().unwrap();

        let v1comp: v1Compatibility =
            serde_json::from_str(&resp.history.first().unwrap().v1Compatibility).unwrap();
        return v1comp
            .created
            .split(".")
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .to_string();
    }

    fn get_manifest_v2(&self, repo_name: &str, tag_name: &str) -> Manifest {
        const MANIFEST_PATH: &str = "/manifests/";
        const MANIFEST_V2_HEADER: &str = "application/vnd.docker.distribution.manifest.v2+json";

        println!(
            "Fetching [{}] repository manifest for [{}]...",
            repo_name, tag_name
        );
        let resp = self
            .http_client
            .get(format!(
                "{}{}{}{}",
                BASE_URL, repo_name, MANIFEST_PATH, tag_name
            ))
            .header("Accept", MANIFEST_V2_HEADER)
            .send()
            .unwrap();

        let tag_digest = resp
            .headers()
            .get("Docker-Content-Digest")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        #[derive(Deserialize)]
        struct ManifestApiResponse {
            pub layers: Vec<Layer>,
        }

        let json: ManifestApiResponse = resp.json().unwrap();

        return Manifest {
            digest: tag_digest,
            layers: json.layers,
        };
    }
}
