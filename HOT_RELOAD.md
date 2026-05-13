# 🔥 Hot Reload in Azumi
# Hot Reload in Azumi

> **DEPRECATED**: This file has been consolidated into [docs/guide.md](docs/guide.md#debugging). The content below is preserved for reference.

Azumi provides a built-in, self-hosting hot reload system that requires zero configuration and zero external binaries.
---

## 🚀 The "Gold Standard" (Recommended)

Just add one line to your `main.rs`. Azumi will automatically detect development mode, watch your files, and patch your UI in sub-second time.

### Setup

```rust
// src/main.rs
#[tokio::main]
async fn main() {
    // 1. ⚡ Add this line at the VERY BEGINNING of main()
    azumi::devtools::auto_reload();
    
    // ... setup state ...

    // 2. 🔌 Merge the devtools router
    // IMPORTANT: If using .with_state(), merge devtools AFTER!
    let app = Router::new()
        .route("/", get(handler))
        .with_state(my_state) 
        .merge(azumi::devtools::router());

    // ... axum::serve ...
}
```

### How it works:
1.  **CSS Changes**: Patched instantly (< 50ms) without page reload.
2.  **HTML Changes**: Patched in sub-second time via WebSocket reload.
3.  **Logic Changes**: Triggers an automatic server restart.

---

## 🐌 Fallback: Classic Mode

If you prefer not to use the built-in watcher, you can use `cargo-watch`. This is slower (5-15s) as it restarts the entire compiler on every change.

```bash
cargo install cargo-watch
cargo watch -x run
```

---

## 🛠️ Requirements & Troubleshooting

-   **Debug Mode**: Hot reload is only active in `debug` builds (not `--release`).
-   **Terminal**: The master watcher only starts when running in an interactive terminal.
-   **WebSocket**: Ensure `azumi::devtools::router()` is merged into your Axum app so the browser can receive signals.
-   **Port**: By default, it expects the server on port `8080`. If you use a different port, set the `PORT` environment variable:
    ```bash
    PORT=3000 cargo run
    ```