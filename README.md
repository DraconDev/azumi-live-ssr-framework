# Azumi

> **Rust full-stack. No WASM. No second language. Compiler catches your frontend bugs.**

```bash
cargo install azumi-cli
azumi new my-app
cd my-app && cargo run
# → http://localhost:8080
```

---

## What Is Azumi?

Azumi is a **full-stack Rust web framework** for people who are tired of their frontend being the thing that breaks.

| Next.js / SvelteKit | Leptos / Dioxus | **Azumi** |
|---|---|---|
| Two languages (JS + backend) | WASM download tax (~150KB+) | **One language: Rust** |
| Type safety breaks at `fetch()` | Complex reactivity model | **~3KB runtime, HTML is truth** |
| Hydration overhead | DOM bridge overhead | **Zero hydration, zero WASM** |

**Born from:** *"My backend is Rust — rock solid, memory safe, compiled. My frontend is JS/TS — runtime errors, npm fragility, `undefined is not a function`. Why?"*

---

## 5-Minute Quickstart

### 1. Install the CLI

```bash
cargo install azumi-cli
```

### 2. Create a project

```bash
azumi new my-app
cd my-app
```

This generates:
- `Cargo.toml` with Azumi + Axum + Tokio
- `src/main.rs` with a working page + interactive counter demo
- `.gitignore`

### 3. Run it

```bash
cargo run
# → http://localhost:8080
```

You get a landing page with an interactive counter. The counter uses `#[azumi::live]` — state is HMAC-signed, mutations run on the server, and the DOM updates without a full page reload.

---

## The Pitch: Compiler Catches What the Browser Can't

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

| Other Frameworks | Azumi |
|---|---|
| CSS typo → silent fail | CSS typo → **compile error** |
| Missing class → invisible bug | Missing class → **compile error** |
| Invalid HTML → maybe works? | Invalid HTML → **compile error** |
| Click handler typo → 💀 | Click handler typo → **compile error** |
| Missing alt text → a11y fail | Missing alt text → **compile error** |

---

## Key Features

### 1. CSS-HTML Co-Validation (Industry First)

The compiler knows your styles. Use a class that doesn't exist? Error. Typo? Error.

### 2. Zero-Hydration Architecture

- Static pages: **0KB JavaScript**
- Interactive pages: **~3KB runtime** (event delegator + Idiomorph)
- No virtual DOM. No WASM. HTML is truth.

### 3. HMAC-Signed State

Every component's state is cryptographically signed. Users can't forge state or tamper with serialized data.

### 4. Surgical Interactivity

```rust
#[azumi::live]
pub struct Counter { pub count: i32 }

#[azumi::live]
impl Counter {
    pub fn increment(&mut self) { self.count += 1; }
}
```

The compiler auto-detects simple mutations and generates optimistic predictions. Click → instant UI update → server confirms.

### 5. AI-Ready Validation Pipeline

The `html!` macro runs validators in sequence: CSS properties → node order → Raw() blocking → format! detection → class/ID validation → HTML structure → attribute whitelist.

AI-generated code is guided toward correct patterns by default.

---

## Client Features (No Custom JS Needed)

| Feature | Attribute | Use Case |
|---|---|---|
| Form actions | `az-action` + `az-target` | Submit form, swap result HTML |
| Client state | `az-ui` + `az-on` | Tabs, toggles, counters |
| Conditional classes | `az-bind:class` | Dynamic styling |
| Confirmation | `az-confirm` | "Are you sure?" dialogs |
| Auto-init | `az-init` | Run on page load |
| Scroll reveal | `az-reveal` | Animate on scroll |
| Scroll to top | `scroll-top` | Smooth scroll |

See [docs/interactivity.md](docs/interactivity.md) for the full catalog.

---

## Performance

| Metric | Azumi | React | HTMX |
|---|---|---|---|
| **Bundle Size** | ~3KB | 100KB+ | 14KB |
| **Hydration** | None | Required | None |
| **CSS Validation** | Compile-time | Runtime | None |
| **Type Safety** | Full Rust | TypeScript | None |

---

## Documentation

| Document | What You'll Find |
|---|---|
| [docs/guide.md](docs/guide.md) | Full framework guide — components, control flow, live state, forms, security |
| [docs/interactivity.md](docs/interactivity.md) | Client feature catalog — every `az-*` attribute with examples |
| [docs/comparison.md](docs/comparison.md) | Framework comparison — Next.js, SvelteKit, Leptos, Dioxus, HTMX, Maud |
| [docs/architecture.md](docs/architecture.md) | Internal architecture — validation pipeline, rendering flow, macro expansion |
| [CHANGELOG.md](CHANGELOG.md) | Release history and migration guides |
| [TODO.md](TODO.md) | Development roadmap |

---

## Project Structure

```
azumi/
├── src/              # Core framework
│   ├── lib.rs       # Component trait, prelude, routes! macro
│   ├── action.rs    # Server action registry, success_fragment, error_fragment
│   ├── security.rs  # HMAC-SHA256 state signing
│   ├── script.rs    # AzumiScript, escape helpers
│   └── ...
├── macros/           # Procedural macros (html!, component, live, action)
├── demo/             # Interactive learning platform (20 lessons)
├── cli/              # azumi-cli scaffolding tool
└── docs/             # Documentation
```

---

## Stability Promise

**Current release:** `v47.20.46` — rapid iteration phase to find the right API shape.

**Starting with v48.0.0:** Azumi follows strict [Semantic Versioning](https://semver.org/):

| Bump | What It Means |
|---|---|
| **Major** | Actual breaking changes. At most every 3 months. Full migration guide included. |
| **Minor** | New features, backward compatible. Published monthly. |
| **Patch** | Bug fixes only. Published as needed. |

`azumi = "48"` in Cargo.toml will never break your build.

---

## License

Dracon License v1.1 — small teams can use included software for free; larger organizations need the annual commercial license. See [LICENSE](LICENSE) for details.

---

_The only web framework that validates your HTML, scopes your CSS, checks accessibility, and generates optimistic UI from Rust code — all at compile time._
