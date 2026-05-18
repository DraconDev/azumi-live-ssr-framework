# GitHub Repository Settings (SEO)

These values must be set on the GitHub repository to maximize discoverability.

## Repository Name
`azumi-live-ssr-framework` — descriptive, SEO-friendly, tells you exactly what it is

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
- "rust ssr" → in repo name + description + topics
- "server rendered html rust" → in description
- "rust web framework" → in topics + keywords

## crates.io metadata (in Cargo.toml):
- `description`: "Server-rendered HTML framework for Rust. Client interactivity without JavaScript. HTMX alternative with compile-time validation."
- `keywords`: html, htmx, ssr, web-framework, compile-time
- `categories`: template-engine, web-programming, gui

## GitHub rename action items:
1. Go to https://github.com/DraconDev/azumi-live-ssr-framework/settings
2. Rename repo to `azumi-live-ssr-framework`
3. Set the description above
4. Set the homepage URL
5. Add all 18 topics
6. GitHub auto-redirects old URL → new URL
7. Update any external links (blog posts, etc.)

Note: crate name stays `azumi` (cargo add azumi). Only the repo display name changes.
