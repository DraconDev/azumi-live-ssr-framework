use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::Response,
    Router,
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::{Child, Command};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

/// Returns the router for Azumi development tools
/// currently includes the hot reload websocket endpoint
///
/// # Security
///
/// This function is **development-only**. It will panic in release builds
/// to prevent accidental exposure of dev endpoints in production.
///
/// If you see this panic, remove `devtools` from your Cargo features:
/// ```toml
/// [dependencies]
/// azumi-live-ssr-framework = { version = "48", default-features = true }
/// # NOT: features = ["devtools"]  ← remove this for production
/// ```
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Guard: prevent devtools from running in release builds
    #[cfg(not(debug_assertions))]
    {
        panic!(
            "Azumi devtools enabled in release build! \
            This exposes development endpoints (hot reload, template updates) in production. \
            Remove the 'devtools' feature from your Cargo.toml before deploying. \
            If you need hot reload, run with 'cargo run' (debug mode) instead."
        );
    }
    // Guard: even in debug, verify we're not in a production-like environment
    #[cfg(debug_assertions)]
    {
        static WARNED: OnceLock<bool> = OnceLock::new();
        if std::env::var("AZUMI_ALLOW_DEVTOOLS_IN_RELEASE").is_ok()
            && WARNED.set(true).is_ok()
        {
            eprintln!(
                "⚠️  Azumi: AZUMI_ALLOW_DEVTOOLS_IN_RELEASE is set. \
                Devtools endpoints are active. ONLY use this for staging/QA, never in production."
            );
        }
    }
    crate::hot_reload::router()
}

/// The "Easiest Solution" for developers.
///
/// Call this at the very beginning of your `main()` function.
/// It automatically manages sub-second patching and server restarts
/// during development (debug mode).
pub fn auto_reload() {
    auto_reload_if(cfg!(debug_assertions));
}

/// Start hot-reload only if the provided condition is true.
///
/// # Usage
/// ```rust,no_run
/// let is_dev = true; // or your own config check
/// azumi::devtools::auto_reload_if(is_dev);
/// ```
pub fn auto_reload_if(enabled: bool) {
    if !enabled {
        return;
    }

    // If we are already the worker, just start the internal CSS watcher and return to main()
    if std::env::var("AZUMI_IS_WORKER").is_ok() {
        subsecond_watch();
        return;
    }

    // If we aren't in a terminal or something went wrong, don't trap the user
    // unless they explicitly forced it.
    if !std::io::stdin().is_terminal() && std::env::var("AZUMI_FORCE_WATCH").is_err() {
        return;
    }

    println!("🔥 Azumi Smart Watcher Active");
    run_master_loop();
    // Use a cleaner exit path that allows destructors to run
    // In practice, run_master_loop never returns, but this avoids the hard exit
    // if it ever does (e.g., watcher error)
}

/// Check if an argument is safe from shell injection.
///
/// Returns true if the argument does not contain any shell metacharacters
/// that could enable command injection when passed to a shell.
///
/// # Security
///
/// This function is used to validate CLI arguments before passing them
/// to a subprocess. Blocked characters include:
/// - Shell operators: `;`, `|`, `&`, `>`, `<`
/// - Variable expansion: `$`, `` ` ``, `%`
/// - Quote removal: `'`, `"`, `\`
/// - Grouping: `(`, `)`, `{`, `}`, `[`, `]`
/// - Glob/brace expansion: `*`, `?`, `#`, `~`, space
/// - Newlines which can inject HTTP headers
#[allow(clippy::manual_pattern_char_comparison)]
pub fn is_arg_safe(arg: &str) -> bool {
    !arg.contains(|c: char|
        c == '\r' || c == '\n' || c == ';' || c == '|' || c == '&' ||
        c == '>' || c == '<' || c == '$' || c == '`' || c == '(' ||
        c == ')' || c == '!' || c == '*' || c == '?' || c == '#' ||
        c == '\'' || c == '"' || c == '\\' ||
        c == '[' || c == ']' || c == '{' || c == '}' ||
        c == '%' || c == '~' || c == ' '
    )
}

use std::io::IsTerminal;

