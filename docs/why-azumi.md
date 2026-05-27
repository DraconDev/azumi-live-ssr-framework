# Why Azumi

## The Origin Story

We built web apps in Rust. The backend was rock solid — memory safe, compiled, fast. But the frontend was always the problem.

First we tried the JavaScript route: React, Next.js, TypeScript. The type safety was nice, but the ecosystem was a treadmill. React rewrites every 2 years. Next.js changes the recommended pattern every 6 months. A `node_modules` with 1,200 packages. We spent more time maintaining the build than building the product.

Then we tried HTMX. Server-rendered HTML, no JS ecosystem, simple. It worked — until it didn't. Every toggle needs a network roundtrip. Every expand, every confirm dialog, every "are you sure?" — all waiting for the server. The app felt slow even when the server was fast. And for simple client-side state like a tab switch, there was no answer. You either accept the network latency or you write custom JavaScript — which puts you back in the ecosystem you were trying to escape.

**Azumi started as HTMX and improved upon it.** Same server-rendered approach. Same HTML-first philosophy. But the toggle works instantly. The reveal animates on scroll. The optimistic counter updates before the server confirms. ~11KB of JavaScript (gzipped) handles what HTMX needs the network for.

---

## The Two Problems

### Problem 1: The Frontend Ecosystem Is a Treadmill

| Year | What React Told You To Do |
|------|---------------------------|
| 2016 | Use class components |
| 2019 | No, use hooks |
| 2022 | No, use server components |
| 2024 | No, use server actions |
| 2025 | No, use... |

Every framework migration means rewriting your app. Every `npm install` is a security audit waiting to happen. Every `package.json` update is a potential breakage. TypeScript helps, but the ecosystem around it is still fragile.

**The real cost isn't the rewrite — it's the uncertainty.** You never know when the pattern you're using will be deprecated. You never know if your dependencies will abandon you. You're renting your architecture instead of owning it.

### Problem 2: Pure Server-Side Has No Interactivity

HTMX proved that server-rendered HTML works. No JS framework, no build step, no hydration. Just HTML over the wire.

But there's a ceiling. HTMX can't:
- Toggle a menu without a network request
- Expand a section without a network request
- Show an "are you sure?" dialog without a network request (unless you write custom JS)
- Do optimistic UI (show the result before the server confirms)
- Animate elements on scroll

For a documentation site, HTMX is perfect. For an interactive app — a dashboard, admin panel, e-commerce checkout — the constant network roundtrips make the UI feel sluggish.

---

## The Thesis

> **Server-rendered HTML + client interactivity + no JS ecosystem. Pick all three.**

Every other framework makes you choose:

| Approach | Gets | Sacrifices |
|----------|------|-----------|
| **HTMX / Rails / Maud** | Server simplicity, no JS | No client interactivity — every interaction = network roundtrip |
| **React / SvelteKit / Next.js** | Rich interactivity | JS/TS ecosystem churn, runtime errors, two languages |
| **Leptos / Dioxus** | One language (Rust), interactivity | WASM download tax (~150KB+), DOM bridge overhead |

Azumi's answer: server-rendered HTML where the common interactions — toggles, reveals, confirms, optimistic updates — work without a network roundtrip. ~11KB of JS handles it. You never write that JS. You never maintain that JS. And because it's Rust, there's no ecosystem to keep up with.

---

## What "No Custom JS" Actually Means

In practice, Azumi apps ship **zero lines of custom JavaScript.** Every interactive behavior is a directive on your HTML:

```rust
html! {
    // Two-way input binding — no JS, no manual on:input
    <input type="text" bind:value={state.name} />
    <input type="checkbox" bind:checked={state.agree} />

    // Keyed list — items tracked by identity, smooth reordering
    @for item in &items @keyed(item.id) {
        <div az-transition:fade={"true"}>{&item.name}</div>
    }

    // Smooth transitions — fade, slide, scale on DOM enter/exit
    <div az-transition:slide={"true"}>"I slide in smoothly"</div>

    // Client-side form validation — 8 rules, zero custom JS
    <input data-validate="email:required,email" />

    // Confirm before action — no JS, no network
    <button az-confirm="Delete this item?">"Delete"</button>

    // Scroll reveal animation — no JS
    <section az-reveal={true}>"Appears on scroll"</section>

    // Optimistic form submit — no custom JS
    <form az-action={like_post_PATH} az-target={"#like-area"}>
        <button>"Like"</button>
    </form>
}
```

The only JavaScript in a production Azumi app comes from **third-party integrations** that require it (payment providers, analytics, etc.) — the same situation you'd have in any framework.

---

## When to Use Azumi

✅ **Dashboards and admin panels** — interactive state, form actions, optimistic updates, all server-rendered

✅ **E-commerce** — HMAC-signed state for checkout, form validation, product filtering without page reloads

✅ **Content sites** — server-rendered SEO, scroll reveals, zero JS by default

✅ **Internal tools** — rapid development, Rust type safety, no frontend build step

✅ **Apps where you're tired of React** — migration from JS to all-Rust, stable API, no ecosystem churn

## When NOT to Use Azumi

❌ **You need client-side routing** — Azumi doesn't do SPA-style instant page transitions. Every navigation is a server request. If you need an SPA, use Leptos or Dioxus.

❌ **You need to run Rust in the browser** — Azumi doesn't compile to WASM. If you need canvas, WebGL, or a full browser-based editor, use Leptos or Dioxus.

❌ **Your team doesn't know Rust** — Azumi won't teach you Rust. If your team is JS-first and happy with it, SvelteKit is probably a better choice.

---

## Azumi Is Complete

