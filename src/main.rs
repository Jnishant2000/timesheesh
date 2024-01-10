use std::process::Command;
use std::fs::File;
use std::io::{self, Write};
// use whoami;
use chrono::{DateTime, Local};

fn main() {
    // Get the current username dynamically
    // let username = whoami::username();

    // Execute the git log command
    let output = Command::new("git")
        .arg("log")
        // .arg(format!("--author={}", username))
        .arg("--format=%h,%an,%ad,%s") // Customize the format for CSV output
        .output()
        .expect("Failed to execute command");

    // Check if the command was successful
    if output.status.success() {
        // Convert the output bytes to a string
        let commit_logs = String::from_utf8_lossy(&output.stdout);

        // Filter out empty lines and split the commit logs into lines
        let lines: Vec<&str> = commit_logs.lines().filter(|line| !line.is_empty()).collect();

        // Process commit logs and calculate time differences
        let commit_data: Vec<CommitData> = lines.iter().take(10).map(|&line| process_commit_line(line)).collect();

        // Write the commit data to a CSV file
        if let Err(err) = write_to_csv("commit_logs.csv", commit_data) {
            eprintln!("Error writing to CSV file: {}", err);
        }
    } else {
        // Print the error message if the command failed
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", error_message);
    }
}

struct CommitData {
    hash: String,
    author: String,
    date: String,
    message: String,
    time_difference: Option<String>,
}

fn process_commit_line(line: &str) -> CommitData {
    let fields: Vec<&str> = line.split(',').collect();
    let hash = fields[0].to_string();
    let author = fields[1].to_string();
    let date = fields[2].to_string();
    let message = fields[3].to_string();
    let time_difference = calculate_time_difference(fields[2]);
    
    CommitData { hash, author, date, message, time_difference }
}

fn calculate_time_difference(date_str: &str) -> Option<String> {
    // Parse the commit date
    if let Ok(commit_date) = DateTime::parse_from_str(date_str, "%c %z") {
        // Calculate the difference in time from the current time
        let current_time = Local::now();
        let duration = current_time.signed_duration_since(commit_date);

        // Format the time difference as "X hours, Y minutes, Z seconds"
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        let formatted_time = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

        Some(formatted_time)
    } else {
        None
    }
}

fn write_to_csv(filename: &str, commit_data: Vec<CommitData>) -> io::Result<()> {
    let mut file = File::create(filename)?;

    // Write CSV header
    writeln!(file, "Commit Hash,Author,Date,Message,Time Difference")?;

    // Write commit data to the CSV file
    for commit in commit_data {
        writeln!(file, "{},{},{},{},{}", commit.hash, commit.author, commit.date, commit.message, commit.time_difference.unwrap_or_else(|| "N/A".to_string()))?;
    }

    println!("Commit data saved to {}", filename);

    Ok(())
}
