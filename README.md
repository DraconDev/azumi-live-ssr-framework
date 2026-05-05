# Azumi

> **The framework that catches your CSS typos before your users do.**

```rust
html! {
    <div class={my_buttn}>  // ❌ COMPILE ERROR: 'my_buttn' not found. Did you mean 'my_button'?
        "Click me"
    </div>
    <style>
        .my_button { background: "#3b82f6"; }
    </style>
}
```

**Azumi** is an **AI-first, compile-time validated web framework** for Rust. Your CSS classes, HTML structure, accessibility, and UI logic — all verified before a single byte hits production. Built for AI code generation first, with named macros that are explicit, searchable, and unambiguous.

No runtime errors. No "works on my machine". No surprises.

---

## 🚀 v42.0.0 Release Notes

**Azumi v42.0.0** — Safe injection macros (json_data!, inline_css!, inline_script!), unconditional Raw() blocking, ~1,400 tests.

### Breaking Changes
- **Raw() is unconditionally blocked** — ALL usages inside `html!` are compile errors. Use safe injection macros instead.
- **format! with web patterns blocked** — `format!("<div>{}</div>", x)` inside html! is a compile error.

### What's New
- **json_data!("VAR" = &data)** — Safe JSON data injection for JavaScript, escapes `</script>` (case-insensitive)
- **inline_css!(CSS_VAR)** — Safe CSS injection, escapes `</style>` (case-insensitive)
- **inline_script!(JS_VAR)** — Safe JavaScript injection, escapes `</script>` (case-insensitive)
- **escape_style_content()** — New function for CSS escaping
- **AZUMI_RULES** — Framework rules array for AI verification

### Migration from v41.x
- `Raw()` anywhere in `html!` → Use `json_data!`, `inline_css!`, or `inline_script!` macros
- `format!()` building HTML/CSS/JS inside `html!` → Build outside html! and pass as variable
- See [AGENTS.md](AGENTS.md) for AI code generation guidelines

### Migration from v26.x → v28
- `#[azumi::live]` + `#[azumi::live_impl]` now required together for predictions
- Predictions auto-detected from `#[azumi::live_impl]` — manual `data-predict` is optional
- See [MIGRATION.md](MIGRATION.md) for full v26 → v27+ upgrade guide

### Migration from v14.x–v25
- `Raw("window.location.hash...")` → Use `{session_cleanup_script()}`
- All framework Components use `{}` syntax, not `@{Raw(...)}`
- `#[azumi::page]` replaces manual SEO setup
- `<script src="azumi.js" />` → Use `{azumi_script()}` (macro transformation removed)
- `AZUMI_SECRET` still required in production

### Safe Injection Macros

| Macro | Purpose | Escapes |
|-------|---------|---------|
| `json_data!("VAR" = &data)` | JSON → JavaScript | `</script>` (case-insensitive) |
| `inline_css!(CSS_VAR)` | CSS injection | `</style>` (case-insensitive) |
| `inline_script!(JS_VAR)` | JavaScript injection | `</script>` (case-insensitive) |

**Escape hatches (`#[doc(hidden)]` — internal framework use only):**
- `Raw()`, `TrustedHtml`, `from_fn()`, `from_fn_once()` — never use these in application code

---

## 🛠️ Development Experience

Azumi includes a built-in hot reload system. Just add one line:

```rust
fn main() {
    azumi::devtools::auto_reload(); // ⚡ That's it!
}
```

- **CSS**: Instant, zero-reload updates via WebSocket.
- **HTML**: Sub-second template patching.
- **Logic**: Auto-restart on Rust code changes.

## ⚡ The Pitch

Traditional frameworks let bugs slip through to production. Azumi doesn't.

| Other Frameworks              | Azumi                                  |
| ----------------------------- | -------------------------------------- |
| CSS typo → silent fail        | CSS typo → **compile error**           |
| Missing class → invisible bug | Missing class → **compile error**      |
| Invalid HTML → maybe works?   | Invalid HTML → **compile error**       |
| Click handler typo → 💀       | Click handler typo → **compile error** |
| Nested forms → broken submit  | Nested forms → **compile error**       |
| Missing alt text → a11y fail  | Missing alt text → **compile error**   |

