use crate::config::Config;
use crate::filters::Filters;

use chrono::prelude::*;
use flate2::bufread::GzDecoder;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use strsim::normalized_levenshtein;

pub fn process_files(config: Config) {
    if config.directory_mode.is_some() || config.current_directory_mode {
        let dir = if config.current_directory_mode {
            std::env::current_dir().unwrap()
        } else {
            config.directory_mode.as_ref().unwrap().to_path_buf()
        };

        for entry in dir.read_dir().expect("Failed to read directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                let ext = path.extension().and_then(|os_str| os_str.to_str()).unwrap_or("");
                if ext == "gz" || ext == "log" {
                    process_file(&config, &path);
                }
            }
        }
    } else {
        process_file(&config, config.file_path.as_ref().unwrap());
    }
}

pub fn process_file(config: &Config, path: &Path) {
    println!("---------------------------------------------------------");
    println!("Processing: {:?}", path);

    let filters = if config.use_filter_file {
        let filter_file_path = config.filter_file_path.as_ref().expect("Filter file path is missing");
        Some(Filters::from_file(filter_file_path).expect("Failed to load filters from file"))
    } else {
        None
    };

    let process_result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(process_lines(config, path, filters.as_ref()));

    if let Err(e) = process_result {
        eprintln!("Error processing file {:?}: {}", path, e);
    }
}

pub async fn process_lines(
    config: &Config,
    path: &Path,
    filters: Option<&Filters>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path).expect("Failed to open file");
    let reader: Box<dyn BufRead> = if path.extension().and_then(|os_str| os_str.to_str()) == Some("gz") {
        Box::new(BufReader::new(GzDecoder::new(BufReader::new(file))))
    } else {
        Box::new(BufReader::new(file))
    };

    let date_re = Regex::new(r"\[(.*?)\]").unwrap();
    let mut in_trace = false;
    let mut messages: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut similarity_scores: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            continue;
        }

        let date_match = date_re.captures(&line);
        let timestamp = date_match.as_ref().and_then(|cap| cap.get(1)).map(|m| m.as_str());
        let time = timestamp.and_then(|ts| DateTime::parse_from_rfc3339(ts).ok().map(|dt| dt.timestamp()));

        if let Some(filters) = filters {
            if filters.is_filtered(&line) {
                continue;
            }
        }

        if let Some(start) = config.start_date {
            if let Some(time) = time {
                if time < start {
                    continue;
                }
            }
        }

        if let Some(end) = config.end_date {
            if let Some(time) = time {
                if time > end {
                    continue;
                }
            }
        }

        if let Some(ref search) = config.search {
            if !line.contains(search) {
                continue;
            }
        }

        if config.exclude_trace {
            if line.contains("Stack trace:") || line.starts_with("Next") {
                in_trace = true;
            }
        }

        if !in_trace && !config.count_mode {
            println!("{}", line);
        } else if !in_trace && config.count_mode {
            let msg = date_re.replace_all(&line, "").to_string();

            if let Some(sim_arg) = config.similarity_threshold {
                let mut best_match = None;
                let mut best_score = 0.0;

                for (key, _) in &messages {
                    let score = normalized_levenshtein(key, &msg);
                    if score > sim_arg && score > best_score {
                        best_match = Some(key.clone());
                        best_score = score;
                    }
                }

                if let Some(key) = best_match {
                    *messages.get_mut(&key).unwrap() += 1;
                    similarity_scores.insert(key, best_score);
                } else {
                    messages.insert(msg.clone(), 1);
                }
            } else {
                *messages.entry(msg).or_insert(0) += 1;
            }
        }

        if config.exclude_trace && line.contains("{main}") {
            in_trace = false;
        }
    }

    if config.count_mode {
        let mut message_vec: Vec<(&String, &usize)> = messages.iter().collect();
        message_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (key, count) in message_vec {
            let sim_score = similarity_scores.get(key).unwrap_or(&1.0);
            println!("{} (Count: {}, Minimum Similarity: {:.3})", key, count, sim_score);
        }
    }

    println!("END OF {:?}", path);
    println!("---------------------------------------------------------");

    Ok(())
}
