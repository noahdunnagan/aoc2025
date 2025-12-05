use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Advent of Code 2025 task runner")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new day from template
    New {
        /// Day number (1-25)
        day: u8,
    },
    /// Run a specific day
    Run {
        /// Day number (1-25)
        day: u8,
        /// Run in release mode
        #[arg(short, long)]
        release: bool,
    },
    /// Run all days
    All {
        /// Run in release mode
        #[arg(short, long)]
        release: bool,
    },
    /// Format all code
    Fmt,
    /// Run clippy on all code
    Clippy,
    /// Download input for a day (requires AOC_SESSION env var)
    Download {
        /// Day number (1-25)
        day: u8,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { day } => new_day(day),
        Commands::Run { day, release } => run_day(day, release),
        Commands::All { release } => run_all(release),
        Commands::Fmt => fmt(),
        Commands::Clippy => clippy(),
        Commands::Download { day } => download_input(day),
    }
}

fn new_day(day: u8) -> Result<()> {
    let day_str = format!("day{:02}", day);
    let day_path = Path::new(&day_str);

    if day_path.exists() {
        anyhow::bail!("Day {} already exists!", day);
    }

    println!(" Creating {}...", day_str);

    // Create directory structure
    fs::create_dir_all(day_path.join("src"))?;

    // Create Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version.workspace = true
edition.workspace = true

[dependencies]
common.workspace = true
"#,
        day_str
    );
    fs::write(day_path.join("Cargo.toml"), cargo_toml)?;

    // Create main.rs from template
    let main_rs = format!(
        r##"use common::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) -> i64 {{
    // TODO: Implement part 1
    0
}}

fn part2(input: &str) -> i64 {{
    // TODO: Implement part 2
    0
}}

fn main() {{
    println!(" Day {:02} ");
    println!("Part 1: {{}}", part1(INPUT));
    println!("Part 2: {{}}", part2(INPUT));
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_example_part1() {{
        assert_eq!(part1(EXAMPLE.trim()), 0);
    }}

    #[test]
    fn test_example_part2() {{
        assert_eq!(part2(EXAMPLE.trim()), 0);
    }}

    #[test]
    fn test_input_part1() {{
        assert_eq!(part1(INPUT), 0);
    }}

    #[test]
    fn test_input_part2() {{
        assert_eq!(part2(INPUT), 0);
    }}
}}
"##,
        day
    );
    fs::write(day_path.join("src/main.rs"), main_rs)?;

    // Create empty input file
    fs::write(day_path.join("input.txt"), "")?;

    // Add to workspace
    add_to_workspace(&day_str)?;

    println!("✨ Created {}!", day_str);
    println!("");
    println!("Next steps:");
    println!("  1. Paste your input into {}/input.txt", day_str);
    println!("  2. Run with: cargo run -p {}", day_str);
    println!("  3. Or use: cargo d {}", day_str);
    println!("");

    Ok(())
}

fn add_to_workspace(day_str: &str) -> Result<()> {
    let cargo_path = Path::new("Cargo.toml");
    let content = fs::read_to_string(cargo_path)?;

    // Simple approach: find members array and add the new day
    if content.contains(&format!("\"{}\"", day_str)) {
        return Ok(()); // Already in workspace
    }

    // Find the members line and add the new day
    let new_content = content.replace("members = [", &format!("members = [\n    \"{}\",", day_str));

    fs::write(cargo_path, new_content)?;
    Ok(())
}

fn run_day(day: u8, release: bool) -> Result<()> {
    let day_str = format!("day{:02}", day);

    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("-p").arg(&day_str);

    if release {
        cmd.arg("--release");
    }

    let status = cmd.status().context("Failed to run cargo")?;

    if !status.success() {
        anyhow::bail!("Failed to run day {}", day);
    }

    Ok(())
}

fn run_all(release: bool) -> Result<()> {
    for day in 1..=25 {
        let day_str = format!("day{:02}", day);
        let day_path = Path::new(&day_str);

        if day_path.exists() {
            println!("\n{}", "=".repeat(40));
            run_day(day, release)?;
        }
    }
    Ok(())
}

fn fmt() -> Result<()> {
    println!("🔧 Formatting code...");
    let status = Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .status()
        .context("Failed to run cargo fmt")?;

    if status.success() {
        println!("✨ All formatted!");
    }
    Ok(())
}

fn clippy() -> Result<()> {
    println!("🔍 Running clippy...");
    let status = Command::new("cargo")
        .arg("clippy")
        .arg("--all-targets")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .status()
        .context("Failed to run clippy")?;

    if !status.success() {
        anyhow::bail!("Clippy found issues!");
    }

    println!("✨ Clippy is happy!");
    Ok(())
}

fn download_input(day: u8) -> Result<()> {
    let session = std::env::var("AOC_SESSION").context(
        "AOC_SESSION environment variable not set. Get your session cookie from adventofcode.com",
    )?;

    let url = format!("https://adventofcode.com/2025/day/{}/input", day);
    let day_str = format!("day{:02}", day);
    let input_path = format!("{}/input.txt", day_str);

    println!("📥 Downloading input for day {}...", day);

    let output = Command::new("curl")
        .arg("-s")
        .arg("-H")
        .arg(format!("Cookie: session={}", session))
        .arg(&url)
        .output()
        .context("Failed to run curl")?;

    if output.status.success() {
        let content = String::from_utf8_lossy(&output.stdout);
        if content.contains("Puzzle inputs differ by user") || content.contains("Please log in") {
            anyhow::bail!("Invalid session cookie!");
        }
        fs::write(&input_path, &*content)?;
        println!("✨ Downloaded to {}", input_path);
    } else {
        anyhow::bail!("Failed to download input");
    }

    Ok(())
}
