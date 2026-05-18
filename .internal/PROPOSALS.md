# Azumi Improvement Proposals

> Strategic improvements that advance Azumi's core mission: **AI-first, compile-time validated web framework where wrong code simply doesn't compile.**

---

## Tier 1: High-Impact, High-Alignment

These directly strengthen Azumi's unique value proposition — compile-time safety that catches bugs before they ship.

### 1. Compile-Time Route Type Safety

**Problem:** Routes are strings. A typo in `href="/abuot"` or `az-action="/contct"` compiles fine but breaks at runtime. This is the #1 category of bug that Azumi's philosophy should prevent but currently doesn't.

**Proposal:** Introduce a `#[azumi::route]` macro that registers routes at compile time, then validate `href` and `az-action` values against the registered route table inside `html!`.

```rust
// In routes.rs:
#[azumi::route] fn home() -> impl Component { ... }
#[azumi::route] fn about() -> impl Component { ... }
#[azumi::route] fn contact() -> impl Component { ... }

// In html! — typo caught at compile time:
html! {
    <a href="/abuot">"About"</a>  // ❌ compile error: unknown route "/abuot"
    <form az-action="/contct">    // ❌ compile error: unknown action "/contct"
}
```

**Implementation path:**
- Add a `RouteRegistry` collected via `inventory` (same pattern as `ActionEntry`)
- In `html!`, resolve `href="/..."` and `az-action="/..."` against the registry
- Allow `class:external`-style opt-out: `href:external="/third-party"`
- Phase 1: warn on unknown routes. Phase 2: error.

**Impact:** Eliminates an entire class of 404 bugs and broken form actions at compile time. This is the single highest-value improvement for Azumi's mission.

---

### 2. Compile-Time `json_data!` Type Verification

**Problem:** `json_data!("APP_DATA" = &data)` serializes any `&T` where `T: Serialize`. If the Rust struct drifts from what the JavaScript expects, you get silent runtime breakage.

**Proposal:** Add an optional `#[azumi::contract]` attribute that generates TypeScript type declarations from Rust structs, and validates that `json_data!` variable names match declared contracts.

```rust
#[azumi::contract]
struct AppData {
    count: u32,
    name: String,
}

// Generates: window.APP_DATA type declaration for TypeScript
// At compile time: validates that "APP_DATA" in json_data! matches a contract
```

**Implementation path:**
- `#[azumi::contract]` derives `Serialize` + generates a `.d.ts` snippet
- `json_data!` macro cross-references declared contracts
- Build script could write a consolidated `azumi-types.d.ts`

**Impact:** Full-stack type safety. Rust backend → TypeScript frontend. Zero-cost since it's compile-time only.

---

### 3. Component Props Validation at Compile Time

**Problem:** The `Props::builder().build().expect("all required props provided")` pattern can panic at runtime if a required prop is missing. The `expect` message is unhelpful — it just says the field name, not which component or where.

**Proposal:** Make `build()` return a `Result<Props, MissingPropsError>` and generate compile-time diagnostics that list exactly which props are missing.

```rust
// Current (runtime panic):
let props = card::Props::builder()
    .title("Hello".to_string())
    .build()
    .expect("missing props"); // which ones? no idea

// Proposed (compile-time generated error messages):
// The generated build() code uses const assertions to check required fields
// and produces compile_error!() for each missing prop
```

**Implementation path:**
- In `component.rs`, generate `build()` with per-field `Option` checks
- For each missing required field, produce a `compile_error!` with the field name
- This requires making the builder stateful (typed-state pattern), but the payoff is huge

**Impact:** Every Azumi component becomes fully type-checked. Zero `expect()` panics in production.

---

## Tier 2: Architecture & DX Improvements

### 4. Streaming Live Updates (SSE → Replace Polling)

**Problem:** Current `#[azumi::live]` uses full round-trip POST for every state change. For real-time apps (chat, dashboards, notifications), this means either polling or custom WebSocket code outside Azumi.

