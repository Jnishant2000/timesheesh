use std::env;
use std::process::Command;
use std::fs::File;
use std::io::{self, Write};
use chrono::{DateTime, Local, TimeZone};

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Extract project name and changed files from arguments
    if args.len() >= 3 {
        let project_name = &args[1];
        let changed_files = &args[2];

        // Execute the git log command with the specified project name and changed files
        let output = Command::new("git")
            .arg("log")
            .arg("--format=%h,%an,%ad,%s")
            .arg("--")
            .args(changed_files.split(','))
            .output()
            .expect("Failed to execute command");

        // Check if the command was successful
        if output.status.success() {
            // Convert the output bytes to a string
            let commit_logs = String::from_utf8_lossy(&output.stdout);

            // Filter out empty lines and split the commit logs into lines
            let lines: Vec<&str> = commit_logs.lines().filter(|line| !line.is_empty()).collect();

            // Process commit logs and calculate time differences
            let commit_data: Vec<CommitData> = lines.iter().map(|&line| process_commit_line(line)).collect();

            // Write the commit data to a CSV file
            if let Err(err) = write_to_csv("commit_logs.csv", project_name, commit_data) {
                eprintln!("Error writing to CSV file: {}", err);
            }
        } else {
            // Print the error message if the command failed
            let error_message = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {}", error_message);
        }
    } else {
        eprintln!("Usage: cargo run --release -- <project_name> <changed_files>");
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

fn write_to_csv(filename: &str, project_name: &str, commit_data: Vec<CommitData>) -> io::Result<()> {
    let mut file = File::create(filename)?;

    // Write CSV header with additional columns for project name and changed files
    writeln!(file, "Project Name,Changed Files,Commit Hash,Author,Date,Message,Time Difference")?;

    // Write additional information to each row
    for commit in commit_data {
        writeln!(file, "{},{},{},{},{},{},{}", project_name, commit.changed_files.unwrap_or("N/A"), commit.hash, commit.author, commit.date, commit.message, commit.time_difference.unwrap_or("N/A"))?;
    }

    println!("Commit data saved to {}", filename);

    Ok(())
}
