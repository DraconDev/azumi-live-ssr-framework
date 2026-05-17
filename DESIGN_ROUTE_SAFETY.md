# Design: Compile-Time Route Type Safety

## The Problem

Azumi validates **everything inside `html!`** — tags, attributes, CSS, XSS patterns, HTML structure. But the **links between pages** are untyped strings:

```rust
html! {
    <a href="/abuot">"About"</a>        // ❌ typo — compiles fine, 404 at runtime
    <form az-action="/contct">          // ❌ typo — compiles fine, broken form
    <a href="/blog/posts/42">"Post"</a> // ❌ wrong slug format — compiles fine
}
```

This is the biggest class of bug that Azumi's philosophy should prevent but currently doesn't.

## Why This Is Hard

Proc-macros are **closed** — `html!` only sees its own token stream. It can't reach across files to know what routes exist. This is the fundamental constraint that makes route validation harder than CSS or HTML validation (which are local to each `html!` invocation).

### Approaches That DON'T Work

| Approach | Why It Fails |
|----------|-------------|
| `inventory` at compile time | `inventory` collects at **runtime** (before `main()`). Proc-macros run at **compile time**. They can never meet. |
| Parse the whole crate from `html!` | Proc-macros can only see their own token stream. No access to other files. |
| Global const via `lazy_static!` | Const evaluation happens after macro expansion. Chicken-and-egg. |
| `include!` generated file | `html!` is a proc-macro, not a `macro_rules!`. It can't `include!` files. |

## The Two-Level Solution

### Level 1: Route Constants (80% of value, zero `html!` changes)

A `routes!` macro that generates **typed route constants**. Use a constant, can't typo it.

```rust
// routes.rs
azumi::routes! {
    home:          "/"                      => get  home_handler,
    about:         "/about"                 => get  about_handler,
    contact:       "/contact"              => post contact_action,
    blog_index:    "/blog"                 => get  blog_index,
    blog_post:     "/blog/posts/{slug}"    => get  post_page,
    blog_contact:  "/blog/contact"         => get  contact_page,
}
```

This expands to:

```rust
pub mod routes {
    /// Route: "/" (GET)
    pub const HOME: &str = "/";
    /// Route: "/about" (GET)
    pub const ABOUT: &str = "/about";
    /// Route: "/contact" (POST)
    pub const CONTACT: &str = "/contact";
    /// Route: "/blog" (GET)
    pub const BLOG_INDEX: &str = "/blog";
    /// Route: "/blog/posts/{slug}" (GET)
    pub const BLOG_POST: &str = "/blog/posts/{slug}";
    /// Route: "/blog/contact" (GET)
    pub const BLOG_CONTACT: &str = "/blog/contact";

    /// Build the complete Axum router with all routes
    pub fn router() -> axum::Router {
        axum::Router::new()
            .route("/", axum::routing::get(home_handler))
            .route("/about", axum::routing::get(about_handler))
            .route("/contact", axum::routing::post(contact_action))
            .route("/blog", axum::routing::get(blog_index))
            .route("/blog/posts/{slug}", axum::routing::get(post_page))
            .route("/blog/contact", axum::routing::get(contact_page))
    }
}
```

Usage in `html!` — **no macro changes needed**:

```rust
html! {
    // Type-safe: constant, can't typo
    <a href={routes::ABOUT}>"About"</a>

    // Type-safe: action URL
    <form az-action={routes::CONTACT} method="POST">

    // Type-safe: dynamic route with format
    <a href={format!("/blog/posts/{}", slug)}>"Read post"</a>
}
```

**If you type `routes::ABUOT`, you get a compile error.** Rust's type system does the work.

**Implementation effort:** ~200 lines for a new `routes.rs` module in the macro crate. No changes to `html!` itself.

---

### Level 2: String Literal Validation (remaining 20%, requires file communication)

Validate bare string literals like `href="/about"` against the known route table inside `html!`. This requires the proc-macro to access route data from other files.

**The mechanism: `routes!` writes a manifest file, `html!` reads it.**

```
┌──────────────┐     writes manifest      ┌─────────────────┐
│  routes!     │ ──────────────────────→  │  target/         │
│  (proc-macro)│    target/azumi-routes.json│  azumi-routes.json│
└──────────────┘                           └─────────────────┘
                                                    │
                                                    │ reads
                                                    ▼
                                           ┌──────────────────┐
                                           │  html!            │
                                           │  (proc-macro)     │
                                           │  validates href=  │
                                           └──────────────────┘
```

**Step 1: `routes!` writes the manifest**

In `macros/src/routes.rs`, the `routes!` macro writes a JSON manifest to a well-known location:

```rust
pub fn expand_routes(input: TokenStream) -> TokenStream {
    let parsed = parse_route_definitions(&input);

    // Write manifest file for html! to read
    let manifest = serde_json::json!({
        "routes": parsed.iter().map(|r| &r.path).collect::<Vec<_>>(),
        "actions": parsed.iter()
            .filter(|r| r.method == "post")
            .map(|r| &r.path)
            .collect::<Vec<_>>(),
    });

    // Write to OUT_DIR or a well-known temp location
    let manifest_path = std::env::var("OUT_DIR")
        .map(|dir| std::path::Path::new(&dir).join("azumi-routes.json"))
        .unwrap_or_else(|_| {
            std::path::PathBuf::from("/tmp/azumi-routes.json")
        });

    std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap())
        .unwrap_or_else(|e| eprintln!("warning: failed to write route manifest: {}", e));

    // Tell Cargo to rerun if routes change
    // (this is emitted by the calling crate's build script)

    // Generate the module with constants + router
    generate_routes_module(&parsed)
}
```

**Step 2: Build script coordinates the pipeline**

In the user's `build.rs`:

```rust
fn main() {
    // Tell Cargo about the route manifest
    println!("cargo:rerun-if-changed=src/routes.rs");

    // Set OUT_DIR so routes! can write the manifest
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-env=AZUMI_ROUTES_DIR={}", out_dir);
}
```

**Step 3: `html!` reads the manifest and validates**

In the `html!` macro expansion, before validation:

```rust
// In macros/src/validators.rs or a new route_validator.rs

fn validate_route_references(nodes: &[Node]) -> Vec<TokenStream> {
    // Read the route manifest
    let route_table = load_route_manifest();
    if route_table.is_empty() {
        return vec![]; // No routes defined — skip validation
    }

    let mut errors = vec![];

    for node in nodes {
        if let Node::Element(elem) = node {
            for attr in &elem.attrs {
                match attr.name.as_str() {
                    "href" | "action" => {
                        if let AttributeValue::Static(path) = &attr.value {
                            if path.starts_with('/') && !route_table.contains(path) {
                                let msg = format!(
                                    "Unknown route '{}'. Check for typos or register it with azumi::routes!. \
                                     Known routes: {}",
                                    path,
                                    route_table.join(", ")
                                );
                                errors.push(quote_spanned! {
                                    attr.value_span.unwrap_or(attr.span) =>
                                    compile_error!(#msg);
                                });
                            }
                        }
                    }
                    "az-action" => {
                        if let AttributeValue::Static(path) = &attr.value {
                            // Check against registered action paths
                            if !action_table.contains(path) {
                                let msg = format!(
                                    "Unknown action '{}'. Known actions: {}",
                                    path,
                                    action_table.join(", ")
                                );
                                errors.push(quote_spanned! {
                                    attr.value_span.unwrap_or(attr.span) =>
                                    compile_error!(#msg);
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    errors
}

fn load_route_manifest() -> Vec<String> {
    // Try to read from OUT_DIR first (set by build script)
    if let Ok(dir) = std::env::var("AZUMI_ROUTES_DIR") {
        let path = std::path::Path::new(&dir).join("azumi-routes.json");
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(routes) = manifest["routes"].as_array() {
                        return routes.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect();
                    }
                }
            }
        }
    }

    // Fallback: try well-known temp location
    let fallback = std::path::PathBuf::from("/tmp/azumi-routes.json");
    if fallback.exists() {
        if let Ok(content) = std::fs::read_to_string(&fallback) {
            if let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(routes) = manifest["routes"].as_array() {
                    return routes.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                }
            }
        }
    }

    vec![] // No manifest found — skip validation gracefully
}
```

**Step 4: Opt-out for external links**

Just like `class:external` bypasses CSS validation:

```rust
html! {
    // External links bypass route validation
    <a href:external="https://github.com">"GitHub"</a>
    <a href:external="/third-party-path">"Partner"</a>
}
```

This is already supported — `href:external` is parsed as a different attribute name, so the route validator simply skips it.

---

## Handling `#[azumi::action]` Auto-Registered Routes

The `#[azumi::action]` macro already generates action paths automatically:

```rust
// Current expansion:
azumi::inventory::submit! {
    azumi::action::ActionEntry {
        path: concat!("/_azumi/action/", stringify!(contact_action)),
        handler: contact_action_router,
    }
}
```

**Enhancement:** Also write these to the manifest:

```rust
// In action.rs expansion, also write to the manifest:
let manifest_path = get_manifest_path();
let mut manifest = read_manifest(&manifest_path);
manifest["actions"].as_array_mut().unwrap().push(
    serde_json::Value::String(format!("/_azumi/action/{}", stringify!(#fn_name)))
);
write_manifest(&manifest_path, &manifest);
```

