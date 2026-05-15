mod templates;

const AZUMI_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

use std::fs;
use std::path::PathBuf;

fn print_usage() {
    let name = std::env::args().next().unwrap_or_else(|| "azumi".to_string());
    eprintln!("Azumi {AZUMI_VERSION} — Scaffolding CLI for the Azumi web framework");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {name} new <project-name> [--out <path>] [--template <name>]");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  new    Create a new Azumi project");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --out <path>        Output directory (defaults to ./<project-name>)");
    eprintln!("  --template <name>   Project template (default, components)");
    eprintln!("  --help              Show this help");
    eprintln!("  --version           Show version");
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
            let mut template = "default".to_string();

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
                    "--template" | "-t" => {
                        i += 1;
                        if i >= args.len() {
                            eprintln!("Error: --template requires a name argument");
                            std::process::exit(1);
                        }
                        template = args[i].clone();
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
            create_project(name, &dir, &template);
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

fn create_project(name: &str, dir: &PathBuf, template: &str) {
    if dir.exists() {
        eprintln!("Error: directory already exists: {}", dir.display());
        std::process::exit(1);
    }

    if !is_valid_rust_ident(name) {
        eprintln!("Error: project name '{}' is not a valid Rust identifier.", name);
        eprintln!("Use lowercase letters, digits, and underscores. Must start with a letter or underscore.");
        std::process::exit(1);
    }

    if name.contains("{{") || name.contains("}}") {
        eprintln!("Error: project name contains template syntax.");
        std::process::exit(1);
    }

    match template {
        "default" => create_default_project(name, dir),
        "components" => create_components_project(name, dir),
        _ => {
            eprintln!("Error: unknown template '{template}'");
            eprintln!("Available templates: default, components");
            std::process::exit(1);
        }
    }
}

fn create_default_project(name: &str, dir: &PathBuf) {
    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create project directory");

    let cargo_toml = templates::CARGO_TOML
        .replace("{{project_name}}", name)
        .replace("{{azumi_version}}", AZUMI_VERSION);
    fs::write(dir.join("Cargo.toml"), &cargo_toml).expect("Failed to write Cargo.toml");
    fs::write(src_dir.join("main.rs"), templates::MAIN_RS).expect("Failed to write main.rs");

    let gitignore = "target/\n.DS_Store\n*.swp\n*.swo\n";
    fs::write(dir.join(".gitignore"), gitignore).expect("Failed to write .gitignore");

    print_success(dir);
}

fn create_components_project(name: &str, dir: &PathBuf) {
    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create project directory");

    let cargo_toml = templates::CARGO_TOML
        .replace("{{project_name}}", name)
        .replace("{{azumi_version}}", AZUMI_VERSION);
    fs::write(dir.join("Cargo.toml"), &cargo_toml).expect("Failed to write Cargo.toml");
    fs::write(src_dir.join("main.rs"), templates::MAIN_RS).expect("Failed to write main.rs");
    fs::write(src_dir.join("components.rs"), templates::COMPONENTS_RS).expect("Failed to write components.rs");

    let gitignore = "target/\n.DS_Store\n*.swp\n*.swo\n";
    fs::write(dir.join(".gitignore"), gitignore).expect("Failed to write .gitignore");

    println!("  ✅ Created Azumi project (with component library) at: {}", dir.display());
    println!();
    println!("  ┌──────────────────────────────────────────────────┐");
    println!("  │  cd {}                         │", dir.display());
    println!("  │  cargo run                                       │");
    println!("  │  → http://localhost:8080                          │");
    println!("  └──────────────────────────────────────────────────┘");
    println!();
    println!("  📦 Component library available in: src/components.rs");
    println!("  📖 Learn more: https://github.com/DraconDev/azumi");
}

fn print_success(dir: &PathBuf) {
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

fn is_valid_rust_ident(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        None => false,
        Some(c) => {
            if !c.is_ascii_alphabetic() && c != '_' {
                return false;
            }
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
    }
}
