use std::{collections::HashMap, env};

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;

use super::progress_bar::ProgressBarWrapper;

pub struct RegistryClient {
    http_client: reqwest::blocking::Client,
    url: String,
}

pub struct Tag {
    pub name: String,
    pub manifest: Manifest,
}
pub struct TagGroup {
    pub digest: String,
    pub created: DateTime<Utc>,
    pub tags: Vec<Tag>,
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
            url: match env::var("REGISTRY_URL") {
                Ok(url) => url + "/v2/",
                Err(_) => panic!(
                    "Registry URL missing. Please set the value using the env var `REGISTRY_URL`."
                ),
            },
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
            .get(format!("{}{}", self.url, CATALOG_PATH))
            .send()
            .expect(&format!("Unable to fetch the catalog. Check that the registry address [{}] is correct and that it is running.", self.url))
            .json()
            .unwrap();

        return resp.repositories;
    }

    pub fn get_tags_grouped_by_digest(
        &self,
        repo_name: &str,
        bar: &ProgressBarWrapper,
    ) -> Vec<TagGroup> {
        const TAGS_PATH: &str = "/tags/list";

        bar.set_message(format!("Fetchin [/tags/list] for {}", repo_name));

        #[derive(Deserialize)]
        struct Tags {
            tags: Vec<String>,
        }

        let resp: Tags = self
            .http_client
            .get(format!("{}{}{}", self.url, repo_name, TAGS_PATH))
            .send()
            .unwrap()
            .json()
            .unwrap();

        bar.set_length(resp.tags.len().try_into().unwrap());
        bar.set_message(format!("Found {} tags for {}", resp.tags.len(), repo_name));

        let tags: Vec<Tag> = resp
            .tags
            .into_iter()
            .map(|tag_name| {
                bar.set_message(format!(
                    "Fetchin manifest of [{}] for {}",
                    tag_name, repo_name
                ));

                let tag = Tag {
                    manifest: self.get_manifest_v2(repo_name, &tag_name),
                    name: tag_name,
                };
                bar.inc(1);
                return tag;
            })
            .collect();

        let mut tags_group_by_digest: HashMap<String, Vec<Tag>> = HashMap::new();
        for tag in tags.into_iter() {
            tags_group_by_digest
                .entry(tag.manifest.digest.clone())
                .or_insert(Vec::new())
                .push(tag);
        }

        let mut tags_group_by_digest = tags_group_by_digest
            .into_iter()
            .map(|(digest, tags)| TagGroup {
                created: self.get_created(repo_name, tags.first().unwrap()),
                digest,
                tags,
            })
            .collect::<Vec<_>>();

        tags_group_by_digest.sort_by(|a, b| a.created.partial_cmp(&b.created).unwrap());
        return tags_group_by_digest;
    }

    pub fn get_created(&self, repo_name: &str, tag: &Tag) -> DateTime<Utc> {
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
        .get(format!("{}{}{}{}",self.url, repo_name, MANIFEST_PATH, tag.name))
        .send()
        .expect(&format!("Unable to fetch the catalog. Check that the registry address [{}] is correct and that it is running.", self.url))
        .json().unwrap();

        let v1comp: v1Compatibility =
            serde_json::from_str(&resp.history.first().unwrap().v1Compatibility).unwrap();

        let date_string: String = v1comp
            .created
            .split(".")
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .to_string();

        return Utc
            .datetime_from_str(&date_string, "%Y-%m-%dT%H:%M:%S")
            .unwrap();
    }

    pub fn delete_digest(&self, repo: &String, digest: &String) -> () {
        let url = format!("{}{}/manifests/{}", self.url, repo, digest);

        println!("DELETE: {url}");

        let result = self.http_client.delete(url).send().unwrap();

        if !result.status().is_success() {
            println!(
                "Status code different from 2xx when deleting the digest {} -> {}",
                digest,
                result.status().as_str()
            );
            panic!("{}", result.text().unwrap());
        }
    }

    fn get_manifest_v2(&self, repo_name: &str, tag_name: &str) -> Manifest {
        const MANIFEST_PATH: &str = "/manifests/";
        const MANIFEST_V2_HEADER: &str = "application/vnd.docker.distribution.manifest.v2+json";

        let resp = self
            .http_client
            .get(format!(
                "{}{}{}{}",
                self.url, repo_name, MANIFEST_PATH, tag_name
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
