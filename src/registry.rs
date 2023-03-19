use serde::Deserialize;

mod scan;

const BASE_URL: &str = "http://localhost:5000/v2/";

pub struct RegistryClient {
    http_client: reqwest::blocking::Client,
}

#[derive(Deserialize)]
pub struct Layer {
    size: usize,
    digest: String,
}
#[derive(Deserialize)]
pub struct Manifest {
    digest: String,
    layers: Vec<Layer>,
}

impl RegistryClient {
    pub fn new() -> RegistryClient {
        RegistryClient {
            http_client: (reqwest::blocking::Client::new()),
        }
    }

    pub fn get_catalog(&self) -> Vec<(u16, String)> {
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

        return resp
            .repositories
            .into_iter()
            .enumerate()
            .map(|(idx, repo)| ((idx + 1) as u16, repo))
            .collect();
    }

    pub fn scan(&self, repos: &Vec<(u16, String)>) -> () {
        return scan::run(&self, repos);
    }

    pub fn get_tags(&self, repo_name: &str) -> Vec<String> {
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

        return resp.tags;
    }

    pub fn get_manifest_v2(&self, repo_name: &str, tag_name: &str) -> Manifest {
        const MANIFEST_PATH: &str = "/manifests/";
        const MANIFEST_V2_HEADER: &str = "application/vnd.docker.distribution.manifest.v2+json";

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
        
        let json: Manifest = resp.json().unwrap();

        return Manifest {
            digest: tag_digest,
            layers: json.layers,
        };
    }
}
