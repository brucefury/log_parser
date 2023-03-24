mod config;
mod processor;
mod filters;

use clap::{App, Arg};
use config::Config;
use processor::process_files;
use std::time::Instant;

fn main() {
    let matches = App::new("Log Parser")
        .version("0.1.0")
        .help("
            Parses log files with various filtering options.\n
            Commands:
                --t Exclude Trace - Will not include stack traces in results.
                --start Start Date - Start date in ISO ISO8602.
                --end End Date - Start date in ISO ISO8602.
                --d <value> Directory Mode - Includes all .gz or log file in directory.
                --c Current Directory Mode - Includes all .gz or log file in current directory.
                --s <value> Similarity Score - If in count mode group similar messages.  Not for use on large unfiltered files.
                --S <value> Search String - Search value - does simple text search.
                --uff Use filter file.
                --ffp Filter file path.
                --p File Path - To run on a single file.
                --C Count Mode - Count messages, with the --sim we can group similar messages(significantly slower).
                --h Show Help - Show help file.\n
                
            Examples:
                'log_parser --exclude-trace -C -s 0.8 -p ./system.log.4.gz' - Count all logs with a similarity score of 0.8 or greater, given the file path, excluding trace.
                'log_parser -d ../logs --exclude-trace -C -s 0.8' - Count all logs with a similarity score of 0.8 or greater, given the directory, excluding trace.
                'log_parser -d ../logs --start 2023-02-19T02:00:00+00:00 --end 2023-02-19T03:00:00+00:00 --t' - Output all logs in date range, given the directory, excluding trace.
                'log_parser --uff --ffp ./filters.json -p ../system.log.4.gz -t' - Use filter file to get rid of common spam.
            ")
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
                .long("uff")
                .help("Filter out common spam messages using a JSON filter file"),
        )
        .arg(
            Arg::new("filter_file_path")
                .long("ffp")
                .help("Path to filter file - must be a JSON file")
                .takes_value(true),
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
        ).get_matches();

    let config = Config::from_matches(matches);
    let start_time = Instant::now(); 

    process_files(config);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:.5} seconds", elapsed_time.as_secs_f64());
}