This is worth calling out explicitly: **Azumi is not a framework you need to keep up with.**

There's no ecosystem of plugins, no plugin marketplace, no competing state management libraries, no "which CSS-in-JS solution should I use?" debates. It does what it does and it's done. We'll add features and fix bugs, but you won't need to rewrite your app to stay current.

**Compare:**
- React: class components → hooks → server components (3 rewrites in 8 years)
- Next.js: pages router → app router (1 rewrite, massive migration)
- Angular: 6 major rewrites
- **Azumi: 0 rewrites. Your code works.**

Build your product, not your toolchain.

---

## Competitive Landscape

### The Alternatives

| Approach | Gets | Sacrifices |
|----------|------|-----------|
| **HTMX + Alpine** | Server simplicity, some client state | No components, no scoped CSS, no compile safety, two libraries |
| **SvelteKit** | Premium DX, transitions, ecosystem | Separate frontend repo, npm, JS/TS, two languages |
| **Leptos** | One language (Rust), signals | WASM download (~150KB+), slow compiles, no ecosystem |
| **Azumi** | Components, scoped CSS, transitions, two-way binding, compile safety, no npm | Smaller community, fewer examples |

### Feature Comparison

| Feature | Azumi | HTMX+Alpine | Svelte | Leptos |
|---------|:---:|:---:|:---:|:---:|
| Components + typed props | ✅ | ❌ | ✅ | ✅ |
| Two-way binding | ✅ `bind:value` | ⚠️ `x-model` | ✅ `bind:` | ✅ `bind:` |
| Keyed list updates | ✅ `@keyed` | ❌ | ✅ `{#each (id)}` | ✅ `<For>` |
| Scoped CSS (auto) | ✅ | ❌ | ✅ | ❌ |
| Transitions (fade/slide/scale) | ✅ | ❌ | ✅ | ❌ |
| Optimistic UI | ✅ `data-predict` | ❌ | ✅ manual | ❌ |
| Form validation (built-in) | ✅ 8 rules | ❌ | ✅ | ❌ |
| Compile-time CSS validation | ✅ | ❌ | ❌ | ❌ |
| Compile-time route validation | ✅ | ❌ | ❌ | ❌ |
| Compile-time ARIA validation | ✅ | ❌ | ❌ | ❌ |
| Single `cargo build`, no npm | ✅ | ⚠️ need HTMX CDN | ❌ | ✅ |
| Ecosystem | 🔴 Tiny | 🟢 Large | 🟢 Massive | 🔴 Tiny |

**Nobody else offers: components + scoped CSS + transitions + two-way binding + keyed lists + compile-time validation — all from `cargo build` with zero npm.**

### Runtime Size Comparison (gzipped)

| Framework | JS Shipped (gzipped) | Notes |
|-----------|---------------------|-------|
| **Azumi** | **11KB** | idiomorph + runtime (transitions, validation, bindings, predictions) |
| HTMX 2.0 | 15KB | No client interactivity, no transitions, no validation |
| HTMX + Alpine | ~24KB | Two libraries, no components, no scoped CSS |
| Alpine.js 3.x | 10KB | Client interactivity only |
| Svelte | ~2KB | Compiled JS, no runtime |
| React 18 + ReactDOM | 46KB | Full SPA overhead |
| Leptos (WASM) | 150KB+ | WASM binary + runtime |

---

## The Deeper Value: Reliability Over Features

Azumi isn't trying to have the most features. It's trying to be the most reliable way to build a web app.

- **Compiler-validated HTML/CSS** — typos, missing classes, broken structure → compile errors
- **HMAC-signed state** — users can't tamper with serialized state
- **XSS prevention by default** — `Raw()` is blocked inside `html!`, auto-escaping everywhere
- **ARIA validation** — `aria-expanded="yes"` is a compile error (valid: `true`/`false`)
- **Route constants** — `href={about_page_ROUTE}` prevents typos at compile time
- **Borrow-friendly props** — `&str` parameters in components, zero `.clone()` from render methods
- **Zero custom JS** — the attack surface for XSS through your own JavaScript is literally zero

These aren't features you check off on a comparison matrix. They're the absence of entire categories of bugs. The bugs you don't write because the compiler won't let you.

That's the real value of "if it compiles, it works." Not as a marketing slogan, but as an engineering practice.

---

## What Only Azumi Has

These features exist in no other framework — Rust or otherwise:

| Feature | Azumi | HTMX | Svelte | Leptos |
|---------|:---:|:---:|:---:|:---:|
| Compile-time CSS validation | ✅ | ❌ | ❌ | ❌ |
| Compile-time HTML validation | ✅ | ❌ | ❌ | ❌ |
| Compile-time route validation | ✅ | ❌ | ❌ | ❌ |
| Unconditional `Raw()` ban | ✅ | ❌ | ❌ | ❌ |
| ARIA value validation | ✅ | ❌ | ❌ | ❌ |
| Auto-scoped CSS per component | ✅ | ❌ | ✅ | ❌ |
| Two-way binding without JS/WASM | ✅ | ❌ | ✅ (JS) | ✅ (WASM) |
| Keyed list updates without JS/WASM | ✅ | ❌ | ✅ (JS) | ✅ (WASM) |
| Transitions without JS/WASM | ✅ | ❌ | ✅ (JS) | ❌ |
| Optimistic UI predictions | ✅ | ❌ | manual | ❌ |
| HMAC-signed component state | ✅ | ❌ | ❌ | ❌ |
| Client interactivity (no JS/WASM) | ✅ | ❌ | ✅ (JS) | ✅ (WASM) |
| Zero npm, single `cargo build` | ✅ | ✅ | ❌ | ✅ |
