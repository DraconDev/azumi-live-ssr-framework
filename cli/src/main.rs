mod templates;

use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

const AZUMI_VERSION: &str = "v47.20.20";

#[derive(Parser)]
#[command(name = "azumi", about = "Scaffolding CLI for the Azumi web framework", version = AZUMI_VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Azumi project
    New {
        /// Name of the project
        name: String,
        /// Output directory (defaults to ./<name>)
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name, out } => {
            let dir = out.unwrap_or_else(|| PathBuf::from(&name));
            create_project(&name, &dir);
        }
    }
}

fn create_project(name: &str, dir: &PathBuf) {
    if dir.exists() {
        eprintln!("Error: directory already exists: {}", dir.display());
        std::process::exit(1);
    }

    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create project directory");

    // Write Cargo.toml with project name and current Azumi version
    let cargo_toml = templates::CARGO_TOML
        .replace("{{project_name}}", name)
        .replace("{{azumi_version}}", AZUMI_VERSION);
    fs::write(dir.join("Cargo.toml"), &cargo_toml).expect("Failed to write Cargo.toml");

    // Write main.rs
    fs::write(src_dir.join("main.rs"), templates::MAIN_RS).expect("Failed to write main.rs");

    // Write .gitignore
    let gitignore = "target/\n.DS_Store\n*.swp\n*.swo\n";
    fs::write(dir.join(".gitignore"), gitignore).expect("Failed to write .gitignore");

    println!("  ✅ Created Azumi project at: {}", dir.display());
    println!();
    println!("  ┌──────────────────────────────────────────────────┐");
    println!("  │  cd {}                         │", dir.display());
    println!("  │  cargo run                                       │");
    println!("  │  → http://localhost:8080                          │");
    println!("  └──────────────────────────────────────────────────┘");
    println!();
    println!("  📖 Learn more: https://github.com/DraconDev/azumi");
}
