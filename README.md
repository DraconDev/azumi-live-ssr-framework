# Azumi

### Server-rendered HTML with client interactivity — all Rust, zero custom JavaScript

> HTMX's simplicity. React's interactivity. No JS ecosystem. No WASM. No ecosystem churn.

```bash
cargo install azumi-cli
azumi new my-app
cd my-app && cargo run
# → http://localhost:8080
```

---

## Why Azumi?

Web development has two real problems:

**1. The frontend ecosystem is a treadmill.** React rewrites every 2 years. Next.js changes patterns every 6 months. Your `node_modules` has 1,200 packages. You spend more time maintaining the build than building the product.

**2. Pure server-side has no interactivity.** HTMX and Rails prove that server-rendered HTML works — but every toggle, expand, and confirm requires a network roundtrip. Your app feels slow even when the server is fast.

Azumi started as HTMX and improved upon it. Same server-rendered approach. Same HTML-first philosophy. But the toggle works instantly. The reveal animates on scroll. The optimistic counter updates before the server responds. **~10KB of JavaScript (gzipped) handles what HTMX needs the network for.**

And because it's Rust — not JavaScript — **you never have to rewrite your frontend framework.** Azumi is complete. No ecosystem to maintain. No migration guides to follow. Build your product, not your toolchain.

---

## The Trilemma (And How Azumi Solves It)

Every other framework forces you to pick two of three:

| Need | HTMX / Rails | React / Next.js | Leptos / Dioxus | **Azumi** |
|------|:---:|:---:|:---:|:---:|
| Server-rendered, simple | ✅ | ❌ hydration | ❌ WASM | ✅ |
| Client interactivity | ❌ network roundtrip | ✅ | ✅ | ✅ ~10KB runtime |
| No JS ecosystem churn | ✅ | ❌ npm/TS hell | ✅ but WASM tax | ✅ |

**Azumi is the only option that gives you all three.**

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
| CSS typo → invisible bug | CSS typo → **compile error** |
| Missing class → silent fail | Missing class → **compile error** |
| Invalid HTML → maybe works? | Invalid HTML → **compile error** |
| XSS attempt → runtime escape | XSS attempt → **compile error** |
| Missing `alt` → a11y fail | Missing `alt` → **compile error** |

But type safety is the safety net, not the reason you're here. You're here because you're done with the frontend treadmill and you want your toggles to work without a network request.

---

## Quickstart

### 1. Install the CLI

```bash
cargo install azumi-cli
```

### 2. Create a project

```bash
azumi new my-app
cd my-app
```

### 3. Run it

```bash
cargo run
# → http://localhost:8080
```

You get a landing page with an interactive counter — HMAC-signed state, server-side mutations, instant DOM updates. Zero custom JavaScript.

---

## What It Looks Like

**A page:**

```rust
#[azumi::page(route = "/about")]
fn about_page() -> impl Component {
    html! {
        <div>
            <h1>"About"</h1>
            <p>"Server-rendered HTML. No WASM. No hydration."</p>
        </div>
    }
}
```

**An interactive component:**

```rust
#[azumi::live]
struct Counter { count: i32 }

#[azumi::component]
fn counter(state: &Counter) -> impl Component {
    html! {
        <div az-scope={state}>
            <span>{state.count}</span>
            <button az-on:click="increment">"+1"</button>
        </div>
        <style>
            .counter { display: "flex"; gap: "1rem"; }
        </style>
    }
}
```

Click → instant optimistic update → server confirms. No network roundtrip for the UI response. No JavaScript written by you.

---

## Client Features — No Custom JS Needed

| Feature | Attribute | What It Does |
|---|---|---|
| Form actions | `az-action` + `az-target` | Submit form, swap result HTML via DOM morphing |
| Client state | `az-ui` + `az-on` | Tabs, toggles, counters — no server roundtrip |
| Conditional classes | `az-bind:class` | Dynamic styling based on state |
| Confirmation dialogs | `az-confirm` | "Are you sure?" before submitting |
| Scroll reveal | `az-reveal` | Animate elements on scroll into view |
| Auto-init | `az-init` | Run action on page load |

**Production proof:** [dracon.dev](https://dracon.dev) runs 3 production sites on Azumi with 98 `html!` calls and zero lines of custom JavaScript. The only JS is third-party integrations (Paddle, Google Analytics).

---

## Performance

| Metric | Azumi | React | HTMX | Leptos |
|---|---|---|---|---|
| **JS shipped (gzipped)** | **10KB** | 46KB | 15KB | 150KB+ WASM |
| **Hydration** | None | Required | None | Required |
| **CSS validation** | Compile-time | Runtime | None | None |
| **Type safety** | Full Rust | TypeScript | None | Full Rust |
| **Ecosystem churn** | None | Constant | Minimal | Moderate |

---

## When to Use Azumi (And When Not To)

**Use Azumi when:**
- You want server-rendered HTML with real interactivity
- You're tired of maintaining a JS/TS frontend stack
- You want your app to still work in 5 years without a rewrite
- Your app is a dashboard, admin panel, e-commerce, or content site

**Don't use Azumi when:**
- You need client-side routing (SPAs with instant page transitions)
- You need to run Rust in the browser (use Leptos or Dioxus)
- You need complex canvas/WebGL/editor interactions (use a JS framework)

---

## Documentation

| Document | What You'll Find |
|---|---|
| [docs/why-azumi.md](docs/why-azumi.md) | Why Azumi exists — origin story, competitive landscape, thesis |
| [docs/guide.md](docs/guide.md) | Full developer guide — components, live state, forms, security |
| [CHANGELOG.md](CHANGELOG.md) | Release history and migration guides |

---

## Stability Promise

Azumi is **complete**. It does what it does and it's done. No ecosystem to keep up with. No framework rewrites every 2 years.

Starting with v48.0.0, Azumi follows strict [Semantic Versioning](https://semver.org/):

| Bump | What It Means |
|---|---|
| **Major** | Breaking changes. At most every 3 months. Full migration guide included. |
| **Minor** | New features, backward compatible. |
| **Patch** | Bug fixes only. |

`azumi = "48"` in `Cargo.toml` will never break your build.

---

## License

This project is dual-licensed:

- **AGPL-3.0-only** — See [LICENSE](LICENSE) for the full text. Default for open source use.
- **Commercial License** — For organizations that prefer not to comply with AGPLv3's source disclosure requirements. See [COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md) for details.

By contributing, you agree to the terms in [CLA.md](CLA.md).
