use anyhow::Result;
use clap::Parser;
use colored::*;
use ignore::WalkBuilder;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Directory to search for manifests
    #[arg(default_value = ".")]
    dir: PathBuf,

    /// Required fields (comma separated). E.g., 'license,repository'
    #[arg(short, long, default_value = "license,repository,description")]
    require: String,
}

fn lint_cargo_toml(_path: &Path, content: &str, required: &[&str]) -> Result<Vec<String>> {
    let parsed: toml::Value = toml::from_str(content)?;
    let mut missing = Vec::new();

    if let Some(package) = parsed.get("package").and_then(|v| v.as_table()) {
        for req in required {
            if !package.contains_key(*req) {
                missing.push(req.to_string());
            }
        }
    } else {
        missing.push("package".to_string());
    }

    Ok(missing)
}

fn lint_package_json(_path: &Path, content: &str, required: &[&str]) -> Result<Vec<String>> {
    let parsed: serde_json::Value = serde_json::from_str(content)?;
    let mut missing = Vec::new();

    if let Some(obj) = parsed.as_object() {
        for req in required {
            if !obj.contains_key(*req) {
                missing.push(req.to_string());
            }
        }
    } else {
        missing.push("root object".to_string());
    }

    Ok(missing)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let required_fields: Vec<&str> = args
        .require
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if required_fields.is_empty() {
        println!("{}", "No required fields specified.".yellow());
        return Ok(());
    }

    let mut has_errors = false;

    let walker = WalkBuilder::new(&args.dir).hidden(true).build();

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.file_type().is_some_and(|ft| ft.is_file()) {
            let path = entry.path();
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();

            let missing = if file_name == "Cargo.toml" {
                if let Ok(content) = fs::read_to_string(path) {
                    lint_cargo_toml(path, &content, &required_fields).unwrap_or_default()
                } else {
                    Vec::new()
                }
            } else if file_name == "package.json" {
                if let Ok(content) = fs::read_to_string(path) {
                    lint_package_json(path, &content, &required_fields).unwrap_or_default()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            };

            if !missing.is_empty() && missing != vec!["package"] {
                has_errors = true;
                println!(
                    "{}: {} missing required fields: {}",
                    path.display().to_string().bold(),
                    "Error".red(),
                    missing.join(", ").yellow()
                );
            }
        }
    }

    if has_errors {
        std::process::exit(1);
    } else {
        println!("{}", "All manifests passed.".green());
    }

    Ok(())
}
