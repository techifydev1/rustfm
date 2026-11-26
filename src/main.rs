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
}