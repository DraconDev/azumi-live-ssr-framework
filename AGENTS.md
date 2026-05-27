# Azumi — AI Agent Guide

> **Active reference** — this file is the primary AI-facing guide for code generation rules.
> **For step-by-step recipes, see [AI_COOKBOOK.md](AI_COOKBOOK.md).**
> Detailed usage docs live in [docs/guide.md](docs/guide.md), but the rules below are authoritative for AI-generated code.

## Philosophy

Azumi is an **AI-first, compiler-validated web framework**. Every decision prioritizes what makes AI-generated code correct by default:

- **Safe macros for real work, bare tags for injection**: `json_data!` handles serde + variable naming; `<style>{var}</style>` and `<script>{var}</script>` auto-escape content
- **Zero escape hatches**: `Raw()`, `from_fn()` are `#[doc(hidden)]` — AIs should never reach for them. `TrustedHtml` is the public, documented escape hatch for pre-sanitized HTML.
- **Compile-time validation**: Wrong patterns produce clear errors with exact alternatives shown

## Three Golden Rules for AI-Generated Code

1. **NO `Raw()` inside `html!`** — compile error, use the safe patterns instead
2. **NO `format!` building web content** inside `html!` — compile error, use interpolation or safe macros
3. **ALWAYS use `<style>{var}</style>` / `<script>{var}</script>` for CSS/JS injection, and `json_data!` for JSON data**

## Safe Injection Patterns

### `<style>{css_var}</style>`
Safe CSS injection. Auto-escapes `</style>` (case-insensitive) inside `html!`.

```rust
html! {
    <style>{THEME_CSS}</style>
}
// Renders: <style>.my_class { color: red; }</style>
```

### `<script>{js_var}</script>`
Safe JavaScript injection. Auto-escapes `</script>` (case-insensitive) inside `html!`.

```rust
html! {
    <script>{TRACKING_JS}</script>
}
// Renders: <script>console.log('track');</script>
```

### `json_data!("VAR_NAME" = &rust_data)`
Safe JSON data injection for JavaScript. Auto-serializes, escapes `</script>`.

```rust
let data = serde_json::json!({"key": "value"});
html! {
    {azumi::json_data!("APP_DATA" = &data)}
}
// Renders: <script>APP_DATA = {"key":"value"};</script>
```

### `TrustedHtml::new(html)` — Pre-Sanitized HTML Injection

For injecting HTML from trusted sources (CMS output, markdown renderer, sanitized HTML).
This is the **safe replacement for `Raw()`** — it's public and documented, unlike `Raw()` which is `#[doc(hidden)]`.

```rust
use azumi::TrustedHtml;

// From &str
let cms_body = TrustedHtml::new("<p>Hello from <strong>CMS</strong></p>");
html! {
    <div>{cms_body}</div>
}
// Renders: <div><p>Hello from <strong>CMS</strong></p></div>

// From owned String
let markdown_html = TrustedHtml::from_string(render_markdown(&input));
html! {
    <article>{markdown_html}</article>
}
```

**When to use:**
- CMS/markdown HTML output that you trust
- HTML from your own sanitization pipeline
- Pre-rendered component HTML (e.g., subnavs, breadcrumbs built elsewhere)

**When NOT to use:**
- Untrusted user input → use `{user_input}` (auto-escaped)
- Already-escaped strings → just use `{x}` directly
- JavaScript → use `<script>{var}</script>`
- CSS → use `<style>{var}</style>`
- JSON data → use `json_data!`

## Control Flow in `html!`

### `@for` Iteration

Use `@for` to render dynamic lists. Works with any Rust iterator.