**Proposal:** Add `#[azumi::live_stream]` that uses SSE to push server-side state changes to connected clients, with automatic DOM morphing.

```rust
#[azumi::live]
struct ChatRoom { messages: Vec<String> }

#[azumi::live_stream]
impl ChatRoom {
    // Server pushes when new messages arrive
    async fn on_new_message(&mut self, msg: String) {
        self.messages.push(msg);
        // SSE event auto-sent to all connected clients
    }
}
```

**Implementation path:**
- Each `az-scope` element opens an SSE connection on mount
- Server-side state changes broadcast via `tokio::broadcast` channels
- Client-side: Idiomorph morphs the incoming HTML fragment
- Reuse existing `SseEvent::fragment()` infrastructure

**Impact:** Real-time apps without leaving Azumi's model. Competes with LiveView/Elixir natively.

---

### 5. Typed CSS Custom Properties (Design Tokens)

**Problem:** CSS custom properties (`--my-color: red`) are strings with no type checking. A typo like `--my-clor: rd` compiles and silently produces wrong styles.

**Proposal:** Add a `define_tokens!` macro that creates typed CSS custom properties with compile-time validation.

```rust
define_tokens! {
    Colors {
        primary: "#0070f3",    // validated as CSS color
        danger: "#d32f2f",
        bg: "#fafafa",
    }
    Spacing {
        sm: "0.5rem",          // validated as CSS length
        md: "1rem",
        lg: "2rem",
    }
}

html! {
    <div style={--primary-color: Colors::primary, --spacing: Spacing::md}>
        "Content"
    </div>
}
```

**Impact:** Design system enforcement at compile time. No more `var(--typo-here)` bugs.

---

### 6. Accessibility Score at Compile Time

**Problem:** The accessibility validator checks individual rules (img needs alt, buttons need content), but doesn't give a holistic score or catch structural a11y issues like skip-nav links, landmark regions, or focus order.

**Proposal:** Add `#[azumi::a11y_audit]` page-level attribute that generates a compile-time accessibility report.

```rust
#[azumi::page]
#[azumi::a11y_audit(level = "AA")]
fn my_page() -> impl Component { ... }
// Compile warning: "Page missing skip-navigation link"
// Compile warning: "No landmark <main> found"
// Compile warning: "3 images without alt text"
```

**Implementation path:**
- Extend `accessibility_validator.rs` with page-level checks
- Use the existing AST walking infrastructure
- Start with WCAG 2.1 Level A, expand to AA

**Impact:** Azumi becomes the only framework that catches accessibility violations at compile time. This is a massive differentiator.

---

### 7. Partial Hydration / Island Architecture

**Problem:** Currently, if ANY part of a page is interactive, the entire Azumi JS runtime (~10KB gzipped) loads. For mostly-static pages with one interactive counter, this is overkill.

**Proposal:** Add `az-island` attribute that selectively hydrates only interactive components.

```rust
html! {
    <div>
        <h1>"Static content"</h1>           <!-- zero JS -->
        <p>"More static content"</p>
        <div az-island>                       <!-- only this gets JS -->
            <button az-on:click="call increment">
                "Count: "{state.count}
            </button>
        </div>
    </div>
}
```

**Implementation path:**
- `az-island` elements get their own scope + event delegation
- Non-island content requires zero JS registration
- The runtime could be split: core (1KB) + islands (2KB), only loading islands when `az-island` exists

**Impact:** Sub-1KB JS for static pages. ~10KB (gzipped) only when interactivity is needed. Best of both worlds.

---

## Tier 3: Ecosystem & Polish

### 8. `azumi check` — Standalone Lint Command

**Problem:** Azumi's compile-time validation is amazing, but you only see it when you compile. There's no way to run a quick lint pass without a full `cargo build`.

**Proposal:** Add `azumi check` CLI command that runs the `html!` validator over source files without compiling.

```bash
$ azumi check src/
✓ src/pages/home.rs: 3 components, 0 issues
✓ src/pages/about.rs: 1 component, 0 issues  
✗ src/pages/contact.rs: 2 issues
  → line 14: static class attribute "form-group" — use class:external="form-group"
  → line 22: missing alt attribute on <img>
```

