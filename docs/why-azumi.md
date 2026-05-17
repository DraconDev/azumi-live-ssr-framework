# Why Azumi

## The Origin Story

We built web apps in Rust. The backend was rock solid — memory safe, compiled, fast. But the frontend was always the problem.

First we tried the JavaScript route: React, Next.js, TypeScript. The type safety was nice, but the ecosystem was a treadmill. React rewrites every 2 years. Next.js changes the recommended pattern every 6 months. A `node_modules` with 1,200 packages. We spent more time maintaining the build than building the product.

Then we tried HTMX. Server-rendered HTML, no JS ecosystem, simple. It worked — until it didn't. Every toggle needs a network roundtrip. Every expand, every confirm dialog, every "are you sure?" — all waiting for the server. The app felt slow even when the server was fast. And for simple client-side state like a tab switch, there was no answer. You either accept the network latency or you write custom JavaScript — which puts you back in the ecosystem you were trying to escape.

**Azumi started as HTMX and improved upon it.** Same server-rendered approach. Same HTML-first philosophy. But the toggle works instantly. The reveal animates on scroll. The optimistic counter updates before the server confirms. 3KB of JavaScript handles what HTMX needs the network for.

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

Azumi's answer: server-rendered HTML where the common interactions — toggles, reveals, confirms, optimistic updates — work without a network roundtrip. 3KB of JS handles it. You never write that JS. You never maintain that JS. And because it's Rust, there's no ecosystem to keep up with.

---

## What "No Custom JS" Actually Means

In practice, Azumi apps ship **zero lines of custom JavaScript.** Every interactive behavior is a directive on your HTML:

```rust
html! {
    // Toggle a section — no JS, no network
    <button az-on:click="toggle:details">"Show details"</button>

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

| Framework | Approach | Stars | Azumi's Advantage | Azumi's Gap |
|-----------|----------|-------|-------------------|-------------|
| **HTMX** | HTML-over-the-wire | 40K+ | Client interactivity, compile-time validation, optimistic UI | Smaller community, fewer examples |
| **Leptos 0.8** | WASM + signals | 20K+ | No WASM download, no hydration, simpler mental model | Leptos has client-side routing, reactive state |
| **Dioxus 0.6** | WASM + virtual DOM | 35K+ | No WASM download, simpler, server-rendered by default | Dioxus has desktop/mobile, cross-platform |
| **SvelteKit 2** | Compiled JS + SSR | 80K+ | Rust type safety, zero JS by default, no npm | SvelteKit has richer ecosystem, Vite tooling |
| **Maud** | Rust HTML macros | 1.5K | Interactive components, CSS validation, ARIA, live state | Maud is simpler but dead-end for interactivity |
| **Next.js** | JS/TS full-stack | 130K+ | No ecosystem churn, compile-time validation, all Rust | Next.js has massive ecosystem, more examples |

**Nobody else occupies Azumi's exact position: server-rendered + interactive + no-JS-ecosystem.**

---

## The Deeper Value: Reliability Over Features

Azumi isn't trying to have the most features. It's trying to be the most reliable way to build a web app.

- **Compiler-validated HTML/CSS** — typos, missing classes, broken structure → compile errors
- **HMAC-signed state** — users can't tamper with serialized state
- **XSS prevention by default** — `Raw()` is blocked inside `html!`, auto-escaping everywhere
- **ARIA validation** — `aria-expanded="yes"` is a compile error (valid: `true`/`false`)
- **Route constants** — `href={about_page_ROUTE}` prevents typos at compile time
- **Zero custom JS** — the attack surface for XSS through your own JavaScript is literally zero

These aren't features you check off on a comparison matrix. They're the absence of entire categories of bugs. The bugs you don't write because the compiler won't let you.

That's the real value of "if it compiles, it works." Not as a marketing slogan, but as an engineering practice.
