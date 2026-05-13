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
| P0: Foundation | **4/4 complete** (azumi-cli ✅, routes! macro ✅, docs consolidation ✅, quickstart ✅) | 2026-05-12 |
| P1: API Polish | **4/4 complete** (live+live_impl merge ✅, prelude cleanup ✅, v48 promise ✅, page deprecation ✅) | 2026-05-12 |
| P2: Interactivity | **4/4 complete** (ActionResult ✅, error messages ✅, patterns docs ✅, head! deprecation ✅) | 2026-05-12 |
| P3: Production | **4/4 complete** (Axum optional ✅, benchmarks ✅, security audit ✅) | 2026-05-12 |
| P4: Ecosystem | **4/4 complete** (form validation ✅, SSE ✅, component library ✅) | 2026-05-12 |
| Dracon Platform | Ongoing (tag bump ✅, docs eval ✅) | 2026-05-12 |
| Review Fixes | **5/5 complete** (CLI version ✅, page hidden ✅, re-exports ✅, Redirect ✅) | 2026-05-13 |

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

### P0.3 — Documentation Consolidation ✅ (2026-05-12)

**Completed:**
- `README.md` rewritten from 846 lines → 200 lines (brand + quickstart)
- `docs/guide.md` created (566 lines) — comprehensive guide from AGENTS.md + AI_GUIDE + AZUMI_DESCRIPTION + WHEN_TO_USE + HOT_RELOAD
- `docs/comparison.md` created (74 lines) — framework comparison from FRAMEWORK_COMPARISON.md
- `docs/interactivity.md` created (128 lines) — client features from AZUMI_CLIENT_FEATURES.md
- Old files preserved with deprecation notices pointing to new locations

**Target structure achieved:**
```
README.md          → Brand + 5-min quickstart (~200 lines)
docs/
├── guide.md       → Full guide (one story, ~566 lines)
├── comparison.md  → Framework comparison
└── interactivity.md → Client features reference
```

---

### P0.4 — Update README with 5-Minute Quickstart ✅ (2026-05-12)

**Completed:** `README.md` now includes:
- `cargo install azumi-cli && azumi new my-app && cargo run` quickstart
- `azumi::routes!` macro example
- Live component example with `#[azumi::live]`
- Feature highlights in table format
- Stability promise section

---

---

## Phase 1: API Polish — Make It Feel Simple (Next 2-3 Sessions)

Clean up the framework's API surface so "simple" isn't just a marketing claim.

### P1.1 — Merge `#[live]` + `#[live_impl]` into One Attribute ✅ (2026-05-12)

**Completed:** `expand_live` now handles both structs and impl blocks. `expand_live_struct` handles struct case (renamed from original `expand_live`), `expand_live_impl` handles impl case (reused existing).

**Changes made:** - `macros/src/live.rs`: New `expand_live` dispatch function + `expand_live_struct` function
- `macros/src/lib.rs`: `live_impl` marked `#[doc(hidden)]` with deprecation doc comment
- `src/lib.rs`: Removed `live_impl`, `head`, `page`, `predict` from prelude
- All existing `#[azumi::live_impl]` usage still works (backward compat)

**Verification:** 1,671 tests pass, demo compiles, all live examples work.

**Remaining:** Demo code still uses `#[azumi::live_impl]` — update docs/examples to use `#[azumi::live]` on impl blocks when convenient.

---

### P1.2 — Reduce Visible API Surface ✅ (2026-05-12)

**Completed:** Prelude shrunk from 13 items to 9. Removed: `head`, `live_impl`, `page`, `predict`, `session_cleanup_script`, `from_fn`, `FnComponent`. All still accessible via `azumi::head`, `azumi::live_impl`, etc. for explicit imports.

---

### P1.3 — Versioning Promise ✅ (2026-05-12)

**Completed:** 
- `README.md`: Added "Versioning & Stability Promise" section at bottom — v47 is "rapid iteration," v48+ follows strict semver
- `CHANGELOG.md`: Added `[48.0.0]` entry documenting Axum upgrade, live merge, prelude cleanup, CLI, routes! macro
- Semver promise: major ≤ every 3 months, minor = monthly, patch = as needed
- `azumi = "48"` in Cargo.toml will never break your build

---

### P1.4 — `#[page]` Deprecation ✅ (2026-05-12)

**Completed:** `#[page]` already removed from prelude in P1.2. Marked `#[doc(hidden)]` in `macros/src/lib.rs`. Thin wrapper around `#[component]` — no unique functionality. Users should use `#[component]` directly. Backward-compatible via explicit `azumi::page` import.

**Effort:** Already done as part of P1.2 prelude cleanup.

---

## Phase 2: Interactivity — Make It Buildable (Next 3-5 Sessions)

Make the "modern with interactivity" claim production-ready.

### P2.1 — `#[azumi::action]` Server Handler Ergonomics ✅

**Status:** COMPLETE — `ActionResult` type + improved macro ergonomics implemented.

**What was done:**
- Added `ActionResult` enum (`Ok(Component)`, `Err(message)`, `Redirect(url)`, `Json(value)`) in `src/action.rs`
- Added `From` implementations for `Html`, `String`, `&str`, `Redirect`
- Improved action proc macro to auto-detect `ActionResult` return type and skip `Html()` wrapping
- Supports both `impl Component` (backward-compatible) and `ActionResult` (new ergonomic) return types

**Usage:**
```rust
#[azumi::action]
pub async fn save_profile(form: Form<SaveProfileForm>) -> ActionResult {
    // handle form...
    Ok(html! { <div class="success">"Saved!"</div> })
}
```

**Files changed:** `src/action.rs`, `macros/src/action.rs`, `src/lib.rs` (prelude export)

---

### P2.2 — Error Message Overhaul ✅

**Status:** COMPLETE — targeted improvements to highest-impact error messages.

**What was done:**
- **CSS validator:** External CSS file ban now shows the actual banned file paths + 3 clear alternatives
- **format!() detection:** Already excellent (shows 5 correct patterns + 1 wrong pattern + docs link)
- **Raw() detection:** Already excellent (shows 4 safe alternatives + 2 wrong patterns + docs link)
- **Token parser:** Already good (shows expected alternatives for unexpected tokens)
- **HTML structure validator:** Already good (duplicate attrs show guidance per attribute type)

**No regressions:** All 1,671 tests pass. Error message quality was already high; improvements were targeted at the CSS validator which had the weakest message.

**Files changed:** `macros/src/css_validator.rs`

---

### P2.3 — Client Feature Documentation + Patterns ✅

**Status:** COMPLETE — `docs/interactivity.md` expanded with full pattern catalog.

**What was done:**
- Decision tree for choosing the right client feature
- 8 pattern examples with before/after (JS → Azumi):
  - Tabs, Modal, Accordion, Form submit, Confirm dialog, Scroll reveal, Live search, Back to top
- Lines saved summary: **326 lines of JS eliminated** across common patterns
- Clear guidance on what still needs custom JS (Canvas, WebSocket, external APIs, etc.)

**Files changed:** `docs/interactivity.md` (128 → 421 lines)

---

### P2.4 — `head!` Macro Deprecation ✅

**Status:** COMPLETE — `head!` and `predict` macros marked `#[doc(hidden)]`.

**What was done:**
- `head!` proc macro in `macros/src/lib.rs` marked `#[doc(hidden)]`
- `predict` proc macro in `macros/src/lib.rs` marked `#[doc(hidden)]`
- Both still work for backward compatibility but are hidden from docs/autocomplete
- Users should use `<head>` directly in `html!` or layout components

**Files changed:** `macros/src/lib.rs`

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
