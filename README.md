# Azumi

**Live SSR for Rust — server-rendered HTML with instant interactivity. No JavaScript. No WASM. No ecosystem churn.**

---

## You know the problem.

You picked HTMX because server-rendered HTML is simple. But every toggle needs a server roundtrip. Every "are you sure?" waits for the network. Your app feels slow even when the server is fast.

Or you picked React and got the interactivity — but also 1,200 packages in `node_modules`, a rewrite every two years, and two languages to maintain.

**Azumi gives you both.** Server-rendered HTML where the toggle works *instantly*. The reveal animates on scroll. The optimistic counter updates before the server confirms. ~11KB of runtime (gzipped). Zero custom JavaScript written by you.

---

## This is what it looks like.

A page — server-rendered, SEO-friendly, no hydration:

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

An interactive component — the button works **without a network roundtrip**:

```rust
#[azumi::live]
struct Counter { count: i32 }

#[azumi::component]
fn counter(state: &Counter) -> impl Component {
    html! {
        <div>
            <span>{state.count}</span>
            <button az-on:click="increment">"+1"</button>
        </div>
    }
}
```

Click → instant update → server confirms. The UI responds before the network. No JavaScript written by you.

And the compiler has your back:

```rust
html! {
    <div class={my_buttn}>   // ❌ COMPILE ERROR: 'my_buttn' not found. Did you mean 'my_button'?
        "Click me"
    </div>
    <img src="photo.jpg">   // ❌ COMPILE ERROR: <img> requires an `alt` attribute
}
```

**CSS typos, broken HTML, XSS attempts, missing ARIA — caught at `cargo build`.** Entire categories of bugs simply cannot exist in a compiled Azumi app.

---

## What's in the box

| Feature | How you do it |
|---------|--------------|
| Components + typed props | `#[azumi::component]` |
| Two-way binding | `bind:value={state.field}` |
| Keyed list updates | `@for item in items @keyed(item.id)` |
| Scoped CSS (per component) | `<style>` block — auto-scoped |
| Transitions | `az-transition:fade` / `:slide` / `:scale` |
| Form validation (8 rules) | `data-validate="field:required,email"` |
| Optimistic UI | `data-predict` + auto-detected predictions |
| Scroll reveal | `az-reveal={true}` |
| Confirm dialogs | `az-confirm="Are you sure?"` |
| Live state (signed) | `#[azumi::live]` + `az-scope` |
| Route constants | `#[azumi::page(route = "/")]` → `_ROUTE` |
| CSP builder | `ContentSecurityPolicy::new()` |
| SSE streaming | `SseEvent::fragment(html!)` |
| Hot reload | Automatic in dev mode |

**Runtime: 44KB uncompressed / 11KB gzipped.** One `<script>` tag. No npm.

---

## Add to your project

```bash
cargo add azumi-live-ssr-framework
```

## Or start a new project

```bash
cargo install azumi-cli
azumi new my-app
cd my-app && cargo run
```

---

## The Trilemma

Every other framework forces you to pick two of three. **Azumi gives you all three.**

| | Server-rendered | Interactive | No JS ecosystem |
|---|:---:|:---:|:---:|
| **HTMX** | ✅ | ❌ roundtrip | ✅ |
| **React** | ❌ hydration | ✅ | ❌ npm hell |
| **Leptos** | ❌ WASM | ✅ | ✅ WASM tax |
| **Azumi** | ✅ | ✅ ~11KB | ✅ |

---

## What Only Azumi Has

| Feature | Azumi | HTMX | Svelte | Leptos |
|---------|:---:|:---:|:---:|:---:|
| Compile-time CSS validation | ✅ | ❌ | ❌ | ❌ |
| Compile-time HTML validation | ✅ | ❌ | ❌ | ❌ |
| Compile-time route validation | ✅ | ❌ | ❌ | ❌ |
| Unconditional `Raw()` ban (XSS) | ✅ | ❌ | ❌ | ❌ |
| ARIA value validation | ✅ | ❌ | ❌ | ❌ |
| Auto-scoped CSS per component | ✅ | ❌ | ✅ | ❌ |
| Two-way binding without JS/WASM | ✅ | ❌ | ✅ (JS) | ✅ (WASM) |
| Keyed lists without JS/WASM | ✅ | ❌ | ✅ (JS) | ✅ (WASM) |
| Transitions without JS/WASM | ✅ | ❌ | ✅ (JS) | ❌ |
| Optimistic UI predictions | ✅ | ❌ | manual | ❌ |
| HMAC-signed component state | ✅ | ❌ | ❌ | ❌ |

---

## No Custom JavaScript

All interactive behavior is a directive on your HTML:

| Want this | Write this |
|---|---|
| Two-way binding | `<input bind:value={state.name} />` |
| Keyed list | `@for item in items @keyed(item.id)` |
| Smooth transitions | `<div az-transition:fade={"true"}>` |
| Form validation | `<input data-validate="email:required,email" />` |
| Confirm before delete | `<button az-confirm="Delete this?">` |
| Animate on scroll | `<section az-reveal={true}>` |
| Submit a form | `<form az-action={path} az-target={"#result"}>` |
| Tabs / counters / toggles | `az-ui` + `az-on` |

[dracon.dev](https://dracon.dev) runs 3 production sites on Azumi.

---

## Runtime Size (Real Measurements)

| Framework | JS Shipped (gzipped) |
|---|---|
| **Azumi** | **11KB** |
| HTMX 2.0 | 15KB |
| HTMX + Alpine | ~24KB |
| React 18 + ReactDOM | 46KB |
| Leptos (WASM) | 150KB+ |

Azumi ships **64% of HTMX's size** while adding client interactivity that HTMX lacks.

---

## When to Use Azumi (And When Not To)

✅ Dashboards, admin panels, e-commerce, content sites — anything that needs server-rendered HTML with real interactivity

✅ You're tired of maintaining a JS/TS frontend stack

✅ You want your app to still work in 5 years without a rewrite

❌ You need client-side routing (SPAs) — use Leptos or Dioxus

❌ You need Rust in the browser (canvas, editors) — use Leptos or Dioxus

---

## Azumi Is Complete

No ecosystem to keep up with. No framework rewrites every 2 years. No migration guides.

> React: class components → hooks → server components (3 rewrites in 8 years)
> Next.js: pages router → app router (massive migration)
> **Azumi: 0 rewrites. Your code works.**

Strict [Semantic Versioning](https://semver.org/). `azumi-live-ssr-framework = "47"` in `Cargo.toml` will never break your build.

---

## Documentation

| | |
|---|---|
| [Why Azumi](docs/why-azumi.md) | Origin story, competitive landscape, thesis |
| [Developer Guide](docs/guide.md) | Components, live state, forms, security |
| [Adding Azumi to Axum](docs/migration/from-axum.md) | 6-step incremental adoption |
| [Changelog](CHANGELOG.md) | Release history |

---

## License

Dual-licensed: **AGPL-3.0-only** (open source) or **Commercial License** ([COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md)). By contributing, you agree to [CLA.md](CLA.md).
