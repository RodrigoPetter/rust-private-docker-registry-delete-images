use std::collections::HashMap;

use crate::registry::ScanElement;

pub struct Sizes {
    pub size: usize,
    pub size_dedup_global: usize,
}

pub fn calculate(elements: &Vec<ScanElement>) -> HashMap<String, Sizes> {
    let mut result = HashMap::with_capacity(elements.len() + 1);
    let mut total: usize = 0;
    let mut total_dedup: usize = 0;

    // When multiple images share the same base layer (such as Alpine Linux), we want to avoid
    // summing the layer size multiple times. However, we must sum it at least once. Therefore,
    // we aggregate the layer value for the first image that we check. This may result in a
    // misleading display size, such as displaying zero for the global deduplication if all
    // layers are shared between two distinct repositories.
    let mut global_digest_tracker: Vec<String> = vec![];
    for element in elements.iter() {
        let mut size: usize = 0;
        let mut size_dedup_global: usize = 0;
        for tag_group in element.tags_grouped_by_digest.iter() {
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
        result.insert(
            element.repository.clone(),
            Sizes {
                size,
                size_dedup_global,
            },
        );
    }

    result.insert(
        "TOTALS".to_string(),
        Sizes {
            size: total,
            size_dedup_global: total_dedup,
        },
    );

    return result;
}

//TODO: sorting function
// result.elements.sort_by(|a, b| {
//     b.size_dedup_global
//         .partial_cmp(&a.size_dedup_global)
//         .unwrap()
// });
