use std::process::Command;

use crate::search::models::SearchResult;

pub fn search(query: &str) -> Vec<SearchResult> {
    let search = format!("ytsearch5:{query}");

    let output = Command::new("yt-dlp")
        .args([
            &search,
            "--print",
            "%(title)s|%(webpage_url)s",
            "--no-update",
        ])
        .output();

    match output {
        Ok(out) => {
            let text = String::from_utf8_lossy(&out.stdout);

            text.lines()
                .filter_map(|line| {
                    let (title, url) = line.split_once('|')?;

                    Some(SearchResult {
                        title: title.to_string(),
                        url: url.to_string(),
                    })
                })
                .collect()
        }

        Err(_) => Vec::new(),
    }
}