This way, `html!` validates `az-action="/_azumi/action/contact_action"` too.

---

## Handling Dynamic Route Segments

Routes like `/blog/posts/{slug}` have dynamic segments. String literal `href="/blog/posts/hello"` should match the pattern. The validator needs fuzzy matching:

```rust
fn route_matches_pattern(route: &str, pattern: &str) -> bool {
    // Exact match
    if route == pattern { return true; }

    // Pattern has dynamic segments: /blog/posts/{slug}
    let pattern_parts: Vec<&str> = pattern.split('/').collect();
    let route_parts: Vec<&str> = route.split('/').collect();

    if pattern_parts.len() != route_parts.len() { return false; }

    for (pat, seg) in pattern_parts.iter().zip(route_parts.iter()) {
        if pat.starts_with('{') && pat.ends_with('}') { continue; } // dynamic segment
        if pat != seg { return false; }
    }

    true
}
```

---

## Implementation Plan

### Phase 1: Route Constants (1 week)

| Task | File | Lines |
|------|------|-------|
| `routes!` macro definition | `macros/src/routes.rs` (new) | ~250 |
| Register in `lib.rs` | `macros/src/lib.rs` | +5 |
| Unit tests | `macros/src/routes.rs` | ~100 |
| Integration test | `tests/route_tests.rs` (new) | ~150 |
| Docs + AGENTS.md update | `docs/guide.md`, `AGENTS.md` | ~50 |

**Total: ~550 new lines, zero changes to `html!`**

### Phase 2: Manifest + Validation (1-2 weeks)

| Task | File | Lines |
|------|------|-------|
| Manifest write in `routes!` | `macros/src/routes.rs` | +50 |
| Manifest write in `action!` | `macros/src/action.rs` | +30 |
| Route validator | `macros/src/route_validator.rs` (new) | ~200 |
| Integrate into `html!` pipeline | `macros/src/lib.rs` | +5 |
| Build script helper | `src/build_support/routes.rs` (new) | ~50 |
| `href:external` support (already works) | — | 0 |
| Unit tests | `macros/src/route_validator.rs` | ~150 |
| Integration tests | `tests/route_validation_tests.rs` (new) | ~200 |

**Total: ~685 new lines, ~5 lines changed in existing code**

### Phase 3: Auto-registration from `#[azumi::live]` (3-5 days)

| Task | File | Lines |
|------|------|-------|
| Write live routes to manifest | `macros/src/live.rs` | +30 |
| Pattern matching for dynamic segments | `macros/src/route_validator.rs` | +40 |
| Tests | | ~80 |

---

## Usage Examples

### Full type-safe routing

```rust
// ── routes.rs ──────────────────────────────────────────────
azumi::routes! {
    home:         "/"                    => get  home_handler,
    about:        "/about"               => get  about_handler,
    blog_index:   "/blog"                => get  blog_index,
    blog_post:    "/blog/posts/{slug}"   => get  post_handler,
    contact:      "/contact"             => post contact_action,
}

// ── main.rs ────────────────────────────────────────────────
#[tokio::main]
async fn main() {
    let app = routes::router()
        .merge(azumi::action::register_actions(axum::Router::new()))
        .merge(azumi::devtools::router());

    axum::serve(listener, app).await
}

// ── components/nav.rs ──────────────────────────────────────
html! {
    <nav>
        <a href={routes::HOME}>"Home"</a>
        <a href={routes::ABOUT}>"About"</a>
        <a href={routes::BLOG_INDEX}>"Blog"</a>
    </nav>
}

// ── pages/blog.rs ──────────────────────────────────────────
html! {
    // Level 2: string literals validated too
    <a href="/blog/posts/hello-world">"Read more"</a>    // ✓ matches pattern
    <a href="/blog/pots/hello-world">"Read more"</a>     // ❌ unknown route

    // External links bypass validation
    <a href:external="https://rust-lang.org">"Rust"</a>  // ✓ no check
}
```

---

## Why This Design Works

1. **Level 1 requires zero `html!` changes** — just the new `routes!` macro generating constants. Rust's type system does the checking.

2. **Level 2 is opt-in** — if there's no manifest file, validation silently skips. No breakage for existing apps.

3. **Follows Azumi's existing patterns** — `class:external` for CSS opt-out → `href:external` for route opt-out. `inventory` for action registration → manifest file for compile-time access.

4. **Incremental adoption** — use Level 1 (constants) today, get Level 2 (string validation) when ready.

5. **Catches the #1 runtime bug class** — broken links and broken form actions become compile errors. This is what "if it compiles, it works" actually means for a web framework.