```rust
// Simple iteration
html! {
    <ul>
        @for item in &items {
            <li>{item}</li>
        }
    </ul>
}

// With index
html! {
    @for (i, item) in items.iter().enumerate() {
        <div>{i + 1}". "{item}</div>
    }
}

// Ranges
html! {
    @for i in 1..=5 {
        <span>{i}</span>
    }
}

// Iterator adapters (filter, map, take, skip, rev, chain, etc.)
html! {
    @for n in nums.iter().filter(|x| **x % 2 == 0) {
        <div>{n}</div>
    }
}

// Nested iteration
html! {
    @for row in &matrix {
        <tr>
            @for cell in row {
                <td>{cell}</td>
            }
        </tr>
    }
}

// Key-value pairs
html! {
    @for (name, url) in &links {
        <a href={url}>{name}</a>
    }
}
```

### `@for` with `@keyed(id)` — Keyed List Updates

Use `@keyed(expr)` to give each list item a stable identity. The DOM morpher uses the key to move/reuse items instead of destroying and recreating them. Preserves scroll position, input focus, and CSS transitions.

```rust
html! {
    @for item in &items @keyed(item.id) {
        <div class={row_class}>{&item.name}</div>
    }
}
// Renders: <div data-key="1" class="row">Alice</div>
//          <div data-key="2" class="row">Bob</div>
```

**Rules:**
- Key expression must evaluate to a stable, unique identifier per item
- Without `@keyed`, every list change triggers full DOM replacement
- The first HTML element inside each `@for` body gets the `data-key` attribute
- Works with both owned (`Vec<T>`) and borrowed (`&[T]`) iterators

**Why this matters**: `@keyed` prevents list jank — no scroll jumps, no lost focus, smooth DOM updates when items are added, removed, or reordered.

### `@if` / `@match`

```rust
html! {
    @if user.is_admin {
        <div>"Admin panel"</div>
    }

    @match status {
        200 => <span>"OK"</span>,
        404 => <span>"Not Found"</span>,
        _ => <span>"Error"</span>,
    }
}
```

## Two-Way Form Binding

### `bind:value` — Sync Input ↔ State

Automatically syncs an `<input>` value with state. Debounced at 200ms. No manual JS required.

```rust
html! {
    <input type="text" bind:value={state.name} />
    <input type="checkbox" bind:checked={state.agree} />
    <select bind:value={state.choice}>
        <option value="rust">"Rust"</option>
    </select>
    <textarea bind:value={state.bio}></textarea>
}
```

**Renders as:** `data-bind-value="field.path"` in HTML. The client runtime reads this attribute, listens for `input`/`change` events, and syncs the value into the nearest `az-scope` or `az-ui` state.

**Supports:**
- `<input>` (text, email, password, number, etc.)
- `<input type="checkbox">` → `bind:checked` (boolean toggle)
- `<input type="radio">` → `bind:value` (string set on selected radio)
- `<select>` and `<textarea>`
- **Nested field paths**: `bind:value={state.user.name}` updates `state.user.name`
- **Debounce**: 200ms default (configurable via `debounce=N`)

**Why this matters:** The #1 interactivity gap. Without `bind:value`, every form input needs a separate `on:input` handler with manual debounce logic. With `bind:value`, it's one keyword.

## Scoped CSS

Every `<style>` block inside `html!` is **automatically scoped** to its component. CSS selectors are rewritten at compile time to include a unique `data-{scope_id}` attribute.

```rust
html! {
    <div class={card_class}>
        <style>.card { color: red; }</style>
        <p>"This text is red"</p>
    </div>
}
// Output:
// <div class="card" data-s1a2b3c4d="s1a2b3c4d">
//   <style data-azumi-scope="s1a2b3c4d">.card[data-s1a2b3c4d] { color: red; }</style>
//   <p>This text is red</p>
// </div>
```

**Rules:**
- All `<style>` blocks are scoped by default — no CSS leaks between components
- `style! global { ... }` creates unscoped global CSS
- `<style>{var}</style>` with dynamic CSS bypasses scoping (for user-provided themes)
- `style! { ... }` provides typed, validated CSS with Rust expressions in values
- The scope ID is deterministic from the source position (same code → same hash across builds)