**Everything happens at compile time.** Your IDE shows errors before you save. Your CI fails before it deploys. Your users never see a broken page.

---

## 🚀 Features

### 1. CSS-HTML Co-Validation (Industry First)

The compiler knows your styles. Use a class that doesn't exist? Error. Typo in a class name? Error.

```rust
html! {
    <div class={my_buttn}>  // ❌ "my_buttn" not found. Did you mean "my_button"?
        "Oops"
    </div>
    <style>
        .my_button { background: "#3b82f6"; }
    </style>
}
```

### 2. HTML Structure Validation

Invalid HTML nesting caught at compile time:
- Tables can only contain valid children (`tr`, `thead`, `tbody`, etc.)
- Lists can only contain `<li>`
- Forms cannot be nested
- `<p>` cannot contain block-level elements
- `<a>` cannot contain other `<a>` tags
- Headings can only contain phrasing content
- Tag name validation (unknown tags flagged)

### 3. Accessibility Validation

Built-in a11y checks at compile time:
- `<img>` must have `alt` attribute
- `<input>` must have valid `type`
- ARIA roles must be valid
- Buttons must have accessible content
- `<a target="_blank">` must have `rel="noopener noreferrer"`
- `<iframe>` must have `title`

### 4. Optimistic UI (Auto-Detected Predictions)

Write Rust. Get instant UI. No JavaScript required.

**Auto-detection:** `#[azumi::live_impl]` analyzes your methods and auto-detects predictions. The client executes them optimistically when buttons are clicked.

```rust
#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) { self.count += 1; }
    pub fn toggle(&mut self) { self.active = !self.active; }
}

#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        // Predictions are auto-detected from #[azumi::live_impl]!
        // No data-predict needed for simple mutations
        <button on:click={state.increment}>"+1"</button>
        <button on:click={state.toggle}>"Toggle"</button>
    }
}
```

**Manual override:** Add `data-predict` attributes for custom predictions or complex mutations:

```rust
// Manual prediction for complex cases
<button on:click={state.reset} data-predict="count = 0">"Reset"</button>
```

**Supported prediction patterns:**
- `"field = value"` — Set a field
- `"field = !field"` — Toggle a boolean
- `"field = field + value"` — Increment
- `"field = field - value"` — Decrement
- `"field.push(value)"` — Add to vector
- `"field = []"` — Clear vector

### 5. Signed State (Anti-Tampering)

Every component's state is HMAC-signed. Users can't forge state, but **authorization is your responsibility**.

```rust
// User tries to edit az-scope JSON in DevTools...
// → 400 Bad Request. Automatically. No code needed.

// But any user with a valid signed state CAN trigger any action.
// YOU must add authorization checks in your action methods.
```

### 6. Automatic SEO

Azumi infers metadata directly from your Rust code.

```rust
/// About Us
/// We are a team of passionate developers...
#[azumi::page]
pub fn about_us() -> impl Component { ... }
```

Generates `<title>`, `<meta description>`, OpenGraph tags, and Twitter cards automatically.

### 7. Production-Ready Asset Pipeline

- **Content-hashed filenames** → Immutable caching (1 year)
- **Automatic path rewriting** → Write `/static/logo.png`, get `/assets/logo.a8f3c2.png`
- **CSS minification** → Zero config via `lightningcss`

---

## 🚨 Critical Rules

### 1. CSS Values Must Be Double-Quoted

```rust
// ✅ CORRECT
.btn { padding: "1rem"; background: "#4CAF50"; }

// ❌ WRONG
.btn { padding: 1rem; background: #4CAF50; }
```

### 2. CSS Classes: Snake Case + Bracket Syntax

- All CSS classes MUST be `snake_case`. Dashes are **BANNED** in class names.
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

### 5. Live State Requires Component Link

```rust
#[azumi::live]
pub struct Counter { pub count: i32 }

#[azumi::live_impl(component = "counter_view")]  // ← Required!
impl Counter {
    pub fn increment(&mut self) { self.count += 1; }
}

#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! { /* ... */ }
}
```

### 6. Event Binding: Direct Method References

```rust
// ✅ CORRECT
<button on:click={state.increment}>"Click"</button>

// ❌ WRONG — no closures, no function calls
<button on:click={|| state.increment()}>
<button on:click={state.increment()}>
```