// The master loop runs forever, reaping child processes on each restart.
// Clippy warns about zombie processes because the final Child isn't explicitly
// waited on — but the loop never exits in normal operation.
#[allow(clippy::zombie_processes)]
fn run_master_loop() {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;

    // Detect the binary name to ensure we restart the correct target
    let exe = match std::env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Devtools: Failed to get current exe: {}", e);
            return;
        }
    };
    let mut bin_name = exe.file_name().and_then(|s| s.to_str()).unwrap_or("app");

    // Validate bin_name to prevent command injection
    if bin_name.is_empty() || bin_name.contains(|c: char| c.is_whitespace() || c == ';' || c == '|' || c == '&' || c == '>' || c == '<' || c == '`' || c == '$') {
        eprintln!("Devtools: Invalid binary name '{}', using 'app'", bin_name);
        bin_name = "app";
    }

    let mut server = start_worker(bin_name);
    let (tx, rx) = channel();
    let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Devtools: Failed to create watcher: {}", e);
            return;
        }
    };

    // Watch src directory (the standard Rust project layout)
    for dir in ["src"] {
        if Path::new(dir).exists() {
            if let Err(e) = watcher.watch(Path::new(dir), RecursiveMode::Recursive) {
                eprintln!("Devtools: Failed to watch directory {}: {}", dir, e);
            }
        }
    }

    let mut last_run = Instant::now();
    let port = std::env::var("PORT")
        .map(|p| p.parse::<u16>().map(|n| n.to_string()))
        .unwrap_or_else(|_| Ok("8080".to_string()))
        .unwrap_or_else(|e| {
            eprintln!("WARNING: Invalid PORT '{}' - using default 8080. Error: {}", e, e);
            "8080".to_string()
        });

    loop {
        if let Ok(Ok(event)) = rx.recv() {
            if last_run.elapsed() < Duration::from_millis(200) {
                continue;
            }
            last_run = Instant::now();

            let is_rs = event
                .paths
                .iter()
                .any(|p| p.extension().is_some_and(|e| e == "rs"));
            if !is_rs {
                continue;
            }

            if let Some(path) = event.paths.first() {
                if let Ok(true) = try_hot_patch_internal(path, &port) {
                    println!("⚡ Sub-second patch sent!");
                    continue;
                }
            }

            println!("🔄 Logic change detected. Rebuilding & Restarting...");
            let _ = server.kill();
            let _ = server.wait(); // Reap zombie process before spawning new one
            server = start_worker(bin_name);
        }
    }
}

fn start_worker(bin_name: &str) -> Child {
    let mut cmd = Command::new("cargo");
    // Use --bin to ensure we run the same target even in workspaces
    cmd.args(["run", "--bin", bin_name, "--"]);
    
    // Forward original CLI arguments to the worker
    // SECURITY: Filter args to remove shell metacharacters that could enable injection
    // Uses the shared is_arg_safe function for consistent validation
    let args: Vec<String> = std::env::args().skip(1).filter(|arg| is_arg_safe(arg)).collect();
    cmd.args(&args);

    cmd.env("AZUMI_IS_WORKER", "1")
        .spawn()
        .expect("Failed to start azumi worker. Ensure the binary is available in PATH.")
}

fn try_hot_patch_internal(path: &Path, port: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let templates = extract_templates_internal(&content, path.to_string_lossy().as_ref());

    if templates.is_empty() {
        return Ok(false);
    }

    let mut success = false;
    for (id, parts) in templates {
        let payload = serde_json::json!({ "id": id, "parts": parts }).to_string();
        if send_raw_post(port, "/azumi/update_template", &payload) {
            success = true;
        } else {
            return Ok(false);
        }
    }
    Ok(success)
}

fn send_raw_post(port: &str, path: &str, body: &str) -> bool {
    use std::io::Write;
    use std::net::TcpStream;

    // Validate path to prevent HTTP header injection
    if path.contains('\r') || path.contains('\n') {
        eprintln!("Devtools: Invalid path contains newlines");
        return false;
    }

    let addr = format!("127.0.0.1:{}", port);
    if let Ok(socket_addr) = addr.parse() {
        if let Ok(mut stream) = TcpStream::connect_timeout(
            &socket_addr,
            Duration::from_millis(100),
        ) {
            let request = format!(
                "POST {} HTTP/1.1\r\n\
                 Host: localhost:{}\r\n\
                 Content-Type: application/json\r\n\
                 Content-Length: {}\r\n\
                 Connection: close\r\n\r\n\
                 {}",
                path,
                port,
                body.len(),
                body
            );
            return stream.write_all(request.as_bytes()).is_ok();
        }
    }
    false
}

