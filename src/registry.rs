use serde::Deserialize;

const BASE_URL: &str = "http://localhost:5000/v2/";

pub fn get_catalog() -> Vec<(u16, String)> {
    const CATALOG_PATH: &str = "_catalog";

    #[derive(Deserialize, Debug)]
    struct Catalog {
        repositories: Vec<String>,
    }

    let client = reqwest::blocking::Client::new();

    let resp: Catalog = client
        .get(format!("{}{}", BASE_URL, CATALOG_PATH))
        .send()
        .unwrap()
        .json()
        .unwrap();

    return resp
        .repositories
        .into_iter()
        .enumerate()
        .map(|(idx, repo)| ((idx + 1) as u16, repo))
        .collect();
}