### 7. Text Content Must Be Quoted

```rust
// ✅ CORRECT
<p>"Hello world"</p>
<p>"Count: " {count}</p>

// ❌ WRONG
<p>Hello world</p>
```

### 8. Style Order: HTML First, Style Last

```rust
html! {
    <div class={container}>...</div>
    <style>
        .container { ... }
    </style>
}
```

---

## 🚀 Setup

### Injecting Client Runtime

Azumi is **static by default**. Pages render as pure HTML with zero JavaScript.

To enable interactivity, include the runtime in your root layout:

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
                {azumi_script()}
            </body>
        </html>
    }
}
```

**Why manual?** Static pages stay at 0KB JS. Interactive pages get only what they need (~3KB).

---

## 📦 Features

| Feature | Flag | Description |
|---------|------|-------------|
| `devtools` | Optional | Hot reload, file watcher, CSS patching |
| `schema` | Optional | Schema.org JSON-LD derive macro |
| `test-utils` | Optional | `azumi::test` module for testing |

Production builds (no devtools):
```toml
azumi = { version = "34.5.4" }
```
...
azumi = { version = "30.3.1", features = ["devtools"] }
```

---

## 🏗️ Component Fundamentals

### Basic Component

```rust
#[azumi::component]
pub fn MyComponent(title: &str, count: i32) -> impl Component {
    html! {
        <div class={container}>
            <h1 class={title}>{title}</h1>
            <p>"Count: " {count}</p>
        </div>
        <style>
            .container { padding: "1rem"; }
            .title { font-size: "1.5rem"; color: "#333"; }
        </style>
    }
}
```

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

---

## 🧮 Control Flow

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

## ⚡ Live Interactive Components

### State Definition

```rust
#[azumi::live]
pub struct TodoList {
    pub todos: Vec<TodoItem>,
    pub input: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TodoItem {
    pub id: i64,
    pub text: String,
    pub completed: bool,
}
```

### Methods with Predictions

```rust
#[azumi::live_impl(component = "todo_view")]
impl TodoList {
    pub fn add(&mut self) {
        self.todos.push(TodoItem { id: -1, text: self.input.clone(), completed: false });
        self.input.clear();
    }
    pub fn toggle(&mut self) { /* ... */ }
    pub fn clear(&mut self) { self.todos.clear(); }
}
```

### Async Database Integration

```rust
impl TodoList {
    pub async fn add_todo(&mut self) {
        // 1. Optimistic update (instant)
        self.todos.push(TodoItem { id: -1, text: self.input.clone(), completed: false });
        let text = self.input.clone();
        self.input.clear();

        // 2. Real async DB operation
        sqlx::query("INSERT INTO todos (text) VALUES (?)")
            .bind(text)
            .execute(&*POOL)
            .await
            .unwrap();

        // 3. Re-fetch for consistency
        self.refresh_from_db().await;
    }
}
```

---

## 🔧 Debugging

### Development Server

```bash
cd demo && cargo run
# Visit http://localhost:8080
```

### Debug Macro Expansion

Set `AZUMI_DEBUG=1` to see detailed parsing output during compilation.

---

## 📊 Performance

| Metric | Azumi | React | Vue | Svelte |
|--------|-------|-------|-----|--------|
| **Bundle Size** | ~3KB | 100KB+ | 95KB+ | 50KB |
| **Hydration** | None | Required | Required | Required |
| **CSS Validation** | Compile-time | Runtime | Runtime | Runtime |
| **HTML Validation** | Compile-time | Runtime | Runtime | Runtime |
| **Type Safety** | Full Rust | TypeScript | TypeScript | TypeScript |

---

## 🏢 When to Use Azumi

> **Confused about whether Azumi is right for your project?** See [WHEN_TO_USE_AZUMI.md](./WHEN_TO_USE_AZUMI.md) for a detailed comparison with simpler approaches.

### ✅ Excels At

- **Correctness** — CSS-HTML co-validation, accessibility checks, end-to-end type safety
- **Performance** — Zero hydration, ~3KB runtime, instant TTI
- **Developer Experience** — No API layer, browserless testing, built-in asset pipeline
- **Production** — Low memory footprint, signed state, edge-cacheable
- **Reusable components** — Build once, use everywhere with compile-time validation

