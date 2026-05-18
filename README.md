# Azumi

**Live SSR for Rust — server-rendered HTML with instant interactivity. No JavaScript. No WASM. No ecosystem churn.**

---

## You know the problem.

You picked HTMX because server-rendered HTML is simple. But every toggle needs a server roundtrip. Every "are you sure?" waits for the network. Your app feels slow even when the server is fast.

Or you picked React and got the interactivity — but also 1,200 packages in `node_modules`, a rewrite every two years, and two languages to maintain.

**Azumi gives you both.** Server-rendered HTML where the toggle works *instantly*. The reveal animates on scroll. The optimistic counter updates before the server confirms. ~10KB of runtime (gzipped). Zero custom JavaScript written by you.

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

## Add to your project

```bash
cargo add azumi
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
| **Azumi** | ✅ | ✅ ~10KB | ✅ |

---

## What Only Azumi Has

7 features that exist in no other framework:

| | Azumi | HTMX | React | Leptos | Maud |
|---|:---:|:---:|:---:|:---:|:---:|
| Compile-time CSS validation | ✅ | ❌ | ❌ | ❌ | ❌ |
| Compile-time HTML validation | ✅ | ❌ | ❌ | ❌ | ❌ |
| Unconditional `Raw()` ban (XSS) | ✅ | ❌ | ❌ | ❌ | ❌ |
| ARIA value validation | ✅ | ❌ | ❌ | ❌ | ❌ |
| Route constant safety | ✅ | ❌ | ❌ | ❌ | ❌ |
| Optimistic UI predictions | ✅ | ❌ | manual | ❌ | ❌ |
| HMAC-signed component state | ✅ | ❌ | ❌ | ❌ | ❌ |

---

## No Custom JavaScript — Production Proof

All interactive behavior is a directive on your HTML:

| Want this | Write this | No JS? |
|---|---|---|
| Toggle a section | `<button az-on:click="toggle:details">` | ✅ |
| Confirm before delete | `<button az-confirm="Delete this?">` | ✅ |
| Animate on scroll | `<section az-reveal={true}>` | ✅ |
| Submit a form | `<form az-action={path} az-target={"#result"}>` | ✅ |
| Tabs / counters / reveals | `az-ui` + `az-on` | ✅ |

[dracon.dev](https://dracon.dev) runs 3 production sites on Azumi — **98 `html!` calls, zero lines of custom JavaScript.** The only JS is third-party integrations (Paddle, analytics).

---

## Runtime Size (Real Measurements)

| Framework | JS Shipped (gzipped) |
|---|---|
| **Azumi** | **10KB** |
| HTMX 2.0 | 15KB |
| Alpine.js 3.x | 15KB |
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

Strict [Semantic Versioning](https://semver.org/). `azumi = "48"` in `Cargo.toml` will never break your build.

---

## Documentation

| | |
|---|---|
| [Why Azumi](docs/why-azumi.md) | Origin story, competitive landscape, thesis |
| [Developer Guide](docs/guide.md) | Components, live state, forms, security |
| [Migrating from Axum](docs/migration/from-axum.md) | 6-step incremental adoption |
| [Changelog](CHANGELOG.md) | Release history |

---

## License

Dual-licensed: **AGPL-3.0-only** (open source) or **Commercial License** ([COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md)). By contributing, you agree to [CLA.md](CLA.md).
