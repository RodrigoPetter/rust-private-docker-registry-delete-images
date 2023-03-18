use std::io::{self, Read};

/// This function is currently experiencing some issues with accurately tracking the space used by Docker images.
/// Specifically, it's only using the digest of the tag to calculate the size,
/// which can result in inaccuracies when there are multiple layers that share the same blob.
/// In order to properly calculate the total size, layers that have the same SHA256 hash should
/// only be counted once towards the total.
/// 
/// For example, many images use the Alpine Linux image as their base, so if this image is included
/// in multiple layers, its size should only be counted once towards the total size.
/// However, if each layer is counted separately, it could result in an overestimation of the total size.
pub fn run(client : &reqwest::blocking::Client, repos: &Vec<(u16, String)>) -> () {
    struct RepoSize {
        index: u16,
        name: String,
        size: f64,
        tag_count: usize,
    }

    let mut repo_sizes: Vec<RepoSize> = vec![];

    for (index, repo) in repos {
        //TODO: get real tada about the tags inside a repository from https://xxx.xx/v2/_REPOSITORY_/tags/list"
        let tags_list = vec!["v3.1.2", "v2.9.8", "v1.2.1", "v1.0.0", "v0.1.0"];

        let mut repo_size_accumulator: f64 = 0.0;

        //TODO: This Vec should be stored outside this loop. Diferent repositories can share the same digest/blob for the tag. I'm keeping it here just for comparison with the GO version. Fit it when the program is ready.
        // Vector used to store digests whose size has already been queried to prevent
        // duplicate queries and avoid double-counting the size of tags that share the same blob.
        let mut digest_tracker: Vec<String> = vec![];

        for tag in tags_list.iter() {
            //TODO: get real tag digest from https://xxx.xx/v2/_REPOSITORY_/manifests/_TAG_ -> NEEDS: "Accept: application/vnd.docker.distribution.manifest.v2+json"
            let tag_digest = format!(
                "{}{}",
                tag, "sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b"
            );

            if !digest_tracker.contains(&tag_digest) {
                println!("[{}] Fetching [{}] repository size...", index, repo);
                digest_tracker.push(tag_digest);

                //TODO: get real tag size from https://xxx.xx/v2/_REPOSITORY_/manifests/_TAG_ -> same url than before, but without the Accept header
                repo_size_accumulator += 1.45354;
            }
        }

        repo_sizes.push(RepoSize {
            index: index.clone(),
            name: repo.clone(),
            size: repo_size_accumulator,
            tag_count: tags_list.len(),
        });
    }

    repo_sizes.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());

    println!("Approximate size used by the compressed images in the registry (note: this size does not take into account that the same layer can be shared between multiple Docker images):");

    let mut total : f64 = 0.0;
    for element in repo_sizes.into_iter() {
        println!("{:>10.2} MB - {:^3} tags - {} ({})", element.size, element.tag_count, element.name, element.index);
        total += element.size;
    }

    println!("Total: {:>7.3}GB", total/1024.0);
    println!("Press enter to continue...");
    io::stdin().read(&mut [0u8]).expect("Failed to read input");
    
    return ();
}