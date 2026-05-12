mod templates;

use std::fs;
use std::path::PathBuf;

const AZUMI_VERSION: &str = "v47.20.20";

fn print_usage() {
    let name = std::env::args().next().unwrap_or_else(|| "azumi".to_string());
    eprintln!("Azumi {AZUMI_VERSION} — Scaffolding CLI for the Azumi web framework");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {name} new <project-name> [--out <path>]");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  new    Create a new Azumi project");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --out <path>    Output directory (defaults to ./<project-name>)");
    eprintln!("  --help          Show this help");
    eprintln!("  --version       Show version");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "new" => {
            if args.len() < 3 {
                eprintln!("Error: 'new' requires a project name");
                eprintln!();
                print_usage();
                std::process::exit(1);
            }
            let name = &args[2];
            let mut out: Option<PathBuf> = None;

            let mut i = 3;
            while i < args.len() {
                match args[i].as_str() {
                    "--out" | "-o" => {
                        i += 1;
                        if i >= args.len() {
                            eprintln!("Error: --out requires a path argument");
                            std::process::exit(1);
                        }
                        out = Some(PathBuf::from(&args[i]));
                    }
                    _ => {
                        eprintln!("Error: unknown argument: {}", args[i]);
                        print_usage();
                        std::process::exit(1);
                    }
                }
                i += 1;
            }

            let dir = out.unwrap_or_else(|| PathBuf::from(name));
            create_project(name, &dir);
        }
        "--help" | "-h" => {
            print_usage();
        }
        "--version" | "-V" => {
            println!("azumi-cli {AZUMI_VERSION}");
        }
        _ => {
            eprintln!("Error: unknown command '{}'", args[1]);
            eprintln!();
            print_usage();
            std::process::exit(1);
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
