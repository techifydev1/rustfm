use std::{
    fs::{self},
    path::Path,
};

use clap::Parser;

#[derive(Parser)]
#[command(name = "Rustfm")]
#[command(about = "A simple command line file manager")]
#[command(version = "0.1")]
struct Cli {
    /// Directory to explore defalts to default directory
    path: Option<String>,

    /// List all files including hidden files
    #[arg(short, long)]
    all: bool,

    /// Lists only files
    #[arg(long, conflicts_with = "only_dirs")]
    only_files: bool,

    /// Lists only dirs
    #[arg(long, conflicts_with = "only_files")]
    only_dirs: bool,
}
struct TypedEntry {
    name: String,
    file_type: String,
    size: u64,
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.as_deref().unwrap_or(".");

    println!("Exloring directory: {}", path);

    list_dir(path, &cli);
}

/// Lists files in a directory
fn list_dir(path_string: &str, cli: &Cli) {
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


    let mut directories: Vec<TypedEntry> = Vec::new();
    let mut files: Vec<TypedEntry> = Vec::new();

    for entry_result in entries {
        match entry_result {
            Ok(entry) => {
                if entry.file_name().to_string_lossy().starts_with(".") && !cli.all {
                    continue;
                }

                let file_type = match entry.file_type() {
                    Ok(file_t) => file_t,
                    Err(e) => {
                        eprintln!("Failed to get file type: {}", e);
                        continue;
                    }
                };

                let extension = entry
                    .path()
                    .extension()
                    .and_then(|os_str| os_str.to_str())
                    .unwrap_or("none")
                    .to_owned();
                let metadata = match entry.metadata() {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to get file's metadata: {}", e);
                        continue;
                    }
                };

                if file_type.is_dir() {
                    if cli.only_files {
                        continue;
                    }
                    let file = TypedEntry {
                        name: entry.file_name().to_string_lossy().into_owned(),
                        file_type: extension,
                        size: metadata.len(),
                    };
                    directories.push(file);
                } else if file_type.is_file() {
                    if cli.only_dirs {
                        continue;
                    }
                    let file = TypedEntry {
                        name: entry.file_name().to_string_lossy().into_owned(),
                        file_type: extension,
                        size: metadata.len(),
                    };
                    files.push(file);
                }
                // println!("{:?}", entry.file_name().to_string_lossy());
            }
            Err(e) => {
                eprintln!("Failed to read an entry: {}", e);
            }
        }
    }

    if !cli.only_files {
        println!("---Directoryies---");
        directories.sort_by(|a, b| a.name.cmp(&b.name));
        for dir in &directories {
            println!("[DIR] {}", dir.name);
            println!("Size: {}", dir.size);
            println!("Type: {}", dir.file_type)
        }
        println!("Total directories: {}", directories.len());
    }

    if !cli.only_dirs {
        let mut total_size = 0;
        println!("---Files---");
        files.sort_by(|a, b| a.name.cmp(&b.name));
        for file in &files {
            println!("[FILE] {}", file.name);
            println!("Size: {}", file.size);
            println!("Type: {}", file.file_type);
            total_size += file.size;
        }

        println!("Total files: {}", files.len());
        println!("Total file size: {}", total_size);
    }
}