**Implementation path:**
- Extract the validator logic into a standalone crate
- Parse `html!` macro invocations from source text
- Run validation pipeline, emit diagnostics

**Impact:** Instant feedback loop. Integrates with editors via LSP.

---

### 9. Per-Request SEO (Kill the Global Singleton)

**Problem:** `init_seo()` is a process-global `RwLock<Option<SeoConfig>>` singleton. In multi-tenant apps, all tenants share the same site name, OG image, etc. This is a known limitation documented in the code.

**Proposal:** Make `SeoConfig` a per-request context, not a global. The `#[azumi::page]` macro already sets page context via task-locals — extend this to SEO config.

```rust
// Instead of:
init_seo(SeoConfig::new("My Site")); // global, once

// Per-request:
async fn handler(tenant: Tenant) -> impl IntoResponse {
    let seo = tenant.seo_config(); // per-tenant
    azumi::context::with_seo_scope(seo, async {
        let html = render_to_string(&my_page());
        axum::response::Html(html)
    }).await
}
```

**Implementation path:**
- Add `SEO_CONFIG` task-local (same pattern as `PAGE_META`)
- `generate_head()` reads from task-local first, then global
- Keep `init_seo()` for simple apps, add `with_seo_scope()` for multi-tenant

**Impact:** Azumi works for SaaS/multi-tenant out of the box.

---

### 10. Azumi Dev Server (Auto-Start + Browser Sync)

**Problem:** The hot reload system (`auto_reload()`) is powerful but requires manual setup — adding the call to `main()`, configuring the WebSocket, and manually opening the browser.

**Proposal:** `azumi dev` command that auto-starts the server, opens the browser, and injects the WebSocket script.

```bash
$ azumi dev
🔄 Building azumi-demo...
⚡ Server running at http://localhost:8080
📖 Browser opened
👀 Watching for changes...
```

**Implementation path:**
- Extend `cli/` with a `dev` subcommand
- Auto-inject `{azumi_script()}` + hot-reload WebSocket URL
- Use `opener` crate (or equivalent) for browser launch
- Proxy to the real Axum server

**Impact:** Zero-config development experience. `azumi dev` and you're running.

---

## Priority Matrix

| # | Proposal | Impact | Effort | ROI |
|---|----------|--------|--------|-----|
| 1 | Route Type Safety | 🔴 Critical | 2-3 weeks | ★★★★★ |
| 3 | Component Props Compile Errors | 🔴 High | 1-2 weeks | ★★★★★ |
| 6 | A11y Audit | 🔴 High | 1-2 weeks | ★★★★☆ |
| 9 | Per-Request SEO | 🟡 Medium | 3-5 days | ★★★★☆ |
| 7 | Island Architecture | 🟡 Medium | 3-4 weeks | ★★★☆☆ |
| 2 | json_data! Type Verification | 🟡 Medium | 2-3 weeks | ★★★☆☆ |
| 8 | `azumi check` CLI | 🟢 Nice | 1-2 weeks | ★★★☆☆ |
| 5 | Typed CSS Tokens | 🟢 Nice | 2-3 weeks | ★★☆☆☆ |
| 4 | SSE Live Streaming | 🟡 Medium | 3-4 weeks | ★★☆☆☆ |
| 10 | Dev Server | 🟢 Nice | 1-2 weeks | ★★☆☆☆ |

---

## The North Star

Azumi's killer feature is **compile-time validation**. Every improvement should ask: *"Does this catch bugs earlier?"*

The biggest gap right now is that **routes and actions are strings**. Everything inside `html!` is validated — tags, attributes, CSS, structure, XSS patterns — but the links between pages and the actions that serve them are untyped strings. Closing this gap (Proposal #1) would make Azumi the first framework where **broken links literally don't compile**.

That's the pitch: *"If it compiles, it works."* Right now that's 90% true. Route type safety gets it to 99%.