**Why this matters:** No class name collisions. No CSS reset wars. No `!important` arms race. Each component owns its styles.

## Form Validation

### `data-validate` — Client-Side Validation

Declarative form validation with zero custom JS.

```rust
html! {
    <form az-action="signup" az-target="#result">
        <input type="text" name="name"
               data-validate="name:required" />
        <p id="name_error" class:external={"form-error"} style={"display:none"}></p>

        <input type="email" name="email"
               data-validate="email:required,email" />
        <p id="email_error" class:external={"form-error"} style={"display:none"}></p>

        <input type="password" name="password"
               data-validate="password:required,min-length:8" />
        <p id="password_error" class:external={"form-error"} style={"display:none"}></p>

        <button type="submit">"Sign Up"</button>
    </form>
}
```

**Supported rules:**

## Transitions (`az-transition:`)

Smooth enter/exit animations for elements during DOM updates. Works automatically after `az-action` morphs or `@keyed` reorders.

```rust
html! {
    // Fade in/out: element fades from opacity 0→1 on enter, 1→0 on exit
    <div az-transition:fade>"I appear smoothly"</div>

    // Slide: element slides open/closed (max-height + opacity)
    <div az-transition:slide>"I slide in"</div>

    // Scale: element scales 0.95→1.0 with opacity on enter
    <div az-transition:scale>"I scale up"</div>

    // With custom duration (default: 200ms)
    <div az-transition:fade duration=500>"Slow fade"</div>
}
```

**Rules:**
- Enter: animation runs immediately after DOM morph completes
- Exit: original element cloned, original removed, clone animates out, then self-destructs
- Works with `@keyed` — reordered items reposition without exit/enter
- Combine with `az-reveal` for scroll-triggered reveals

**Supported rules:**
| Rule | Example | Description |
|------|---------|-------------|
| `required` | `required` | Field must not be empty |
| `email` | `required,email` | Must be valid email format |
| `min-length:N` | `required,min-length:8` | Minimum character length |
| `max-length:N` | `max-length:254` | Maximum character length |
| `url` | `required,url` | Must be valid URL |

**Error display:** Validation errors appear in `id="{field}_error"` elements. Use `aria-invalid` and `aria-describedby` for accessibility.

**Why this matters:** Every form needs validation. Without `data-validate`, you write a separate JS file per form. With it, validation is part of the template.

## Interpolation Patterns

### `{&field}` — Borrow, Don't Clone

`html!` interpolation consumes values by default. For borrowed data (`&self` fields), use `{&field}` to avoid `.clone()`:

```rust
struct Card {
    title: String,
    description: String,
}

impl Component for Card {
    fn render(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ❌ Unnecessary .clone() — allocates for no reason
        // html! { <div>{self.title.clone()}</div> }

        // ✅ Borrow instead — zero allocation
        html! { <div>{&self.title}</div> }
    }
}
```

**Rule**: If the value implements `Display` (which `&String`, `&str`, numbers, etc. do), `{&field}` works without cloning. Use `.clone()` only when you need ownership (e.g., moving into a closure that outlives the borrow).

### `{component}` — Auto-Escaped Display

All `{expression}` interpolation in `html!` is auto-escaped via `Escaped<T>`:

```rust
// Text content — auto-escaped (XSS safe)
html! { <p>{user_input}</p> }
// Renders: <p>safe &amp; escaped</p>

// Component — rendered directly (not double-escaped)
html! { <div>{my_component}</div> }
```

## `#[azumi::component]` — Struct-Based Props with Builder

The `#[azumi::component]` attribute macro converts a function into a module with a `Props` struct and `PropsBuilder`. This eliminates manual parameter passing and enables zero-clone rendering.

```rust
#[azumi::component]
fn user_card(name: String, role: String, active: bool) -> impl Component {
    html! {
        <div>
            <h3>{name}</h3>
            <p>{role}</p>
            @if active { <span>"Active"</span> }
        </div>
    }
}

// Usage — builder pattern, move semantics (no clones needed)
let card = user_card::render(
    user_card::Props::builder()
        .name("Alice".to_string())
        .role("Admin".to_string())
        .active(true)
        .build()
        .expect("all required props provided"),
);
```

