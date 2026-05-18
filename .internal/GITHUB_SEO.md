# GitHub Repository Settings (SEO)

These values must be set on the GitHub repository to maximize discoverability.

## Repository Name
`azumi` (keep it short — the brand is memorable)

## GitHub Description (most important for search)
```
Azumi — Server-rendered HTML framework for Rust. Client interactivity without JavaScript. HTMX alternative with compile-time validation.
```

## Homepage URL
```
https://azumi.dev
```

## Topics (GitHub tags — max 20)
```
rust
html
htmx
ssr
web-framework
compile-time
server-rendered
xss-prevention
component
macro
no-javascript
interactive
htmx-alternative
templating
axum
web
gui
```

## What this targets in GitHub search:
- "rust html framework" → our description contains all 4 words
- "htmx alternative" → explicitly in description
- "rust ssr" → in topics + description
- "server rendered html rust" → in description
- "rust web framework" → in topics + keywords

## crates.io metadata (in Cargo.toml):
- `description`: "Server-rendered HTML framework for Rust. Client interactivity without JavaScript. HTMX alternative with compile-time validation."
- `keywords`: html, htmx, ssr, web-framework, compile-time
- `categories`: template-engine, web-programming, gui

## Org name options (if renaming):
- `azumi-rs/azumi` — Rust convention, clean
- `DraconDev/azumi` — current (works fine, personal brand)

Recommendation: Keep `DraconDev/azumi` for now. If the project grows beyond DraconDev, create `azumi-rs` org and transfer.
