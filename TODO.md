# Azumi — Master TODO

> Full-stack Rust web framework. Simple, secure, compiled.
> Born from: *"My backend is Rust — why is my frontend the thing that breaks?"*

---

## The North Star

A Rust full-stack framework where:

- **No second language** for the frontend. One compiler from DB schema to DOM attribute. No type-safety gap at the `fetch()` boundary.
- **No WASM download tax.** ~3KB runtime. HTML is truth. Interactivity is surgical, not the default.
- **Compiler catches frontend bugs.** CSS class typos, bad HTML nesting, missing alt text, wrong attributes — blocked at compile time, not in production.
- **Feels simple.** Not Leptos-complex (no reactivity model to learn). Not Dioxus-heavy (no WASM build pipeline). Just Rust + `html!` + attributes.

**The problem this solves:**
> *"My backend is Rust — rock solid, memory safe, compiled. My frontend is JS/TS — runtime errors, npm fragility, undefined is not a function. Why?"*

**Competitor positioning:**
| Competitor | Why not |
|-----------|---------|
| Next.js / SvelteKit | Language boundary at fetch(). Two type systems, two build pipelines, two ecosystems. Type safety breaks at the API call. |
| Leptos / Dioxus | WASM download tax (~150KB+). Complex reactivity model. DOM bridge overhead. Overkill for "form submit" and "button click." |
| HTMX | No type safety — everything is strings (URLs, swap targets, triggers). The compiler can't help you catch mistakes. |
| Maud | No interactivity story. Just templates — no form actions, no live state, no optimistic UI. |

**Differentiator:** Compile-time CSS-HTML co-validation + zero-hydration architecture + HMAC-signed state + ~3KB runtime + AI-ready validation pipeline.

---

## Progress

| Phase | Status | Last Updated |
|-------|--------|-------------|
| P0: Foundation | **2/4 complete** (azumi-cli ✅, routes! macro ✅) | 2026-05-12 |
| P1: API Polish | Not started | - |
| P2: Interactivity | Not started | - |
| P3: Production | Not started | - |
| P4: Ecosystem | Not started | - |
| Dracon Platform | Ongoing (tag bump ✅) | 2026-05-12 |

---

## Non-Goals (Things We Will NOT Do)

These come up repeatedly but are intentionally out of scope. Documented so we never waste time reconsidering them.

| Temptation | Why Not |
|-----------|---------|
| **ORM integration** | Pick sqlx/diesel/sea-orm independently. We provide patterns (demo), not packages. |
| **Authentication package** | Too many variants (sessions, JWT, OAuth, OIDC, magic links). Signed state is the foundation; let ecosystem handle auth. |
| **CSS framework integration** | Tailwind/Material/Bootstrap change too fast. Our compile-time validation is the right abstraction level. |
| **WASM / SSR hybrid** | Defeats "no WASM" philosophy. Point users to Leptos if they want WASM. We own "server-rendered with surgical interactivity." |
| **Client-side SPA routing** | MPA + Idiomorph is simpler, faster, no JS-for-navigation requirement. |
| **File-system based routing** | `routes!` macro is simpler, fully type-checked, no build scripts or filesystem coupling. |
| **Custom build tooling** | `cargo` works fine. No Vite/Rollup/Webpack equivalent needed. |
| **Separate template language** | `html!` IS the template language — it's Rust. No syntax highlighting issues, no editor plugins. |

---

## Phase 0: Foundation — Make It Tryable (Next Session)

Blockers that prevent anyone from picking up Azumi and having a working project in 5 minutes. Do these first, in order.

### P0.1 — `azumi new` Scaffolding CLI

**Problem:** No "hello world" flow. Currently: clone repo → read 8 docs → manually wire Axum → hope.

**Target:**
```bash
cargo install azumi-cli
azumi new my-app
cd my-app && cargo run
# → http://localhost:8080 — working page
```

**Generated project:**
- `main.rs` with Azumi + Axum wired (routes, actions, devtools)
- `src/routes/home.rs` — one page component using `azumi::routes!` style
- `src/components/` — one reusable component
- Client runtime (`azumi_script()`) in layout
- One form using `az-action` + `az-target`
- `.gitignore`, `Cargo.toml` with correct deps

**Deliverable:** `azumi-cli` crate, `cargo install`-able.
**Effort:** 1 session | **Deps:** None

---

### P0.2 — `azumi::routes!` Macro

**Problem:** Manual Axum Router wiring for every route = boilerplate that makes "full-stack" claim feel hollow.