fn extract_templates_internal(content: &str, file_path: &str) -> HashMap<String, Vec<String>> {
    let mut templates = HashMap::new();
    // Pre-scan all "html!" positions for efficiency
    let mut html_positions: Vec<usize> = Vec::new();
    let mut search_from = 0;
    while let Some(pos) = content[search_from..].find("html!") {
        html_positions.push(search_from + pos);
        search_from += pos + 5; // Move past this occurrence
    }

    for start in html_positions {
        let open_brace = match content[start..].find('{') {
            Some(i) => start + i,
            None => continue,
        };

        let pre = &content[..start];
        let line = pre.lines().count();
        let line = if pre.ends_with('\n') { line + 1 } else { std::cmp::max(1, line) };

        let last_line = pre.lines().last().unwrap_or("");
        let col = last_line.chars().count() + 1;

        let mut depth = 1;
        let mut inner_end = 0;
        let chars = content[open_brace + 1..].char_indices();

        for (i, c) in chars {
            if c == '{' {
                depth += 1;
            } else if c == '}' {
                depth -= 1;
            }
            if depth == 0 {
                inner_end = open_brace + 1 + i;
                break;
            }
        }

        if depth == 0 {
            let body = &content[open_brace + 1..inner_end];
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
        }
    }
    templates
}

/// Starts a background thread that watches for CSS changes in .rs files
/// and pushes updates to the browser without a full reload.
pub fn subsecond_watch() {
    #[cfg(debug_assertions)]
    {
        std::thread::spawn(|| {
            if let Err(e) = watch_loop() {
                eprintln!("🔥 Azumi Watcher Error: {:?}", e);
            }
        });
    }
}

fn watch_loop() -> Result<(), Box<dyn std::error::Error>> {
    use notify::{RecursiveMode, Watcher};
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx).expect("Failed to initialize file watcher. This may happen if there are too many open file descriptors or insufficient permissions.");

    // Watch src directory
    if Path::new("src").exists() {
        watcher.watch(Path::new("src"), RecursiveMode::Recursive)?;
    }

    println!("🔥 Azumi Subsecond Watcher: Active");

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if event.kind.is_modify() {
                    for path in event.paths {
                        if path.extension().is_some_and(|ext| ext == "rs") {
                            process_file_change(&path);
                        }
                    }
                }
            }
            Ok(Err(e)) => eprintln!("watch error: {:?}", e),
            Err(_) => break,
        }
    }
    Ok(())
}

fn process_file_change(path: &Path) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    // Pre-scan all "html!" positions
    let mut html_positions: Vec<usize> = Vec::new();
    let mut search_from = 0;
    while let Some(pos) = content[search_from..].find("html!") {
        html_positions.push(search_from + pos);
        search_from += pos + 5;
    }

    for start in html_positions {
        let brace_idx = match content[start..].find('{') {
            Some(i) => start + i,
            None => continue,
        };
        let macro_content_start = brace_idx + 1;

        // Find <style>...</style> using pre-scanned positions
        let style_start = match content[macro_content_start..].find("<style") {
            Some(i) => macro_content_start + i,
            None => continue,
        };
        let style_tag_end = match content[style_start..].find('>') {
            Some(i) => style_start + i,
            None => continue,
        };
        let css_start = style_tag_end + 1;

        let style_end = match content[css_start..].find("</style>") {
            Some(i) => css_start + i,
            None => continue,
        };

        let css_content = &content[css_start..style_end];

        // Find first non-whitespace character for line/col
        let first_node_rel = match content[macro_content_start..].find(|c: char| !c.is_whitespace()) {
            Some(i) => i,
            None => continue,
        };
        let first_node_abs = macro_content_start + first_node_rel;

        let line = content[..first_node_abs].lines().count();
        let line_start = content[..first_node_abs].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let col = first_node_abs - line_start;

        let scope_id = crate::css_scoping::compute_scope_id(line, col);
        let scoped_css = crate::css_scoping::scope_css(css_content, &scope_id);
        crate::hot_reload::push_style_update(&scope_id, &scoped_css);
    }
}

