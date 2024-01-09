use std::process::Command;
use std::fs::File;
use std::io::{self, Write};
use whoami;

fn main() {
    // Get the current username dynamically
    let username = whoami::username();

    // Execute the git log command
    let output = Command::new("git")
        .arg("log")
        .arg(format!("--author={}", username))
        .arg("--format=%h,%an,%ad,%s") // Customize the format for CSV output
        .output()
        .expect("Failed to execute command");

    // Check if the command was successful
    if output.status.success() {
        // Convert the output bytes to a string
        let commit_logs = String::from_utf8_lossy(&output.stdout);

        // Split the commit logs into lines
        let lines: Vec<&str> = commit_logs.lines().collect();

        // Take the last 10 commit logs
        let last_10_commits: Vec<&str> = lines.iter().take(10).cloned().collect();

        // Write the last 10 commit logs to a CSV file
        if let Err(err) = write_to_csv("commit_logs.csv", last_10_commits) {
            eprintln!("Error writing to CSV file: {}", err);
        }
    } else {
        // Print the error message if the command failed
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", error_message);
    }
}

fn write_to_csv(filename: &str, commit_logs: Vec<&str>) -> io::Result<()> {
    let mut file = File::create(filename)?;

    // Write CSV header
    writeln!(file, "Commit Hash,Author,Date,Message")?;

    // Write commit logs to the CSV file
    for commit_log in commit_logs {
        writeln!(file, "{}", commit_log)?;
    }

    println!("Commit logs saved to {}", filename);

    Ok(())
}