**Target:**
```rust
azumi::routes! {
    "/" => HomePage,
    "/about" => AboutPage,
    "/products/:id" => ProductPage,
}
```

Auto-prepends `/azumi.js` and `/_azumi/` routes (actions, devtools, hot-reload). Supports dynamic segments and middleware.

**Effort:** 1 session (~100 lines of proc macro) | **Deps:** None

---

### P0.3 — Documentation Consolidation

**Problem:** 8+ markdown files (2,000+ lines). No clear entry point.

**Target structure:**
```
README.md          → Brand + 5-min quickstart (~200 lines)
docs/
├── guide.md       → Full guide (one story, ~3,000 words)
├── reference.md   → API reference
└── comparison.md  → Framework comparison
```

**Merge into guide.md:** AGENTS.md, AI_GUIDE_FOR_WRITING_AZUMI.md, AZUMI_DESCRIPTION.md, WHEN_TO_USE_AZUMI.md
**Keep:** CHANGELOG.md, TODO.md
**Move:** FRAMEWORK_COMPARISON.md → docs/comparison.md
**Effort:** 1 session | **Deps:** P0.1 + P0.2 (so quickstart reflects `azumi new`)

---

## Phase 1: API Polish — Make It Feel Simple (Next 2-3 Sessions)

Clean up the framework's API surface so "simple" isn't just a marketing claim.

### P1.1 — Merge `#[live]` + `#[live_impl]` into One Attribute

**Problem:** The #1 API design complaint. Two attributes for one component is confusing.

**Current:**
```rust
#[azumi::live] pub struct Counter { pub count: i32 }
#[azumi::live_impl(component = "counter_view")]
impl Counter { pub fn increment(&mut self) { self.count += 1; } }
```

**Target:**
```rust
#[azumi::live(component = "counter_view")]
pub struct Counter {
    pub count: i32,
    pub fn increment(&mut self) { self.count += 1; }
}
```

Keep `#[live_impl]` for backward compat, mark `#[doc(hidden)]`.
**Effort:** 1-2 sessions | **Deps:** None (best before P0.3)

---

### P1.2 — Reduce Visible API Surface

**Target: 10 macros → 5 visible, 6 traits → 2 visible, 33 functions → ~15 visible**

| Keep Public | Hide `#[doc(hidden)]` |
|------------|----------------------|
| `html!` | `head!` |
| `#[component]` | `#[page]` |
| `#[live]` (merged) | `#[live_impl]` |
| `#[action]` | `#[predict]` |
| `json_data!` | `from_fn()`, `from_fn_once()` |
| `Component` trait | `session_cleanup_script()` |
| `LiveState` trait | `Escaped`, `RenderWrapper` |
| `azumi_script()` | `FnComponent`, `FnOnceComponent` |
| `render_to_string()` | `HotReloadClosure` |
| `sign_state()`, `verify_state()` | `LiveStateMetadata` |
| `register_actions()` | `FallbackRender` |
| `success_fragment()`, `error_fragment()` | obscure script helpers |
| `compute_scope_id()`, `scope_css()` | |
| `escape_css_string()` | |

**Prelude shrinks:** Remove `head`, `live_impl`, `page`, `predict`, `session_cleanup_script`, `from_fn`, `FnComponent`
**Effort:** 1 session | **Deps:** P1.1

---

### P1.3 — Versioning Promise

**Problem:** v47 in 6 months = breaking changes every ~4 days = zero trust.

**What:** Ship v48.0.0 with semver promise:
- Major = actual breaking changes
- Minor = new features, backward compatible
- Patch = bug fixes only

Add to CHANGELOG.md and README.
**Effort:** 1 changelog entry | **Deps:** None

---

### P1.4 — `#[page]` Deprecation

Mark `#[page]` as `#[doc(hidden)]`. Thin wrapper around `#[component]`. Keep for backward compat.
**Effort:** 30 min | **Deps:** P1.2

---

## Phase 2: Interactivity — Make It Buildable (Next 3-5 Sessions)

Make the "modern with interactivity" claim production-ready.

### P2.1 — `#[azumi::action]` Server Handler Ergonomics

**Target — no boilerplate:**
```rust
#[azumi::action]
pub async fn save_profile(state: &AppState, form: SaveProfileForm) -> ActionResult {
    Ok(html! { <div class="success">"Saved!"</div> })
}
```

Auto-generates: State extraction, Form/Json parsing, IntoResponse wrapping, inventory registration.
**Effort:** 1 session | **Deps:** None

---

### P2.2 — Error Message Overhaul

