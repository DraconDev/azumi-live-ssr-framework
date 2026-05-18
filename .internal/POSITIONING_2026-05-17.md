# Azumi Positioning Assessment (May 2026 — Updated)

## TL;DR

Azumi occupies a unique position: **server-rendered + interactive + no-JS-ecosystem**. No other framework gives you all three. The framework is 9/10 technically; the story was 5/10 but is now 8/10 after the doc rewrite.

---

## The Position (One-Liner)

> **Azumi: HTMX that actually works for interactive apps. No JS ecosystem, no ecosystem churn.**

### Subtitle

> **Server-rendered HTML with client interactivity — all Rust, zero custom JavaScript**

---

## The Thesis (3 Sentences)

1. **The frontend ecosystem is a treadmill.** React rewrites every 2 years. Next.js changes patterns every 6 months. Azumi is Rust, server-rendered, and complete — no ecosystem to maintain, no migration guides.

2. **Pure server-side (HTMX) has no interactivity.** Every toggle, expand, confirm = network roundtrip. Azumi adds client interactivity without JavaScript — ~10KB of runtime (gzipped) handles what HTMX needs the network for.

3. **The compiler catches what the browser can't.** CSS typos, broken HTML, XSS vectors, missing ARIA — caught at `cargo build`. But that's the safety net, not the reason you're here. You're here because you're done with the frontend treadmill.

---

## The Trilemma

Every other framework forces you to pick two of three:

| Need | HTMX / Rails | React / Next.js | Leptos / Dioxus | **Azumi** |
|------|:---:|:---:|:---:|:---:|
| Server-rendered, simple | ✅ | ❌ hydration | ❌ WASM | ✅ |
| Client interactivity | ❌ network roundtrip | ✅ | ✅ | ✅ ~10KB runtime |
| No JS ecosystem churn | ✅ | ❌ npm/TS hell | ✅ but WASM tax | ✅ |

**Azumi is the only option that gives you all three.**

---

## Competitive Landscape (May 2026)

| Framework | Approach | Stars | Azumi's Advantage | Azumi's Gap |
|-----------|----------|-------|-------------------|-------------|
| **HTMX** | HTML-over-the-wire | 40K+ | Client interactivity, compile-time validation, optimistic UI | Smaller community |
| **Leptos 0.8** | WASM + signals | 20K+ | No WASM download, zero hydration, simpler model | Leptos has client-side routing, reactive state |
| **Dioxus 0.6** | WASM + virtual DOM | 35K+ | No WASM download, simpler, server-rendered | Dioxus has desktop/mobile |
| **SvelteKit 2** | Compiled JS + SSR | 80K+ | Rust type safety, zero JS by default, no npm | Richer ecosystem |
| **Maud** | Rust HTML macros | 1.5K | Interactive components, CSS/ARIA validation, live state | Simpler but dead-end for interactivity |
| **Next.js** | JS/TS full-stack | 130K+ | No ecosystem churn, compile-time validation, all Rust | Massive ecosystem |

**Nobody else occupies Azumi's exact position: server-rendered + interactive + no-JS-ecosystem.**

### Runtime Size (Real Measurements, gzipped)

| Framework | JS Shipped | Notes |
|-----------|-----------|-------|
| **Azumi** | **10KB** | azumi.js + idiomorph.js |
| HTMX 2.0 | 15KB | No client interactivity |
| Alpine.js 3.x | 15KB | Client interactivity, but you write JS |
| React 18 + ReactDOM | 46KB | Full SPA overhead |
| Leptos (WASM) | 150KB+ | WASM binary + runtime |

Azumi is **64% of HTMX's size** while adding client interactivity. **22% of React's size.**

---

## Production Proof

