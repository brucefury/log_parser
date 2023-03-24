use clap::ArgMatches;
use std::path::PathBuf;
use chrono::prelude::*;

pub struct Config {
    pub exclude_trace: bool,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub directory_mode: Option<PathBuf>,
    pub current_directory_mode: bool,
    pub similarity_threshold: f64,
    pub search: Option<String>,
    pub use_filter_file: bool,
    pub file_path: Option<PathBuf>,
    pub filter_file_path: Option<PathBuf>,
    pub count_mode: bool,
}

impl Config {
    pub fn from_matches(matches: ArgMatches) -> Config {
        let exclude_trace = matches.is_present("exclude_trace");

        let start_date = matches
            .value_of("start_date")
            .map(|start| {
                DateTime::parse_from_rfc3339(start)
                    .expect("Failed to parse start date")
                    .timestamp()
            });

        let end_date = matches
            .value_of("end_date")
            .map(|end| {
                DateTime::parse_from_rfc3339(end)
                    .expect("Failed to parse end date")
                    .timestamp()
            });

        let directory_mode = matches
            .value_of("directory")
            .map(|s| PathBuf::from(s));

        let current_directory_mode = matches.is_present("current_directory");

        let similarity_threshold = matches
            .value_of("similarity")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.8);

        let search = matches
            .value_of("search")
            .map(|s| s.to_string());

        let use_filter_file = matches.is_present("use_filter_file");

        let file_path = matches
            .value_of("file_path")
            .map(|s| PathBuf::from(s));

        let filter_file_path = matches
            .value_of("filter_file_path")
            .map(|s| PathBuf::from(s));

        let count_mode = matches.is_present("count_mode");

        Config {
            exclude_trace,
            start_date,
            end_date,
            directory_mode,
            current_directory_mode,
            similarity_threshold: Some(similarity_threshold),
            search,
            use_filter_file,
            file_path,
            filter_file_path,
            count_mode,
        }
    }
}
