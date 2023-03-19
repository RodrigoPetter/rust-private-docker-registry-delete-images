use std::io::{self, Read};

use super::Layer;
use super::RegistryClient;

pub fn run(registry_client: &RegistryClient, repos: &Vec<(u16, String)>) -> () {
    struct RepoDetails {
        index: u16,
        name: String,
        tags: Vec<(String, Vec<Layer>)>,
    }

    let mut repo_details: Vec<RepoDetails> = vec![];

    //TODO: make this loop async with multi-thread to reduce scan time
    for (index, repo) in repos {
        let tags_list = registry_client.get_tags(&repo);

        let mut repo_size = RepoDetails {
            index: index.clone(),
            name: repo.clone(),
            tags: vec![],
        };

        for tag in tags_list.into_iter() {
            println!("[{}] Fetching [{}] repository size...", index, repo);
            let layers = registry_client.get_manifest_v2(&repo, &tag);
            repo_size.tags.push((tag, layers))
        }

        repo_details.push(repo_size);
    }

    struct SizeDisplay {
        index: u16,
        name: String,
        tag_count: usize,
        size: f64,
        size_dedup_repo: f64,
        size_dedup_global: f64,
    }

    let mut display: Vec<SizeDisplay> = vec![];

    // When multiple images share the same base layer (such as Alpine Linux), we want to avoid
    // summing the layer size multiple times. However, we must sum it at least once. Therefore,
    // we aggregate the layer value for the first image that we check. This may result in a
    // misleading display size, such as displaying zero for the global deduplication if all
    // layers are shared between two distinct repositories.
    let mut global_digest_tracker: Vec<String> = vec![];

    for details in repo_details.iter() {
        let mut repo_display = SizeDisplay {
            index: details.index,
            name: details.name.clone(),
            tag_count: details.tags.len(),
            size: 0.0,
            size_dedup_repo: 0.0,
            size_dedup_global: 0.0,
        };

        let mut repo_disgest_tracker: Vec<String> = vec![];

        for (_, layers) in details.tags.iter() {
            for layer in layers.into_iter() {
                let size = byte_to_mega(&layer.size);
                repo_display.size += size;

                if !repo_disgest_tracker.contains(&layer.digest) {
                    repo_disgest_tracker.push(layer.digest.clone());
                    repo_display.size_dedup_repo += size;
                }

                if !global_digest_tracker.contains(&layer.digest) {
                    global_digest_tracker.push(layer.digest.clone());
                    repo_display.size_dedup_global += size;
                }
            }
        }

        display.push(repo_display);
    }

    display.sort_by(|a, b| b.size_dedup_global.partial_cmp(&a.size_dedup_global).unwrap());

    println!("\nApproximate size used by the compressed images in the registry:\n");

    let mut total: f64 = 0.0;
    let mut total_dedup: f64 = 0.0;

    print_row(
        "idx",
        "Global Dedup Size",
        "Repo Dedup Size",
        "Total Size",
        "Tag Count",
        "Repository",
    );

    for element in display.into_iter() {
        print_row(
            &element.index.to_string(),
            &format_size(&element.size_dedup_global),
            &format_size(&element.size_dedup_repo),            
            &format_size(&element.size),
            &element.tag_count.to_string(),
            &element.name,
        );

        total += element.size;
        total_dedup += element.size_dedup_global;
    }

    println!("\nTotal: {:>7.3}GB", mega_to_giga(&total));
    println!("Total Dedup: {:>7.3}GB\n", mega_to_giga(&total_dedup));
    println!("Press enter to continue...");
    io::stdin().read(&mut [0u8]).expect("Failed to read input");

    return ();
}

fn byte_to_mega(bytes: &usize) -> f64 {
    return (bytes.clone() as f64 / 1024.0) / 1024.0;
}

fn mega_to_giga(megas: &f64) -> f64 {
    return megas / 1024.0;
}

fn print_row(
    column0: &str,
    column1: &str,
    column2: &str,
    column3: &str,
    column4: &str,
    column5: &str,
) {
    println!(
        "{0:<4} | {1:^17} | {2:^15} | {3:^11} | {4:^9} | {5:}",
        column0, column1, column2, column3, column4, column5
    );
}

fn format_size(size: &f64) -> String {
    if size.clone() < 1000.0 {
        return format!("{:<7.2}MB", size);
    } else {
        return format!("{:<7.2}GB", mega_to_giga(size));
    }
}
