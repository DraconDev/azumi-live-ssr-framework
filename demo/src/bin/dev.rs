use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::{Child, Command};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target_bin = args.get(1).map(|s| s.as_str()).unwrap_or("azumi-demo");
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("🚀 Azumi Smart Dev Server");
    println!("   Target Binary: {}", target_bin);
    println!("   Port: {}", port);

    #[allow(clippy::zombie_processes)]
    let mut server = start_server(target_bin);
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    
    // Watch possible src dirs
    if Path::new("src").exists() {
        watcher.watch(Path::new("src"), RecursiveMode::Recursive).unwrap();
    } else if Path::new("demo/src").exists() {
        watcher.watch(Path::new("demo/src"), RecursiveMode::Recursive).unwrap();
    }

    let mut last_run = Instant::now();

    loop {
        if let Ok(Ok(event)) = rx.recv() {
            if last_run.elapsed() < Duration::from_millis(200) { continue; }
            last_run = Instant::now();

            let is_rs = event.paths.iter().any(|p| p.extension().is_some_and(|e| e == "rs"));
            if !is_rs { continue; }

            if let Some(path) = event.paths.first() {
                // Ignore changes that are ONLY CSS (handled by internal watcher)
                // We optimistically try to patch HTML
                if let Ok(true) = try_hot_patch(path, &port) {
                    println!("⚡ Sub-second patch sent!");
                    continue;
                }
            }

            println!("🔄 Logic change detected. Restarting server...");
            let _ = server.kill();
            let _ = server.wait();
            server = start_server(target_bin);
        }
    }
}

#[allow(clippy::zombie_processes)]
fn start_server(bin_name: &str) -> Child {
    Command::new("cargo")
        .args(["run", "--bin", bin_name])
        .spawn()
        .expect("Failed to start server")
}

fn try_hot_patch(path: &Path, port: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    
    let templates = extract_templates(&content, path.to_string_lossy().as_ref());

    if templates.is_empty() {
        return Ok(false);
    }

    let mut success = false;
    for (id, parts) in templates {
        let client = reqwest::blocking::Client::new();
        let payload = serde_json::json!({ "id": id, "parts": parts });
        let url = format!("http://localhost:{}/azumi/update_template", port);
        
        // Use shorter timeout
        if client.post(&url)
            .json(&payload).timeout(Duration::from_millis(100)).send().is_ok() {
            success = true;
        } else {
            return Ok(false);
        }
    }
    Ok(success)
}

fn extract_templates(content: &str, file_path: &str) -> HashMap<String, Vec<String>> {
    let mut templates = HashMap::new();
    let mut current_idx = 0;
    while let Some(idx) = content[current_idx..].find("html!") {
        let start = current_idx + idx;
        let open_brace = match content[start..].find('{') {
            Some(i) => start + i,
            None => { current_idx = start + 5; continue; }
        };
        
        let pre = &content[..start];
        // Line number 1-based
        let line = pre.lines().count(); 
        // If pre ends with newline, count is correct. If not, it's current line.
        // Wait, lines().count() gives number of lines.
        let line = if pre.ends_with('\n') { line + 1 } else { std::cmp::max(1, line) };
        
        let last_line = pre.lines().last().unwrap_or("");
        let col = last_line.len() + 1;
        
        let mut depth = 1;
        let mut inner_end = 0;
        for (i, c) in content[open_brace+1..].char_indices() {
            if c == '{' { depth += 1; }
            else if c == '}' { depth -= 1; }
            
            if depth == 0 {
                inner_end = open_brace + 1 + i;
                break;
            }
        }
        
        if depth == 0 {
            let body = &content[open_brace+1..inner_end];
            // Split by { } blocks roughly to find static parts
            // This is a naive split
            let mut parts = Vec::new();
            let mut last = 0;
            let mut d = 0;
            
            for (i, c) in body.char_indices() {
                if c == '{' {
                    if d == 0 {
                        parts.push(body[last..i].to_string());
                    }
                    d += 1;
                } else if c == '}' {
                    d -= 1;
                    if d == 0 {
                        last = i + 1;
                    }
                }
            }
            parts.push(body[last..].to_string());
            
            let id = format!("{}:{}:{}", file_path, line, col);
            templates.insert(id, parts);
            current_idx = inner_end;
        } else {
            break;
        }
    }
    templates
}
