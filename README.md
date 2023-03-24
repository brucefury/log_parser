# Log Parser

A command-line tool for parsing log files with various filtering options. It supports `.log` and `.gz` files and offers options for filtering based on date, search term, similarity score, and more.

### Features
- Exclude stack traces from the output
- Filter logs based on start and end dates (in ISO 8601 format)
- Process all `.gz` and `.log` files within a specified directory or the current directory
- Filter logs based on a similarity score for grouping similar messages in count mode
- Search for a specific string in log messages
- Filter out common spam messages using a JSON filter file
- Process a single log file

### Options
- `--start DATE`: Start date in ISO 8601 format (e.g., `2021-09-01T00:00:00+00:00`)
- `--end DATE`: End date in ISO 8601 format (e.g., `2021-09-30T23:59:59+00:00`)
- `-d`, `--directory PATH`: Include all `.gz` and `.log` files in the specified directory
- `-c`, `--current-directory`: Include all `.gz` and `.log` files in the current directory
- `-p`, `--file-path PATH`: Path to a single log file to process
- `-C`, `--count-mode`: Count messages, with the `--similarity` option for grouping similar messages, the default similarity is 0.8
- `-s`, `--similarity SCORE`: Similarity score for grouping similar messages in count mode (0.0 to 1.0, default: 0.8)
- `-S`, `--search STRING`: Search for a specific string in the log messages
- `-t`, `--exclude-trace`: Exclude stack traces from the output
- `--uff`: Use filter file
- `--ffp`: Path to the filter file (must be a JSON file)

### Examples

- Count all logs with a similarity score of 0.8 or greater, given the file path, excluding trace.
    
    ```log_parser --exclude-trace -C -s 0.8 -p ./system.log.4.gz``` 
- Count all logs with a similarity score of 0.8 or greater, given the directory, excluding trace.
    
    ```log_parser -d ../logs --exclude-trace -C -s 0.8```
- Output all logs in date range, given the directory, excluding trace.
    
    ```log_parser -d ../logs --start 2023-02-19T02:00:00+00:00 --end 2023-02-19T03:00:00+00:00 --t```
- Use filter file to get rid of common spam.
    
    ```log_parser --uff --ffp ./filters.json -p ../system.log.4.gz -t```

### Notes
- The filters.json file should be formatted as a JSON object containing an array of strings under the key "filters". Each string in the array represents a keyword or phrase used for filtering log messages. For example:
```json
{
  "filters": [
    "Spam Log 1",
    "Spam Log 2"
  ]
}
```

- The similarity score when counting messages is calculated using the Normalized Levenshtein distance, which is a string metric for measuring the difference between two strings.  The Levenshtein distance between two strings is the minimum number of single-character edits (insertions, deletions, or substitutions) required to transform one string into the other. The Normalized Levenshtein distance is the Levenshtein distance divided by the maximum possible distance, which is the length of the longer string. This normalization results in a score ranging from 0 to 1, where 1 represents identical strings and 0 represents completely dissimilar strings.

- When counting messages in large files, it is essential to optimize the process, as comparing each message to every other message meeting a specific similarity criteria can be time-consuming. To enhance performance, it is recommended to:
    - Apply as many filters as possible to eliminate irrelevant messages.
    - Choose a specific search term to narrow down the scope of messages.
    - Define a precise date range to limit the number of messages being processed.
    - Additionally, excluding stack traces can help speed up the processing and improve the clarity of the output. 