**Current:** `error: expected '>'` (opaque, no fix suggestion)
**Target:** What went wrong + exact span + fix suggestion + docs link

**Scope:** Tokenizer errors (unclosed tags, unmatched braces), validator errors (CSS classes, HTML nesting, alt text, attributes), format!/Raw() blocks.
**Effort:** 2-3 sessions | **Deps:** None

---

### P2.3 — Client Feature Documentation + Patterns

**Deliverable:** `docs/interactivity.md` with:
- Decision tree: when to use each feature
- Pattern catalog: tabs, modals, accordion, live search, form submit, confirm dialogs, scroll animations
- Before/after: JS code → Azumi attribute (lines saved)
- Migration checklist: common JS patterns → Azumi equivalents

**Effort:** 1 session | **Deps:** P0.3 or parallel

---

### P2.4 — `head!` Macro Deprecation

Mark `#[doc(hidden)]`. Users use `<head>` in layout. Keep for backward compat.
**Effort:** 30 min | **Deps:** P1.2

---

## Phase 3: Production Readiness (Next 3-6 Months)

### P3.1 — Make Axum Optional
Split into `azumi-core` (zero deps: Component, html!, CSS scoping, security) + `azumi-axum` (register_actions, devtools, routes!) + `azumi` (meta-crate).
**Effort:** 2-3 sessions | **Deps:** P1.2 (stable API surface)

### P3.2 — Streaming HTML
Optional `async fn render_stream()` on Component. Sends headers immediately, renders incrementally.
**Effort:** 1-2 sessions

### P3.3 — Benchmark Suite Expansion
Add: macro expansion time, memory at scale (1000+ components), concurrent render throughput, WASM comparison, full-page pipeline.
**Effort:** 1 session

### P3.4 — Security Audit
Review HMAC signing, XSS edge cases, DOM clobbering. Expand proptest coverage. Document threat model.
**Effort:** 1 session

---

## Phase 4: Ecosystem Growth (Ongoing)

### P4.1 — Template Hot Reload (Page-Level)
Extend WebSocket to push full template patches via Idiomorph server-side diff.
**Effort:** 1-2 sessions

### P4.2 — Form Validation Helpers
`validated!` macro, auto-generated `aria-invalid`, integration with `error_fragment()`.
**Effort:** 1 session

### P4.3 — WebSocket/SSE Support
`az-sse` attribute: server pushes HTML fragments, Idiomorph patches them in.
**Effort:** 2 sessions

### P4.4 — Component Library Starter
`azumi-ui` crate: Button, Input, Card, Modal, Tabs, Accordion, Form — Azumi-native, unstyled.
**Effort:** 2-3 sessions

---

## Dracon Platform — Tactical Follow-ups

| Item | Status | Notes |
|------|--------|-------|
| DP.1 — Tag bump v47.19.10 → v47.20.20 | ✅ Done | Cargo.toml confirmed at v47.20.20 |
| DP.2 — Audit remaining TS for Azumi-native alternatives | ⏳ Next | Need to evaluate ai-hub-copy.ts (199 lines) |
| DP.3 — CSS consolidation (~18 patterns remain, all legit) | 🔍 Evaluated | No urgent action; consider `vstack-0` for gap:0 |
| DP.4 — TS pipeline monitoring (esbuild if >5 files) | 🔍 Monitoring | 2 TS files + 1 .d.ts — tsc is fine |
| DP.5 — Cross-repo integration tests | ⏳ Pending | Add cargo test --workspace to platform CI |

---

## Dependency Graph

```
P0.1 ──┬── P0.3     P2.1 ──┐
P0.2 ──┘            P2.2 ──┤── (parallel with P0)
                    P2.3 ──┘
P1.1 ──→ P1.2 ──→ P1.4
              │
              └──→ P3.1 (stable API required)

P3.2 ←── P3.1 or standalone
P4.x ───── (anytime, no blockers)
```

---

## Effort Summary

| Phase | Sessions | Weeks | Unlocks |
|-------|----------|-------|---------|
| P0 | ~3 | 1 | "I can try Azumi in 5 min" |
| P1 | ~3-4 | 1-2 | "This feels simple" |
| P2 | ~5-7 | 2-3 | "I can build a real app" |
| P3 | ~4-6 | 2-4 | "I'd bet my company on this" |
| P4 | Ongoing | Ongoing | "Competes with Next.js" |

---

*Last updated: 2026-05-12*
*Direction: Full-stack Rust framework — simple, secure, compiled. One language from DB to DOM.*
*"My backend is Rust — why is my frontend the thing that breaks?"*
