use serde::Deserialize;

mod scan;

const BASE_URL: &str = "http://localhost:5000/v2/";

pub struct RegistryClient {
    http_client: reqwest::blocking::Client,
}

impl RegistryClient {
    pub fn new() -> RegistryClient {
        RegistryClient {
            http_client: (reqwest::blocking::Client::new()),
        }
    }

    pub fn get_catalog(&self) -> Vec<(u16, String)> {
        const CATALOG_PATH: &str = "_catalog";

        #[derive(Deserialize, Debug)]
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

        #[derive(Deserialize, Debug)]
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
}
