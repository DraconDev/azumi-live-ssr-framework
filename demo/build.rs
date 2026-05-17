use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=static");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir);
    let assets_dir = dest_path.join("assets");

    // Create output directory for hashed assets
    if assets_dir.exists() {
        fs::remove_dir_all(&assets_dir).unwrap();
    }
    fs::create_dir_all(&assets_dir).unwrap();

    let mut map = phf_codegen::Map::new();
    // Store tuples of (key, value_code, raw_hashed_path)
    let mut entries = Vec::new();

    // Walk through static directory & Collect entries
    for entry in WalkDir::new("static")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let path = entry.path();
        let relative_path = path.strip_prefix("static").unwrap();

        let content = fs::read(path).expect("Failed to read file");

        // Compute Hash
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = hasher.finalize();
        let hash_hex = hex::encode(hash);
        let short_hash = &hash_hex[0..8];

        // Construct new filename: name.hash.ext
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let ext = path.extension().unwrap_or_default().to_str().unwrap_or("");

        let new_filename = if ext.is_empty() {
            format!("{}.{}", stem, short_hash)
        } else {
            format!("{}.{}.{}", stem, short_hash, ext)
        };

        let new_path = assets_dir.join(&new_filename);
        fs::write(&new_path, &content).expect("Failed to write hashed file");

        // Original request path (e.g., "/static/logo.png")
        let original_key = format!("/static/{}", relative_path.display());
        // Hashed path relative to where we will serve it (e.g., "/assets/logo.a8b9.png")
        let hashed_value = format!("/assets/{}", new_filename);
        // Quoted value for PHF codegen (it expects an expression string)
        let value_code = format!("\"{}\"", hashed_value);

        entries.push((original_key, value_code, hashed_value));
    }

    // Populate the PHF map with long-lived references from `entries`
    for (key, value_code, _) in &entries {
        map.entry(key, value_code);
    }

    // Generate manifest.rs for runtime use
    let manifest_path = dest_path.join("assets_manifest.rs");
    let mut file = fs::File::create(&manifest_path).unwrap();

    write!(
        &mut file,
        "pub static ASSETS: phf::Map<&'static str, &'static str> = "
    )
    .unwrap();
    writeln!(&mut file, "{};", map.build()).unwrap();

    // Generate JSON manifest for macros
    let json_map: std::collections::HashMap<&String, &String> =
        entries.iter().map(|(k, _, v_raw)| (k, v_raw)).collect();

    let manifest_json_path = Path::new("assets_manifest.json");
    let json_file = fs::File::create(manifest_json_path).unwrap();
    serde_json::to_writer_pretty(json_file, &json_map).unwrap();
}
