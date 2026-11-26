use std::{fs, path::Path};

use clap::Parser;

#[derive(Parser)]
#[command(name = "Rustfm")]
#[command(about = "A simple command line file manager")]
#[command(version = "0.1")]
struct Cli {
    /// Directory to explore defalts to default directory
    path: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.as_deref().unwrap_or(".");

    println!("Exloring directory: {}", path);

    list_dir(path);
}

/// Lists files in a directory
fn list_dir(path_string: &str) {
    let path = Path::new(path_string);
    if !path.is_dir() {
        eprintln!("The path you entered is not a directory: {}", path_string);
        return;
    }

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory: {}", e);
            return;
        }
    };

    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for entry_result in entries {
        match entry_result {
            Ok(entry) => {
                let file_type = match entry.file_type() {
                    Ok(file_t) => file_t,
                    Err(e) => {
                        eprintln!("Failed to get file type: {}", e);
                        return;
                    }
                };

                if file_type.is_dir() {
                    let labeled_path = format!("[DIR] {}", entry.file_name().to_string_lossy());
                    directories.push(labeled_path);
                } else if file_type.is_file() {
                    let labeled_path = format!("[FILE] {}", entry.file_name().to_string_lossy());
                    files.push(labeled_path);
                }
                // println!("{:?}", entry.file_name().to_string_lossy());
            }
            Err(e) => {
                eprintln!("Failed to read an entry: {}", e);
            }
        }
    }

    println!("---Directoryies---");
    for dir in directories {
        println!("{dir}");
    }

    println!("---Files---");
    for file in files {
        println!("{file}")
    }
}