[dracon.dev](https://dracon.dev) runs 3 production sites on Azumi:
- 98 `html!` macro calls
- Zero lines of custom JavaScript
- The only JS is third-party integrations (Paddle, analytics)
- HMAC-signed state for checkout, form validation, product filtering

---

## What Makes Azumi Unique (Feature Audit)

| Feature | Azumi | HTMX | React | Leptos | Maud |
|---------|:---:|:---:|:---:|:---:|:---:|
| Server-rendered HTML | ✅ | ✅ | SSR only | SSR | ✅ |
| Client interactivity (no JS) | ✅ | ❌ | ✅ (JS) | ✅ (WASM) | ❌ |
| Compile-time CSS validation | ✅ | ❌ | ❌ | ❌ | ❌ |
| Compile-time HTML validation | ✅ | ❌ | ❌ | ❌ | ❌ |
| XSS prevention (Raw ban) | ✅ | ❌ | ❌ | ❌ | ❌ |
| ARIA value validation | ✅ | ❌ | ❌ | ❌ | ❌ |
| Route constant safety | ✅ | ❌ | ❌ | ❌ | ❌ |
| Optimistic UI predictions | ✅ | ❌ | manual | ❌ | ❌ |
| HMAC-signed state | ✅ | ❌ | ❌ | ❌ | ❌ |
| Borrow-friendly component props | ✅ | — | — | — | ❌ |
| Zero JS ecosystem | ✅ | ✅ | ❌ | ✅ | ✅ |
| No WASM download | ✅ | ✅ | ✅ | ❌ | ✅ |

**Azumi has 6 features that no other framework has.** The compile-time validation pipeline (CSS, HTML, XSS, ARIA, routes) is genuinely unique.

---

## The Real Weaknesses (Honest Assessment)

### Fundamental
1. **No client-side routing** — SPAs with instant page transitions are impossible
2. **No WASM story** — Can't run Rust in the browser for complex client logic
3. **Rust adoption curve** — Target audience must already be Rust-curious

### Practical
4. **Single maintainer risk** — Only one known production user (dracon-platform)
5. **No npm install story** — Rust toolchain is both a moat and a wall
6. **Interactive ceiling** — Complex multi-step client state still needs custom JS

### Mitigated (Previously Listed, Now Addressed)
- ~~"No benchmarks vs alternatives"~~ → Real runtime size comparison added (Azumi 10KB vs HTMX 15KB vs React 46KB)
- ~~"No 'Why Azumi' page"~~ → `docs/why-azumi.md` created
- ~~"Too many docs, no narrative"~~ → 12 docs consolidated to 3
- ~~"Missing 'stability as a feature'"~~ → Full section in README + why-azumi
- ~~"Missing 'no custom JS' proof"~~ → dracon-platform audit data included

---

## The Three Tiers of Pitch

**Tier 1 — For the HTMX user who hit the interactivity wall:**
> "You chose HTMX because server-rendered HTML is simple. But every toggle needs a server roundtrip, reveals need the network, and optimistic UI is impossible. Azumi is the same approach — server-rendered, HTML-first — but the toggle works instantly. ~10KB of JS (gzipped), no WASM, no build step."

**Tier 2 — For the developer tired of the frontend treadmill:**
> "React rewrites every 2 years. Next.js changes patterns every 6 months. Your node_modules has 1,200 packages. Azumi is Rust, server-rendered, and complete. No ecosystem to maintain. No migration guides. Build your product, not your toolchain."

**Tier 3 — For the AI-assisted developer:**
> "AIs are good at TypeScript — but the TS ecosystem is still a mess. Azumi's strict rules mean AI-generated code is correct by default. If the AI hallucinates a bad pattern, the compiler catches it. And you never have to update the framework."

---

## What NOT to Position As

- **Don't say "full-stack"** — Server-first with surgical interactivity. Own it.
- **Don't compete with Leptos/Dioxus on features** — Compete on simplicity and no-WASM/no-JS-eco.
- **Don't say "zero JS"** — You ship ~10KB gzipped. That's "minimal JS" or "near-zero JS".
- **Don't claim SPA capability** — No client-side routing. Be honest about limits.
- **Don't bury the interactivity story** — This is the #1 differentiator vs HTMX/Maud.

---

## Bull Case

1. **The market is moving toward server-first.** HTMX proved there's appetite. Astro proved islands work. SvelteKit added `query.live`. Less JS is the trend.
2. **AI coding is the biggest shift since git.** A framework that makes AI-generated code correct by default has enormous value. Azumi's compile-time validation is the strongest moat.
3. **Rust web adoption is growing.** Leptos 20K stars, Dioxus 35K — people want Rust for the web. A simpler server-first option fills a real gap.
4. **dracon-platform proves it works at scale.** 3 production sites, 98 `html!` calls, real users.
5. **The compile-time validation moat is deep.** Nobody else validates CSS, HTML, ARIA, and XSS at compile time.

## Bear Case

1. **Rust adoption is still niche for web.** Most web devs don't know Rust.
2. **The interactive ceiling is real.** Complex client state → wall.
3. **Community size matters more than quality.** 35K stars beats technical superiority.
4. **"If it compiles, it works" is a hard promise.** One runtime bug undermines the thesis.

---

## Doc Status

| Document | Status | Purpose |
|----------|--------|---------|
| `README.md` | ✅ Rewritten | 30-second pitch + trilemma + quickstart + "when to use" |
| `docs/why-azumi.md` | ✅ Rewritten | Full narrative: origin, thesis, landscape, scope, stability |
| `docs/guide.md` | ✅ Updated | Developer guide with borrowed props, TrustedHtml, interactivity |
| `docs/migration/from-axum.md` | ✅ New | 6-step incremental adoption path |
| `AGENTS.md` | ✅ Updated | AI-facing guide: route constants, borrowed props, live_state |
| `CHANGELOG.md` | ✅ Updated | v47.44 entry with all recent features |
| 10 archived docs | ✅ Archived | Internal docs moved to `docs/archive/` |

**Framework: 9/10 technically. Story: 8/10.** (Up from 5/10 after doc rewrite.)
