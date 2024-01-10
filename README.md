# Git Commit Log Analyzer

A simple Rust program to analyze the commit log of a Git repository, extracting useful information and saving it to a CSV file.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your machine.
- [Git](https://git-scm.com/) installed on your machine.

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/git-commit-log-analyzer.git
    ```

2. Change into the project directory:

    ```bash
    cd git-commit-log-analyzer
    ```

3. Build the Rust project:

    ```bash
    cargo build --release
    ```

## Usage

Execute the built binary to analyze the Git commit log:

```bash
./target/release/git_commit_log_analyzer