### Borrowed Parameters — Zero-Close Props

Use `&str` (or any `&T`) as parameter types to avoid `.clone()` when calling from `render()` methods. The macro automatically injects the required lifetime:

```rust
// No explicit lifetime needed — the macro handles it
#[azumi::component]
fn greeting(name: &str, age: i32) -> impl Component {
    html! { <div>{name} " is " {age}</div> }
}

// Builder accepts &str directly — no .to_string() or .clone()
let comp = greeting::render(
    greeting::Props::builder().name("Alice").age(30).build().unwrap(),
);
```

**Why this matters**: When rendering from `&self`, owned parameters force `.clone()`. Borrowed parameters accept references directly:

```rust
// ❌ Old way: owned params force clone
impl Component for MyPage {
    fn render(&self, f: &mut Formatter) -> fmt::Result {
        html! { {user_card::render(user_card::Props::builder()
            .name(self.title.clone())  // allocates!
            .build().unwrap())} }
    }
}

// ✅ New way: &str params accept references
impl Component for MyPage {
    fn render(&self, f: &mut Formatter) -> fmt::Result {
        html! { {greeting::render(greeting::Props::builder()
            .name(&self.title)  // borrows — zero alloc
            .build().unwrap())} }
    }
}
```

**Rules**:
- `&str` / `&T` without explicit lifetime → macro injects `'a` automatically
- `&'a str` / `&'static str` with explicit lifetime → used as-is (backward compatible)
- `String`, `i32`, etc. → owned, no lifetime injection
- `#[prop(default = "...")]` on `&str` → default must be `&'static str` (e.g., `"N/A"`)
- Mixed borrowed + owned parameters work naturally
```

### Props with Defaults

```rust
#[azumi::component]
fn alert(message: String, #[prop(default = "\"info\".to_string()")] level: String) -> impl Component {
    html! { <div class={level}>{message}</div> }
}

// level defaults to "info" when omitted
alert::Props::builder()
    .message("Saved!".to_string())
    .build()
    .expect("missing message");
```

For borrowed `&str` defaults, use a string literal (which is `&'static str` and coerces to `&'a str`):

```rust
#[azumi::component]
fn labeled(#[prop(default = "\"N/A\"")] label: &str, value: i32) -> impl Component {
    html! { <div>{label} ": " {value}</div> }
}
```

### Props with Children

```rust
#[azumi::component]
fn card(title: String, children: impl Component) -> impl Component {
    html! {
        <div>
            <h2>{title}</h2>
            {children}
        </div>
    }
}

// children is passed separately
card::render(
    card::Props::builder().title("Hello".to_string()).build().unwrap(),
    html! { <p>"Body content"</p> },
);
```

### Live State Components

If the first parameter is named `state` and is a reference to a non-primitive type, `#[azumi::component]` auto-wraps the output with an `az-scope` div for live updates:

```rust
#[azumi::live]
struct Counter { count: i32 }

#[azumi::component]
fn counter(state: &Counter) -> impl Component {
    html! { <span>{state.count}</span> }
    // Auto-wrapped: <div az-scope="..." az-struct="Counter" style="display: contents">...</div>
}
```

### `#[live_state]` — Explicit Live State Attribute

For components where the state parameter isn't named `state`, use `#[live_state]`:

```rust
#[azumi::component]
fn counter(#[live_state] ctx: &Counter) -> impl Component {
    html! { <span>{ctx.count}</span> }
    // Also auto-wrapped with az-scope div
}
```

**When to use `#[live_state]`:**
- Parameter named something other than `state` (e.g., `ctx`, `app_state`, `model`)
- Explicit is better than implicit — makes the intent clear
- Both approaches work identically at runtime