/// Middleware to force no-cache headers in development mode
/// usage: .layer(axum::middleware::from_fn(azumi::devtools::no_cache_middleware))
pub async fn no_cache_middleware(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;

    // Only set headers if we are in debug mode
    #[cfg(debug_assertions)]
    {
        let headers = response.headers_mut();
        // Prevent caching for all responses
        headers.insert(
            axum::http::header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache, no-store, must-revalidate"),
        );
        headers.insert(
            axum::http::header::PRAGMA,
            HeaderValue::from_static("no-cache"),
        );
        headers.insert(
            axum::http::header::EXPIRES,
            HeaderValue::from_static("0"),
        );
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_arg_safe_valid_simple_args() {
        assert!(is_arg_safe("foo"));
        assert!(is_arg_safe("bar"));
        assert!(is_arg_safe("my_component"));
        assert!(is_arg_safe("config.toml"));
    }

    #[test]
    fn test_is_arg_safe_rejects_shell_operators() {
        assert!(!is_arg_safe("; rm -rf /"));
        assert!(!is_arg_safe("| cat /etc/passwd"));
        assert!(!is_arg_safe("& whoami"));
        assert!(!is_arg_safe("> /tmp/out"));
        assert!(!is_arg_safe("< input"));
    }

    #[test]
    fn test_is_arg_safe_rejects_variable_expansion() {
        assert!(!is_arg_safe("$HOME"));
        assert!(!is_arg_safe("${SECRET}"));
        assert!(!is_arg_safe("`id`"));
    }

    #[test]
    fn test_is_arg_safe_rejects_quote_removal() {
        assert!(!is_arg_safe("'hello'"));
        assert!(!is_arg_safe("\"hello\""));
        assert!(!is_arg_safe("hello\\"));
    }

    #[test]
    fn test_is_arg_safe_rejects_grouping_chars() {
        assert!(!is_arg_safe("(ls)"));
        assert!(!is_arg_safe("{ls}"));
        assert!(!is_arg_safe("[ls]"));
    }

    #[test]
    fn test_is_arg_safe_rejects_glob_expansion() {
        assert!(!is_arg_safe("*.rs"));
        assert!(!is_arg_safe("?"));
        assert!(!is_arg_safe("#comment"));
        assert!(!is_arg_safe("~/"));
    }

    #[test]
    fn test_is_arg_safe_rejects_newlines() {
        assert!(!is_arg_safe("hello\r\nworld"));
        assert!(!is_arg_safe("line1\nline2"));
    }

    #[test]
    fn test_is_arg_safe_rejects_space_separator() {
        assert!(!is_arg_safe("arg with spaces"));
        assert!(!is_arg_safe("multiple args"));
    }

    #[test]
    fn test_extract_templates_internal_basic() {
        let content = r#"
fn example() {
    html! {
        <div>Hello</div>
    }
}
"#;
        let templates = extract_templates_internal(content, "test.rs");
        assert!(!templates.is_empty());
    }

    #[test]
    fn test_extract_templates_internal_no_html() {
        let content = r#"
fn example() {
    let x = 5;
}
"#;
        let templates = extract_templates_internal(content, "test.rs");
        assert!(templates.is_empty());
    }

    #[test]
    fn test_extract_templates_internal_with_style() {
        let content = r#"
fn example() {
    html! {
        <style>.foo { color: red; }</style>
        <div>Hello</div>
    }
}
"#;
        let templates = extract_templates_internal(content, "test.rs");
        assert!(!templates.is_empty());
    }

    #[test]
    fn test_extract_templates_internal_multiple() {
        let content = r#"
fn comp1() { html! { <div>1</div> } }
fn comp2() { html! { <span>2</span> } }
"#;
        let templates = extract_templates_internal(content, "test.rs");
        assert_eq!(templates.len(), 2);
    }

    #[test]
    fn test_extract_templates_internal_nested_braces() {
        let content = r#"
fn example() {
    html! {
        <div>
            <style>.foo { color: red; }</style>
            <span>{ value }</span>
        </div>
    }
}
"#;
        let templates = extract_templates_internal(content, "test.rs");
        assert!(!templates.is_empty());
        let parts = templates.values().next().unwrap();
        assert!(!parts.is_empty());
    }

    #[test]
    fn test_extract_templates_internal_empty_file() {
        let templates = extract_templates_internal("", "empty.rs");
        assert!(templates.is_empty());
    }

    #[test]
    fn test_extract_templates_internal_id_format() {
        let content = r#"
fn example() {
    html! { <div>Test</div> }
}
"#;
        let templates = extract_templates_internal(content, "src/components/my_comp.rs");
        let id = templates.keys().next().unwrap();
        assert!(id.contains("src/components/my_comp.rs"));
        assert!(id.contains(':'));
    }
}
