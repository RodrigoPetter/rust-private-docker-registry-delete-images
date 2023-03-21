use std::{io::{self, Read}, collections::HashMap};

use super::{RegistryClient, Tag};
use crate::formats::{byte_to_mega, format_size};

pub fn run(registry_client: &RegistryClient, repos: &Vec<(u16, String)>) -> () {
    struct RepoDetails {
        index: u16,
        name: String,
        tags: HashMap<String, Vec<Tag>>,
    }

    let mut repo_details: Vec<RepoDetails> = vec![];
    //TODO: make this loop async with multi-thread to reduce scan time
    for (index, repo) in repos {
        repo_details.push(RepoDetails {
            index: index.clone(),
            name: repo.clone(),
            tags: registry_client.get_tags(&repo),
        });
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

    for details in repo_details.into_iter() {
        let mut repo_display = SizeDisplay {
            index: details.index,
            name: details.name.clone(),
            tag_count: details.tags.len(),
            size: 0.0,
            size_dedup_repo: 0.0,
            size_dedup_global: 0.0,
        };

        for (_, tags) in details.tags.into_iter() {

            for (idx, tag) in tags.into_iter().enumerate() {

                for layer in tag.manifest.layers.into_iter() {
                    let size = byte_to_mega(&layer.size);
                    repo_display.size += size;

                    // Sums the value only if it is the first element of the tags grouped by
                    // digest, thus deduplicating between the layers in the same repository.
                    if idx == 0 {
                        repo_display.size_dedup_repo += size;
                    }
    
                    if !global_digest_tracker.contains(&layer.digest) {
                        global_digest_tracker.push(layer.digest.clone());
                        repo_display.size_dedup_global += size;
                    }
                }

            }            
        }

        display.push(repo_display);
    }

    display.sort_by(|a, b| {
        b.size_dedup_global
            .partial_cmp(&a.size_dedup_global)
            .unwrap()
    });

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

    println!("\nTotal Dedup: {}", format_size(&total_dedup));
    println!("Total: {:>15}\n", format_size(&total));
    println!("Press enter to continue...");
    io::stdin().read(&mut [0u8]).expect("Failed to read input");

    return ();
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