## Manual Escaping Outside `html!`

### `escape_html(&str) -> String`

For HTML entity escaping outside `html!` (SEO helpers, meta tags, attribute building):

```rust
use azumi::escape_html;

let safe = escape_html("<script>alert('xss')</script>");
// "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;"
```

### `escape_xml(&str) -> String`

Same as `escape_html` but uses `&apos;` (XML spec) instead of `&#x27;` (HTML convention):

```rust
use azumi::escape_xml;

let safe = escape_xml("Tom & Jerry's");
// "Tom &amp; Jerry&apos;s"
```

### `Escaped<T>` — Display Wrapper

`Escaped<T: Display>` wraps any `Display` type to produce HTML-escaped output. Used internally by `html!` interpolation:

```rust
use azumi::Escaped;
use std::fmt::Display;

fn make_safe<T: Display>(val: T) -> String {
    format!("{}", Escaped(val))
}
```

### `escape_css_string(&str) -> String`

Escapes CSS injection characters (`;`, `\`, `{`, `}`, quotes, `/`):

```rust
use azumi::escape_css_string;

let safe = escape_css_string("red; }");
// "red\\; \\}"
```

## Validation Pipeline (in order)

The `html!` macro runs these validators sequentially, short-circuiting on first error:

1. **CSS validation** — validates CSS property names and syntax
2. **Node order** — enforces Script → Content → Style ordering
3. **Raw usage** — **blocks ALL `Raw()` in html!** (unconditional)
4. **Format detection** — blocks `format!` with HTML/CSS/JS/DOM patterns
5. **Class/ID validation** — classes must be defined in `<style>` blocks
6. **HTML structure rules** — tables, lists, forms, buttons, headings, paragraphs
7. **Attribute validation** — valid HTML5 attributes, `data-*`, `aria-*`, `on:*` events

## Escape Hatches (all `#[doc(hidden)]`)

| Type | Visibility | When to use |
|------|-----------|-------------|
| `from_fn` | `#[doc(hidden)]` | Internal macro expansion only |
| `from_fn_once` | `#[doc(hidden)]` | Internal macro expansion only |
| `Raw<T>` | `#[doc(hidden)]` | Internal framework SEO generation only |
| `TrustedHtml` | **Public** | Pre-sanitized HTML from trusted sources (CMS, markdown renderer, sanitized output) |

AIs should **never** generate code using these. If one is needed, flag it to the human developer.

## Route Constants — Compile-Time Link Safety

Azumi validates everything inside `html!` except **route strings**. A typo in `href="/abuot"` compiles fine but 404s at runtime. Prevent this with route constants:

### `#[azumi::page(route = "/path")]` — Auto-Generated Route Constant

```rust
#[azumi::page(route = "/about")]
fn about_page() -> impl Component {
    html! { <div>"About"</div> }
}

// Auto-generated constant:
// about_page_ROUTE == "/about"
```

Use the constant in `html!` and Axum router setup:

```rust
// In main.rs — type-safe router
let app = Router::new()
    .route(about_page_ROUTE, get(about_page));

// In components — type-safe links
html! {
    <a href={about_page_ROUTE}>"About"</a>  // Can't typo the route
}
```

### `action_name_PATH` — Auto-Generated Action Path

Every `#[azumi::action]` function gets a `<name>_PATH` constant:

```rust
#[azumi::action]
fn like_post(form: LikeForm) -> ActionResult { ... }

// Auto-generated constant:
// like_post_PATH == "/azumi/action/like_post"
```

Use it in `html!`:

```rust
html! {
    <form az-action={like_post_PATH} az-target={"#like-area"}>
    </form>
}
```

### Manual Route Constants

For routes not tied to a `#[azumi::page]`, define constants manually:

```rust
pub mod routes {
    pub const HOME: &str = "/";
    pub const BLOG: &str = "/blog";
    pub const BLOG_POST: &str = "/blog/posts/{slug}";
}

let app = Router::new()
    .route(routes::HOME, get(home_handler))
    .route(routes::BLOG, get(blog_handler));

html! {
    <a href={routes::BLOG}>"Blog"</a>
    <a href={format!("/blog/posts/{}", slug)}>"Read"</a>  // Dynamic route
}
```

**Rule**: Always use route constants for `href` and `az-action` values. Never hardcode raw URL strings in `html!`.

## Project Structure

```
azumi/
├── macros/
│   ├── src/
│   │   ├── lib.rs                      # html! macro + validation pipeline
│   │   ├── inline_inject.rs            # json_data! macro only
│   │   ├── html_structure_validator.rs # Raw/format! validation + HTML rules
│   │   ├── token_parser.rs             # html! AST parser (handles <style>{var}</style>)
│   │   ├── css_validator.rs            # CSS property validation
│   │   ├── component.rs                # #[azumi::component] attribute macro
│   │   ├── page.rs                     # #[azumi::page] attribute macro
│   │   ├── action.rs                   # #[azumi::action] attribute macro
│   │   ├── live.rs                     # #[azumi::live] attribute macro
│   │   ├── schema.rs                   # #[azumi::schema] (gated behind "schema" feature)
│   │   ├── style.rs                    # CSS scoping + processing
│   │   ├── codegen.rs                  # Code generation helpers
│   │   ├── css.rs                      # CSS parsing/transformation
│   │   ├── context.rs                  # Macro context tracking
│   │   ├── head.rs                     # <head> content generation
│   │   ├── asset_rewriter.rs           # Asset hash rewriting
│   │   ├── style_processing.rs         # Style block processing
│   │   ├── validators.rs              # Shared validation logic
│   │   └── accessibility_validator.rs  # A11y attribute validation
├── src/
│   ├── lib.rs                          # Public API, Component trait, AZUMI_RULES, prelude
│   ├── script.rs                       # TrustedHtml, azumi_script(), escape_html(), escape_xml(), escape_tag_content()
│   ├── seo.rs                          # SEO helpers (uses Raw internally, outside html!)
│   ├── security.rs                     # HMAC state signing
│   ├── csp.rs                          # ContentSecurityPolicy builder + CspNonce + Axum middleware
│   ├── streaming.rs                    # SSE helpers (SseEvent, sse())
│   ├── css_scoping.rs                  # CSS scope isolation (compute_scope_id, scope_css)
│   ├── form.rs                         # Form validation (FormValidator, ValidatedForm)
│   ├── action.rs                       # Action handlers (#[cfg(feature = "axum")])
│   ├── context.rs                      # Page metadata context
│   ├── hot_reload.rs                   # Hot reload (#[cfg(feature = "devtools")])
│   ├── devtools.rs                     # Dev tools (#[cfg(feature = "devtools")])
│   └── tests.rs                        # Test utilities (render, assert_selector)
├── cli/                                # CLI tool
├── tests/
│   ├── inline_inject_tests.rs          # json_data! + auto-escape tests
│   ├── security_xss_tests.rs           # XSS breakout prevention tests
│   ├── integration_inject_tests.rs      # Integration tests for injection patterns
│   ├── csp_middleware_tests.rs          # CSP middleware + nonce integration tests
│   ├── live_handler_integration_tests.rs # Live handler security tests
│   └── ... (40+ other test files)
├── client/
│   ├── azumi.js                        # Client runtime (~11KB gzipped)
│   └── idiomorph.js                    # DOM morphing library
└── AGENTS.md                           # This file — AI agent instructions
```

## Prelude

`use azumi::prelude::*;` provides:

| Item | Notes |
|------|-------|
| `html`, `component`, `json_data`, `live` | Core macros |
| `Component` | Trait for renderable types |
| `AzumiScript`, `azumi_script`, `session_cleanup_script` | Client runtime helpers |
| `render_to_string`, `render_to_writer` | Render functions |
| `FnComponent` | Closure-based component type |
| `escape_css_string` | CSS string escaping |
| `escape_html` | HTML entity escaping (`&`, `<`, `>`, `"`, `'`) |
| `escape_xml` | XML entity escaping (uses `&apos;` per XML spec) |
| `TrustedHtml` | Pre-sanitized HTML injection (CMS/markdown output) |
| `CspNonce` | CSP nonce (from `csp` module) |
| `FormValidator`, `ValidatedForm`, `ValidationErrors` | Form validation |
| `ActionResult`, `error_fragment`, `success_fragment` | Action responses (`axum` feature only) |

## Common Anti-Patterns (caught at compile time)

```rust
// ❌ Static class attribute — compile error
html! { <div class="button"></div> }
// ❌ String literal in braces — compile error (bypasses validation)
html! { <div class={"button"}></div> }
// ✅ Variable — validated against <style> blocks
html! { <div class={btn_class}></div> }
// ✅ class:external — third-party component classes (CMS, widgets, embedded content)
html! { <div class:external="payment-widget"></div> }

// ❌ Raw() is always wrong in html!
html! { @{Raw(format!("<div>{}</div>", x))} }
// ✅ Use TrustedHtml for pre-sanitized HTML, or auto-escaping for text:
html! { <div>{TrustedHtml::new(cms_body)}</div> }
html! { <div>{user_text}</div> }  // auto-escaped

// ❌ format! building web content in html!
html! { {format!("window.location = '{}'", url)} }
// ✅ Do formatting outside html!:
let url = format!("window.location = '{}'", url);
html! { <script>{url}</script> }

// ❌ format! building CSS in html!
html! { {format!(".btn {{ color: {}; }}", c)} }
// ✅ Use <style> tag with auto-escape:
html! { <style>{css_var}</style> }
```

## `format!` Outside `html!` — The Safe Pattern

The validator blocks `format!` + web patterns (HTML/CSS/JS/DOM keywords) inside `html!` expressions. But `format!` **outside** `html!` is fully allowed:

```rust
// ❌ Blocked: format! with web content inside html!
html! { {format!("<div>{}</div>", name)} }
// ✅ Allowed: build the string outside, inject safely
let js_code = format!("window.location = '{}'", url);
html! { <script>{js_code}</script> }

// ❌ Blocked: format! building CSS inside html!
html! { {format!(".btn {{ color: {}; }}", color)} }
// ✅ Allowed: build CSS outside, inject via <style>
let css = format!(".btn {{ color: {}; }}", color);
html! { <style>{css}</style> }
```

**Boundary**: blocked when `format!` AND web-content patterns (`<`, `>`, `class=`, `window.`, etc.) coexist in the same `{expr}` inside `html!`. `format!` outside `html!` (building a variable) is fully allowed.

## Notes

- `<style>{var}</style>` and `<script>{var}</script>` auto-escape `</style>` and `</script>` sequences (case-insensitive). You do NOT need a macro for this.
- `json_data!("VAR" = &data)` is the only safe injection macro. It does real work: serde serialization + variable naming + `<script>` tag wrapping.
- For external JS/CSS, use `<script src="...">` and `<style src="...">`.
- `@for` is the preferred way to render dynamic lists inside `html!` — never use `format!` + `Raw()` for iteration.
- `{&field}` borrows instead of cloning — prefer it over `.clone()` for `&self` fields in `render()`.
- `escape_html()` and `escape_xml()` are for manual escaping outside `html!` — inside `html!`, all `{expr}` is auto-escaped.

## Tests

```bash
# Run everything
cargo test

# Run macro-specific tests
cargo test -p azumi-macros --lib

# Run injection tests
cargo test --test inline_inject_tests
cargo test --test security_xss_tests
cargo test --test integration_inject_tests
```

## Key Constants

- `azumi::AZUMI_RULES` — array of rule strings for AI reference
- `azumi::AZUMI_AI_HASH` — hash of AI rules for integrity verification
