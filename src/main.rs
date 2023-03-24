mod config;
mod processor;
mod filters;

use clap::{App, Arg};
use config::Config;
use processor::process_files;

fn main() {
    let matches = App::new("Log Parser")
        .version("0.1.0")
        .help("Parses log files with various filtering options")
        .arg(
            Arg::new("exclude_trace")
                .short('t')
                .long("exclude-trace")
                .help("Exclude stack traces from the output"),
        )
        .arg(
            Arg::new("start_date")
                .long("start")
                .value_name("DATE")
                .help("Start date in ISO 8601 format")
                .takes_value(true),
        )
        .arg(
            Arg::new("end_date")
                .long("end")
                .value_name("DATE")
                .help("End date in ISO 8601 format")
                .takes_value(true),
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .value_name("PATH")
                .help("Includes all .gz and .log files in the specified directory")
                .takes_value(true),
        )
        .arg(
            Arg::new("current_directory")
                .short('c')
                .long("current-directory")
                .help("Includes all .gz and .log files in the current directory"),
        )
        .arg(
            Arg::new("similarity")
                .short('s')
                .long("similarity")
                .value_name("SCORE")
                .help("Similarity score for grouping similar messages in count mode")
                .takes_value(true),
        )
        .arg(
            Arg::new("search")
                .short('S')
                .long("search")
                .value_name("STRING")
                .help("Searches for a specific string in the log messages")
                .takes_value(true),
        )
        .arg(
            Arg::new("use_filter_file")
                .short('f')
                .long("use-filter-file")
                .help("Filter out common spam messages using a JSON filter file"),
        )
        .arg(
            Arg::new("filter_file_path")
                .long("filter-file-path")
                .help("Path to filter file - must be a JSON file"),
        )
        .arg(
            Arg::new("file_path")
                .short('p')
                .long("file-path")
                .value_name("PATH")
                .help("Path to a single log file to process")
                .takes_value(true),
        )
        .arg(
            Arg::new("count_mode")
                .short('C')
                .long("count-mode")
                .help("Count messages, with the --similarity option for grouping similar messages"),
        )
        .get_matches();

    let config = Config::from_matches(matches);

    process_files(config);
}
