# Azumi Guide

> Complete guide for Azumi — live SSR that builds on Axum. From first component to production deployment.

---

## Table of Contents

1. [Critical Rules](#critical-rules)
2. [Getting Started](#getting-started)
3. [Components](#components)
4. [Control Flow](#control-flow)
5. [Live State & Interactivity](#live-state--interactivity)
6. [Forms & Actions](#forms--actions)
7. [Security](#security)
8. [Client Features](#client-features)
9. [When to Use Azumi](#when-to-use-azumi)
10. [AI Code Generation](#ai-code-generation)

---

## Critical Rules

These are enforced at compile time. Breaking them produces a clear error.

### 1. CSS Values Must Be Double-Quoted

```rust
// ✅ CORRECT
.btn { padding: "1rem"; background: "#4CAF50"; }

// ❌ WRONG
.btn { padding: 1rem; background: #4CAF50; }
```

### 2. CSS Classes: Snake Case + Bracket Syntax

- All CSS classes MUST be `snake_case`. Dashes are **BANNED**.
- Use `class={variable}` — static `class="..."` is **BANNED**.

```rust
html! {
    <div class={my_card}>...</div>       // ✅
    <div class={my_card active}>...</div> // ✅ multiple classes
    <style>
        .my_card { ... }
        .active { ... }
    </style>
}
```

### 3. IDs: Same Rules as Classes

`id="..."` is **BANNED**. Use `id={variable}` with snake_case.

### 4. Inline Styles: Use the Style DSL

```rust
// ✅ CORRECT
<div style={ --color: "red"; --spacing: "1rem" }>...</div>

// ❌ WRONG
<div style="--color: red">...</div>
```

### 5. Text Content Must Be Quoted

```rust
// ✅ CORRECT
<p>"Hello world"</p>
<p>"Count: " {count}</p>

// ❌ WRONG
<p>Hello world</p>
```

### 6. Style Order: HTML First, Style Last

```rust
html! {
    <div class={container}>...</div>
    <style>
        .container { ... }
    </style>
}
```

### 7. Safe Injection Patterns

| Pattern | Purpose | Escapes |
|---------|---------|---------|
| `<style>{CSS_VAR}</style>` | CSS injection | `</style>` (case-insensitive) |
| `<script>{JS_VAR}</script>` | JavaScript injection | `</script>` (case-insensitive) |
| `{json_data!("VAR" = &data)}` | JSON → JavaScript | `</script>` (macro-level) |

**Three Golden Rules for AI-Generated Code:**
1. **NO `Raw()` inside `html!`** — compile error, use safe alternatives
2. **NO `format!` building web content inside `html!`** — compile error, use safe alternatives
3. **ALWAYS use `<style>{var}</style>` / `<script>{var}</script>` for CSS/JS, and `json_data!` for JSON**

---

## Getting Started

### Option 1: Use the CLI (Recommended)

```bash
cargo install azumi-cli
azumi new my-app
cd my-app && cargo run
# → http://localhost:8080
```

### Option 2: Manual Setup

```bash
cargo new my-azumi-app
cd my-azumi-app
cargo add azumi-live-ssr-framework axum tokio serde serde_json tower-http tracing tracing-subscriber
```

**Minimal `main.rs`:**

```rust
use axum::Router;
use azumi::{component, html, routes};

#[component]
pub fn HomePage() -> impl azumi::Component {
    html! {
        <html>
            <head><title>"My App"</title></head>
            <body>
                <h1>"Hello, Azumi!"</h1>
                {azumi::azumi_script()}
            </body>
        </html>
    }
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "devtools")]
    azumi::devtools::auto_reload();

    let app = routes! {
        "/" => home_handler,
    }
    .merge(azumi::action::register_actions(Router::new()))
    .merge(azumi::devtools::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&HomePage))
}
```

**Cargo.toml features:**

```toml
[dependencies]
azumi-live-ssr-framework = { version = "47", features = ["devtools"] }
# Production: remove "devtools" feature
```

---

## Components

### Basic Component

```rust
#[azumi::component]
pub fn MyComponent(title: &str, count: i32) -> impl Component {
    html! {
        <div class={container}>
            <h1 class={title_class}>{title}</h1>
            <p>"Count: " {count}</p>
        </div>
        <style>
            .container { padding: "1rem"; }
            .title_class { font-size: "1.5rem"; color: "#333"; }
        </style>
    }
}
```

### Borrowed Parameters — Zero-Close Props

Use `&str` (or any `&T`) as parameter types to avoid `.clone()` when calling from `render()` methods. The macro automatically injects the required lifetime:

```rust
// No explicit lifetime needed — the macro handles it
#[azumi::component]
pub fn Greeting(name: &str, age: i32) -> impl Component {
    html! { <div>{name} " is " {age}</div> }
}

// Builder accepts &str directly — no .to_string() or .clone()
let comp = Greeting::render(
    Greeting::Props::builder().name("Alice").age(30).build().unwrap(),
);
```

**Why this matters**: When rendering from `&self`, owned parameters force `.clone()`. Borrowed parameters accept references directly:

```rust
// ❌ Old way: owned params force clone
fn render(&self, f: &mut Formatter) -> fmt::Result {
    html! { {MyComponent::render(MyComponent::Props::builder()
        .title(self.title.clone())  // allocates!
        .build().unwrap())} }
}

// ✅ New way: &str params accept references
fn render(&self, f: &mut Formatter) -> fmt::Result {
    html! { {MyComponent::render(MyComponent::Props::builder()
        .title(&self.title)  // borrows — zero alloc
        .build().unwrap())} }
}
```

**Rules**:
- `&str` / `&T` without explicit lifetime → macro injects `'a` automatically
- `&'a str` / `&'static str` with explicit lifetime → used as-is (backward compatible)
- `String`, `i32`, etc. → owned, no lifetime injection
- `#[prop(default = "...")]` on `&str` → default must be `&'static str` (e.g., `"N/A"`)
- Mixed borrowed + owned parameters work naturally

### Component with Children

```rust
#[azumi::component]
pub fn Container(children: impl Component) -> impl Component {
    html! {
        <div class={container}>{children}</div>
        <style>.container { padding: "1rem"; }</style>
    }
}

// Usage
@Container {
    <p>"Content inside container"</p>
}
```

### Layout with Client Runtime

```rust
#[azumi::component]
pub fn RootLayout(children: impl Component) -> impl Component {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <title>"My Azumi App"</title>
            </head>
            <body>
                {children}
                {azumi::azumi_script()}
            </body>
        </html>
    }
}
```

**Why manual?** Static pages stay at 0KB JS. Interactive pages get only what they need (~11KB gzipped).

---

## Control Flow

### @let — Local Variables

```rust
html! {
    @let name = "Azumi";
    <p>"Hello, " {name} "!"</p>

    @let items = vec!["Item 1", "Item 2"];
    @let count = items.len();
    <p>"Total: " {count}</p>
}
```

### @if — Conditionals

```rust
html! {
    @if state.active {
        <div>"Active"</div>
    } else {
        <div>"Inactive"</div>
    }

    // Conditional class
    <div class={if state.active { "active" } else { "inactive" }}>
        "Content"
    </div>
}
```

### @for — Loops

```rust
html! {
    @for item in &state.items {
        <div>{item}</div>
    }
    @for (i, item) in state.items.iter().enumerate() {
        <div>{i + 1}". " {item}</div>
    }
}
```

### @for with @keyed — Keyed List Updates

Add `@keyed(expr)` to give each list item a stable identity:

```rust
html! {
    // Without @keyed: every list change replaces all DOM nodes
    // With @keyed: items tracked by identity, smooth reordering
    @for item in &items @keyed(item.id) {
        <div class="row">{&item.name}</div>
    }
}
// Renders: <div data-key="1" class="row">...</div>
```

**Use @keyed for:** lists that reorder, have interactive children, or animate.
**Skip @keyed for:** static lists or simple text-only lists.

### @match — Pattern Matching

```rust
html! {
    @match status {
        "loading" => { <p>"Loading..."</p> }
        "success" => { <p>"Done!"</p> }
        "error" => { <p>"Failed"</p> }
        _ => { <p>"Unknown"</p> }
    }
}
```

---

## Live State & Interactivity

### State Definition

```rust
#[azumi::live]
pub struct Counter {
    pub count: i32,
    pub liked: bool,
}
```

### Methods with Predictions

```rust
#[azumi::live]
impl Counter {
    pub fn increment(&mut self) { self.count += 1; }
    pub fn toggle_like(&mut self) { self.liked = !self.liked; }
    pub fn reset(&mut self) { self.count = 0; }
}
```

The compiler auto-detects simple mutations (`+1`, `-1`, `!field`, `.push()`, `.clear()`) and generates optimistic predictions. No JavaScript required.

### Component View

```rust
#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <div class={counter}>
            <div class={value}>{state.count}</div>
            <button class={btn} on:click={state.increment}>"+1"</button>
            <button class={btn} on:click={state.toggle_like}>
                {if state.liked { "❤️" } else { "🤍" }}
            </button>
        </div>
        <style>
            .counter { text-align: "center"; padding: "2rem"; }
            .value { font-size: "3rem"; margin: "1rem 0"; }
            .btn { padding: "1rem 2rem"; cursor: "pointer"; }
        </style>
    }
}
```

### Async Database Integration

```rust
impl Counter {
    pub async fn save_to_db(&mut self) {
        // 1. Optimistic update (instant)
        let count = self.count;

        // 2. Real async DB operation
        sqlx::query("UPDATE counters SET count = ?")
            .bind(count)
            .execute(&*POOL)
            .await
            .unwrap();

        // 3. DOM updates via Idiomorph when server responds
    }
}
```

### How It Works

```
1. INITIAL PAGE LOAD
   Handler → Render component → HTML with az-scope attribute
   <div az-scope="{signed_json}" az-struct="Counter">...</div>

2. USER CLICKS BUTTON
   Client: Auto-detect prediction from az-predictions JSON
           Apply "count = count + 1" instantly (0ms latency)
   Client: POST /azumi/action/Counter/increment
           Body: {signed_state_json} (original, pre-prediction)

3. SERVER HANDLER
   verify_state(body) → Extract JSON (or 400 Bad Request)
   serde_json::from_str(&json) → State (or 500 Error)
   state.increment() → Mutate state
   Component::render(props) → Return new HTML

4. CLIENT UPDATE
   Idiomorph patches DOM with returned HTML
   If prediction was wrong, server HTML wins (reconciliation)
```

---

## Forms & Actions

### Form with az-action

```rust
html! {
    <form az-action="save_settings" az-target="#result" az-swap="morph">
        <input name="theme" type="text">
        <button type="submit">Save</button>
    </form>
    <div id="result"></div>
}
```

**Server handler:**

```rust
async fn save_settings(/* ... */) -> impl axum::response::IntoResponse {
    azumi::action::success_fragment("<p>Settings saved!</p>")
}
```

### Two-Way Binding with `bind:value`

Sync input values with state automatically. No manual `on:input` handlers needed. 200ms debounce by default.

```rust
html! {
    <form az-action="signup" az-target="#result">
        <input type="text" name="name"
               bind:value={state.name}
               data-validate="name:required,min-length:2" />
        <p id="name_error" class="form-error"></p>

        <input type="email" name="email"
               bind:value={state.email}
               data-validate="email:required,email" />
        <p id="email_error" class="form-error"></p>

        <input type="checkbox"
               bind:checked={state.agree} />
        <label>"I agree"</label>

        <select bind:value={state.plan}>
            <option value="free">"Free"</option>
            <option value="pro">"Pro"</option>
        </select>

        <button type="submit">"Sign Up"</button>
    </form>
}
```

**Supported elements:** `<input>` (text, email, password, number, etc.), `<input type="checkbox">` (use `bind:checked`), `<input type="radio">`, `<select>`, `<textarea>`.

**Nested fields:** `bind:value={state.user.name}` syncs into `state.user.name`.

**Renders as:** `data-bind-value="field.path"` in the HTML.

### Success/Error Fragments

```rust
use azumi::action::{success_fragment, error_fragment};

// Success — wraps in <div class="success_message">
success_fragment("<p>Saved!</p>")

// Error — wraps in <div class="error_message"> with optional retry button
error_fragment("Invalid email", None)
error_fragment("Upload failed", Some("my_form")) // includes "Try Again" button
```

---

## Security

### HMAC State Signing

Every state is signed to prevent tampering:

```rust
let signed = "{json}|{timestamp}|{signature_base64}";
// Server verifies:
// ✓ HMAC valid (not tampered)
// ✓ Timestamp fresh (< 1 hour)
// ✓ User-scoped (optional, prevents cross-user replay)
```

### Protection Matrix

| Threat | Azumi Protection |
|--------|-----------------|
| XSS | All text escaped via `Escaped<T>` wrapper |
| CSS Injection | Semicolons/braces escaped |
| State Tampering | HMAC-SHA256 signature |
| Replay Prevention | 1-hour timestamp + user scoping |
| Content Injection | Content-Security-Policy builder |

**Important:** Signed state prevents tampering, but **authorization is your responsibility**. Any user with valid signed state CAN trigger any action. Add authorization checks in your action methods.

### Content-Security-Policy Builder

Azumi provides a CSP builder that generates headers compatible with its architecture:

```rust
use azumi::csp::ContentSecurityPolicy;

// Recommended defaults for Azumi apps (uses 'unsafe-inline' for styles)
let csp = ContentSecurityPolicy::azumi_defaults().build();
// Produces:
// default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline';
// img-src 'self' data:; form-action 'self'; base-uri 'self'; frame-ancestors 'none'

// Customize for your app (e.g., WebSocket dev server, external fonts)
let csp = ContentSecurityPolicy::azumi_defaults()
    .connect_src("'self' ws://localhost:8080")
    .font_src("'self' https://fonts.gstatic.com")
    .upgrade_insecure_requests()
    .build();
```

**Why `style-src` includes `'unsafe-inline'`:** Azumi uses scoped `<style>` blocks in HTML for zero-JS CSS. Nonce-based CSP requires server-side nonce injection on every render, which contradicts Azumi's static-HTML-first model. If your threat model demands nonce-based CSP, you can remove `'unsafe-inline'` and add a nonce system.

### Nonce-based CSP (Stronger XSS Protection)

For applications that need to eliminate `'unsafe-inline'`, Azumi provides per-request nonce generation:

```rust
use azumi::csp::{CspNonce, ContentSecurityPolicy};

// Manual usage — generate nonce and build CSP
let nonce = CspNonce::generate();
let csp = ContentSecurityPolicy::azumi_nonce_defaults(&nonce).build();
// Produces:
// default-src 'self'; script-src 'self' 'nonce-abc123'; style-src 'self' 'nonce-abc123'; ...

// For environments where the system RNG may be unavailable (embedded, sandboxed):
let nonce = CspNonce::try_generate().expect("RNG unavailable");
```

**`CspNonce::generate()` vs `try_generate()`:** `generate()` panics if the system RNG is unavailable — this is deliberate, since CSP without a nonce is a security downgrade. Use `try_generate()` in environments where you need graceful degradation (e.g., early boot, restricted sandboxes).

// Use nonce in html! — <style nonce={nonce.as_str()}>
html! {
    <style nonce={nonce.as_str()}>
        .my_class { color: "red"; }
    </style>
}
```

**Axum middleware** — auto-generates nonce and sets CSP header per request:

```rust
use azumi::csp::csp_nonce_layer;

let app = Router::new()
    .route("/", home_handler)
    .layer(csp_nonce_layer());

// In your handler, extract the nonce:
async fn home_handler(nonce: CspNonce) -> impl IntoResponse {
    let csp = ContentSecurityPolicy::azumi_nonce_defaults(&nonce).build();
    // nonce.as_str() available for <style> and <script> tags
    (
        [("content-security-policy", csp)],
        axum::response::Html(body)
    )
}
```

Custom CSP policy with nonce middleware:

```rust
use azumi::csp::csp_nonce_layer_with;

let app = Router::new()
    .layer(csp_nonce_layer_with(|nonce| {
        ContentSecurityPolicy::azumi_nonce_defaults(nonce)
            .connect_src("'self' ws://localhost:8080")
    }));
```

### Streaming Render

For high-throughput servers, `render_to_writer()` writes directly to any `std::io::Write` implementation, avoiding the intermediate `String` allocation:

```rust
use azumi::render_to_writer;

// Write directly to an Axum response body
let mut body = Vec::new();
render_to_writer(&my_component, &mut body)?;
let html_bytes = axum::body::Bytes::from(body);
```

Use `render_to_writer()` when serving many concurrent requests or rendering large pages. Use `render_to_string()` for simplicity when performance isn't critical.

`render_to_writer()` returns `std::io::Result<()>`, so use `?` to propagate errors.

### Server-Sent Events (SSE)

Azumi provides `SseEvent` for streaming HTML fragments or JSON to connected clients:

```rust
use azumi::streaming::{sse, SseEvent};
use std::time::Duration;
use tokio::time::interval;

async fn notifications() -> impl axum::response::IntoResponse {
    let stream = async_stream::stream! {
        let mut ticker = interval(Duration::from_secs(5));
        loop {
            ticker.tick().await;
            yield SseEvent::fragment(html! {
                <div class="notification">"New message received"</div>
            });
        }
    };
    sse(stream)
}
```

**Event constructors:**
- `SseEvent::fragment(component)` — HTML fragment (event name: `"fragment"`)
- `SseEvent::json(&data)` — JSON payload (event name: `"json"`)
- `SseEvent::heartbeat()` — keep-alive ping (event name: `"ping"`)

**Builder methods:**
- `.id("42")` — set event ID for replay/resume support
- `.name("custom")` — override the event name

**Accessors:**
- `.event()` — get the event name
- `.data()` — get the data payload
- `.get_id()` — get the event ID (returns `Option<&str>`)

---

## Client Features

All interactivity is handled by `az-*` directives on HTML elements. No custom JavaScript required.

### Feature Catalog

| Feature | Attribute | What It Does |
|---|---|---|
| Form actions | `az-action` + `az-target` | Submit form, morph result HTML into target |
| Client state | `az-ui` + `az-on` | Tabs, toggles, counters — no server roundtrip |
| Event handlers | `az-on:click`, `az-on:change`, etc. | Declarative event binding |
| Dynamic text | `az-bind:text` | Live text updates from state |
| Conditional classes | `az-bind:class` | Dynamic styling based on state |
| Confirmation | `az-confirm` | "Are you sure?" before action |
| Auto-init | `az-init` | Run action on page load |
| Transitions | `az-transition:fade`, `:slide`, `:scale` | Smooth enter/exit animations on DOM changes |
| Scroll reveal | `az-reveal` | Fade in on scroll |
| Two-way binding | `bind:value`, `bind:checked` | Sync input ↔ state automatically |
| Keyed lists | `@keyed(id)` | Stable list item identity for smooth updates |
| Form validation | `data-validate` | Client-side validation (8 rules) |
| Scroll reveal | `az-reveal` | Animate elements on scroll into view |
| Scroll top | `scroll-top` | Smooth scroll to top |

### Transitions

Smooth animations on DOM enter/exit. Works automatically after `az-action` responses.

```rust
html! {
    // Fade in when element appears, fade out when removed
    <div az-transition:fade={"true"}>"I fade smoothly"</div>

    // Slide open/closed
    <div az-transition:slide={"true"}>"I slide in"</div>

    // Scale 0.95→1.0 with opacity
    <div az-transition:scale={"true"}>"I scale up"</div>

    // Custom duration (default: 200ms)
    <div az-transition:fade={"duration=500"}>"Slow fade"</div>
}
```

**Supported: `fade`, `slide`, `scale`.** Works with `@keyed` — reordered items reposition without exiting/entering.

### Scroll Reveal

**Tabs:**
```rust
html! {
    <div az-ui="tab:content" class="tabs">
        <button az-on:click="set:tab:overview" class="tab_active">"Overview"</button>
        <button az-on:click="set:tab:pricing" class="tab">"Pricing"</button>
        @if tab == "overview" {
            <div>{overview}</div>
        }
        @if tab == "pricing" {
            <div>{pricing}</div>
        }
    </div>
}
```

**Accordion:**
```rust
html! {
    <div az-ui="toggle:section1" class="accordion">
        <button az-on:click="toggle:section1">"Section 1"</button>
        @if section1 {
            <div class="panel">"Content here"</div>
        }
    </div>
}
```

**Confirm dialog:**
```rust
html! {
    <form az-action={delete_item_PATH} az-confirm="Delete this item?">
        <button type="submit">"Delete"</button>
    </form>
}
```

**Scroll reveal:**
```rust
html! {
    <section az-reveal={true} class="hero">
        <h2>"Appears on scroll"</h2>
    </section>
}
```

### What Still Needs Custom JS

Azumi covers the common interactive patterns. These still require custom JavaScript:
- Complex drag-and-drop reordering
- Canvas / WebGL / charting libraries
- Third-party integrations that require JS SDKs (payments, maps)
- Real-time collaborative editing

For these, use `<script src="...">` to load external JS. Azumi's ~11KB (gzipped) runtime coexists with custom JS when needed.

---

## When to Use Azumi

### Azumi Wins

- **Reusable component library** — `ProductCard`, `NavBar`, etc.
- **Live state** — forms, counters, toggles
- **Multiple developers** — compile-time enforcement
- **Production security** — XSS prevention, HMAC signing

### Simpler Approaches Win

- **One-off static pages** — `format!()` strings or Maud
- **No shared components** — copy-paste is fine
- **Team unfamiliar with Rust macros** — learning curve exists

### Hybrid Approach (Recommended)

```rust
// SIMPLE PAGE: Use format!() strings
pub fn landing_page() -> String {
    format!(r#"<h1>Hello</h1>"#)
}

// SHARED COMPONENTS: Use Azumi
#[azumi::component]
pub fn ProductCard(product: &Product) -> impl Component {
    html! { /* ... */ }
}
```

| Use Case | Recommendation |
|----------|----------------|
| Blog, docs, marketing | Maud or `format!()` |
| SaaS app with shared components | **Azumi** |
| E-commerce | **Azumi** |
| Admin dashboard | **Azumi** |
| One-off landing page | `format!()` |

---

## AI Code Generation

Azumi is designed for AI-assisted development. The validation pipeline guides AI-generated code toward correct patterns:

1. **CSS validation** — property names and syntax
2. **Node order** — Script → Content → Style
3. **Raw usage** — blocks ALL `Raw()` in html!
4. **Format detection** — blocks `format!` with HTML/CSS/JS/DOM patterns
5. **Class/ID validation** — classes must be defined in `<style>` blocks
6. **HTML structure** — tables, lists, forms, buttons, headings
7. **Attribute validation** — valid HTML5, `data-*`, `aria-*`, `on:*` events

### Key Constants for AI Assistants

- `azumi::AZUMI_RULES` — array of rule strings
- `azumi::AZUMI_AI_HASH` — hash of AI rules for integrity verification

### Escape Hatches (Never Use in App Code)

| Type | Visibility | When to Use |
|------|-----------|-------------|
| `from_fn` | `#[doc(hidden)]` | Internal macro expansion only |
| `from_fn_once` | `#[doc(hidden)]` | Internal macro expansion only |
| `Raw<T>` | `#[doc(hidden)]` | Internal framework SEO only |
| `TrustedHtml` | **Public** | Pre-sanitized HTML from trusted sources (CMS, markdown renderer) |

---

## Features

| Feature | Flag | Description |
|---------|------|-------------|
| `devtools` | Optional | Hot reload, file watcher, CSS patching |
| `schema` | Optional | Schema.org JSON-LD derive macro |
| `test-utils` | Optional | `azumi::test` module for testing |

---

## Debugging

```bash
# Run the demo with 20 lessons
cd azumi && cargo run -p azumi-demo
# Visit: http://localhost:8080

# Debug macro expansion
AZUMI_DEBUG=1 cargo build
```

---

## Production Deployment

```bash
# Development (with hot reload)
cargo run --features azumi-live-ssr-framework/devtools

# Production (no devtools, set secret)
AZUMI_SECRET="your-64-char-random-secret" cargo run --release
```