### 🎯 Outside Core Scope

- **Offline-first apps** — Requires separate sync layer
- **Real-time collaboration** — Pair with dedicated real-time layer
- **Mobile native** — Wrap in Tauri/Capacitor for native distribution
- **One-off static pages** — Consider `format!()` strings or Maud for simple pages

### The Honest Take

Azumi trades verbosity for safety. If you need:
- A **component library** (ProductCard, NavBar, etc.) → Azumi wins
- **HMAC state signing** and XSS protection → Azumi wins
- **Quick one-off pages** with no reuse → simpler approaches may win

Azumi is opinionated: logic runs on the server, UI updates are predicted, the server is truth.

---

## 📁 Project Structure

```
azumi/
├── src/                      # Core framework
│   ├── lib.rs               # Component trait, LiveState, CSS scoping
│   ├── security.rs          # HMAC-SHA256 state signing
│   ├── action.rs            # Server action registry
│   ├── devtools.rs          # Hot reload (feature-gated)
│   ├── hot_reload.rs        # WebSocket + template updates
│   ├── seo.rs               # SEO metadata generation
│   └── script.rs            # AzumiScript component
├── macros/                   # Procedural macros
│   └── src/
│       ├── lib.rs           # html! macro expansion
│       ├── token_parser.rs  # HTML/CSS/JSX-like DSL parser
│       ├── component.rs     # #[azumi::component]
│       ├── live.rs          # #[azumi::live] + prediction analysis
│       ├── style.rs         # CSS DSL + lightningcss validation
│       ├── css_validator.rs # Compile-time CSS rules
│       ├── accessibility_validator.rs  # A11y checks
│       ├── html_structure_validator.rs # HTML nesting rules
│       └── asset_rewriter.rs           # Asset path rewriting
├── demo/                     # Interactive learning platform
│   └── src/examples/lessons/ # 20 progressive lessons
└── build.rs                  # Asset hashing + manifest generation
```

---

## 🚀 Quick Start

### 1. Create a Component

```rust
use azumi::prelude::*;

#[azumi::component]
pub fn WelcomeCard(name: &str) -> impl Component {
    html! {
        <div class={welcome_card}>
            <h2 class={title}>"Welcome to Azumi!"</h2>
            <p>"Hello, " {name}</p>
        </div>
        <style>
            .welcome_card { padding: "1.5rem"; background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)"; color: "white"; border-radius: "12px"; }
            .title { font-size: "1.5rem"; font-weight: "bold"; }
        </style>
    }
}
```

### 2. Add Interactivity

```rust
use azumi::prelude::*;

#[azumi::live]
pub struct Counter { pub count: i32, pub liked: bool }

#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) { self.count += 1; }
    pub fn toggle_like(&mut self) { self.liked = !self.liked; }
}

#[azumi::component]
pub fn counter_view<'a>(state: &'a Counter) -> impl Component + 'a {
    html! {
        <div class={counter}>
            <div class={value}>{state.count}</div>
            <button class={btn} on:click={state.increment}>
                {if state.liked { "❤️" } else { "🤍" }}
            </button>
        </div>
        <style>
            .counter { text-align: "center"; padding: "2rem"; }
            .value { font-size: "3rem"; margin: "1rem 0"; }
            .btn { padding: "1rem 2rem"; margin: "0.5rem"; cursor: "pointer"; }
        </style>
    }
}
```

---

## 🤝 Getting Started

### Option 1: Try the Learning Platform

```bash
git clone https://github.com/DraconDev/azumi
cd azumi
cargo run -p azumi-demo
# Visit: http://localhost:8080
```

20 interactive lessons covering components through async database integration.

### Option 2: Create a New Project

```bash
cargo new my-azumi-app
cd my-azumi-app
cargo add azumi
```

### Option 3: Complete Setup (Minimal Example)

Here's a complete `main.rs` showing how to wire Azumi with Axum:

```rust
use axum::{routing::get, Router};
use azumi::{html, component};

#[azumi::component]
pub fn HomePage() -> impl azumi::Component {
    html! {
        <html>
            <head>
                <title>"My Azumi App"</title>
            </head>
            <body>
                <h1>"Hello, Azumi!"</h1>
                <p>"Welcome to the framework that catches your CSS typos."</p>
                {azumi_script()}
            </body>
        </html>
    }
}

#[tokio::main]
async fn main() {
    // Optional: Enable hot reload in development
    azumi::devtools::auto_reload();

    let app = Router::new()
        .route("/", get(|| async {
            axum::response::Html(azumi::render_to_string(&HomePage()))
        }))
        // Register Azumi actions (for interactive components)
        .merge(azumi::action::register_actions(axum::Router::new()))
        // Optional: Enable hot reload endpoints
        .merge(azumi::devtools::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server running at http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
```

#### What Each Part Does

| Component | Purpose |
|-----------|---------|
| `azumi::devtools::auto_reload()` | Sub-second CSS hot reload in development |
| `azumi::action::register_actions(...)` | Registers interactive endpoints |
| `azumi::devtools::router()` | WebSocket endpoint for hot reload |
| `{azumi_script()}` | Client runtime (~3KB) for optimistic UI |

#### Production Deployment

For production, omit devtools and set the secret:

```bash
AZUMI_SECRET="your-64-char-random-secret" cargo run --release
```

---

## 🔄 How Live Components Work

When a user clicks a button, here's what happens:

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. INITIAL PAGE LOAD                                           │
├─────────────────────────────────────────────────────────────────┤
│ Handler → Render component → HTML with az-scope attribute      │
│ <div az-scope="{json}" az-struct="Counter">...</div>          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. USER CLICKS BUTTON                                          │
├─────────────────────────────────────────────────────────────────┤
│ Client: Auto-detect prediction from az-predictions JSON       │
│         (or use manual data-predict override)                   │
│         Apply "count = count + 1" instantly (0ms latency)       │
│ Client: POST /_azumi/action/Counter/increment                  │
│         Body: {signed_state_json} (original, pre-prediction)   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. SERVER HANDLER (Generated by #[azumi::live_impl])          │
├─────────────────────────────────────────────────────────────────┤
│ verify_state(body) → Extract JSON (or 400 Bad Request)        │
│ serde_json::from_str(&json) → State (or 500 Error)            │
│ state.increment() → Mutate state                                │
│ Component::render(props) → Return new HTML                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. CLIENT UPDATE                                               │
├─────────────────────────────────────────────────────────────────┤
│ Idiomorph patches DOM with returned HTML                         │
│ If prediction was wrong, server HTML wins (reconciliation)      │
└─────────────────────────────────────────────────────────────────┘
```

### HMAC State Signing

Every state is signed to prevent tampering:

```rust
let signed = "{json}|{timestamp}|{signature_base64}";
// Server verifies:
// ✓ HMAC valid (not tampered)
// ✓ Timestamp fresh (< 1 hour)
// ✓ User-scoped (optional, prevents cross-user replay)
```

### Security Properties

| Protection | How |
|------------|-----|
| State integrity | HMAC-SHA256 signature |
| Replay prevention | 1-hour timestamp + user scoping |
| XSS | All text escaped via `Escaped<T>` wrapper |
| CSS injection | Semicolons/braces escaped in `escape_css_string` |

---

## 📚 Lesson Index

| Lesson | Topic |
|--------|-------|
| **0** | Components Basics |
| **1** | CSS Scoping |
| **2** | Global vs Scoped Styles |
| **3** | Component Composition |
| **4** | Children Pattern |
| **5** | @let Variables |
| **6** | Control Flow (@if, @for, @match) |
| **7** | Form Handling |
| **8** | Server Actions |
| **9** | Azumi Live Intro |
| **10** | Live Components |
| **11** | Event Binding |
| **12** | Optimistic UI Flow |
| **13** | Form Patterns |
| **14** | Component Composition (Advanced) |
| **15** | Full Application (Todo App) |
| **16** | Async Database (sqlx) |
| **17** | Testing Infrastructure |
| **18** | Security (Signed State) |
| **19** | Authentication (Axum Middleware) |

---

## 📄 License

Dracon License v1.1 — small teams can use included software for free; larger organizations need the annual commercial license. See [LICENSE](LICENSE) for details.

---

_The only web framework that validates your HTML, scopes your CSS, checks accessibility, and generates optimistic UI from Rust code — all at compile time._
