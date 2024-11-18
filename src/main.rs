use std::io::{self, BufRead, Write};
use std::fs::File;
use regex::Regex;

/// Function to check if a line is a GHC progress marker like `[ n of m ]`
fn is_progress_marker(line: &str) -> bool {
    let marker_re = Regex::new(r"\[\s*\d+\s+of\s+\d+\s*\]").unwrap();
    marker_re.is_match(line)
}

/// Function to check if the line contains specific warning patterns
fn contains_incomplete_warning(line: &str) -> bool {
    // Check for patterns like `-Wincomplete-uni-patterns` or similar
    let incomplete_warning_re = Regex::new(r"incomplete").unwrap();
    incomplete_warning_re.is_match(line)
}

/// Function to filter out and write only the incomplete warning blocks to a file
fn filter_incomplete_warnings(input: &str, output_file: &str) {
    let mut file = File::create(output_file).expect("Unable to create file");
    let mut warning_block = String::new();
    let mut in_warning_block = false;
    let mut found_incomplete_warning = false;

    for line in input.lines() {
        if is_progress_marker(line) || (found_incomplete_warning && line.contains("[-W")) {
            // If we reach a progress marker, process the previous warning block
            if found_incomplete_warning && !warning_block.is_empty() {
                writeln!(file, "{}", warning_block).expect("Unable to write to file");
            }
            // Reset the block status
            warning_block.clear();
            in_warning_block = false;
            found_incomplete_warning = false;
            // line.
            // warning_block.push_str(line);
            warning_block.push_str("\n\n\n");
        }

        // Start capturing lines if they match a warning pattern
        if in_warning_block {
            warning_block.push_str(line);
            warning_block.push('\n');
        }

        // Check if this block has an "incomplete" warning
        if contains_incomplete_warning(line) {
            in_warning_block = true;
            found_incomplete_warning = true;
            warning_block.push_str(line);
            warning_block.push('\n');
        }
    }

    // Write the last captured block if it contains an incomplete warning
    if found_incomplete_warning && !warning_block.is_empty() {
        writeln!(file, "{}", warning_block).expect("Unable to write to file");
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let input = stdin.lock().lines().filter_map(Result::ok).collect::<Vec<String>>().join("\n");

    // File to write the filtered incomplete warnings
    let output_file = "filtered_incomplete_warnings.log";

    // Filter incomplete warnings and write to file
    filter_incomplete_warnings(&input, output_file);
}

