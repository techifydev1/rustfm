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
        println!("The path you entered is not a directory: {}", path_string);
    }
    if let Ok(entries) = fs::read_dir(path) {
        for entry_result in entries {
            match entry_result {
                Ok(entry) => {
                    println!("{:?}", entry.path());
                }
                Err(e) => {
                    eprintln!("Failed to read an entry: {}", e);
                }
            }
        }
    }
}