# Azumi — Master TODO

> Full-stack Rust web framework. Simple, secure, compiled.
> Born from: "My backend is Rust — why is my frontend the thing that breaks?"

---

## Phase 0: Foundation — Make It Tryable (Next Session)

These are the **blockers** that prevent anyone from picking up Azumi and having a working project in 5 minutes. Do these first, in order.

### P0.1 — `azumi new` Scaffolding CLI

**Problem:** No "hello world" flow. Currently requires: clone repo → read 8 docs → manually wire Axum → hope it works.

**What:**
```bash
cargo install azumi-cli
azumi new my-app
cd my-app && cargo run
# → http://localhost:8080 — working page with one form, one button
```

**Generated project should include:**
- `main.rs` with Azumi + Axum wired correctly (routes, actions, devtools)
- `src/routes/home.rs` — one page component
- `src/components/` — one reusable component
- Client runtime (`azumi_script()`) already in the layout
- One form using `az-action` + `az-target`
- `.gitignore`, `Cargo.toml` with correct deps

**Deliverable:** A `cargo install`-able binary that generates the scaffolding.

**Estimated effort:** 1 session

**Dependencies:** None — standalone crate, no framework changes needed.

---

### P0.2 — `azumi::routes!` Macro

**Problem:** Every Azumi project requires manual Axum Router wiring. This is boilerplate that turns away newcomers and makes "full-stack framework" claim feel hollow.

**What:**
```rust
// Before: 10 lines of manual wiring
let app = Router::new()
    .route("/", get(|| async { html! { <h1>"Home"</h1> } }))
    .route("/about", get(|| async { html! { <h1>"About"</h1> } }));

// After: 4 lines, declarative, no boilerplate
azumi::routes! {
    "/" => HomePage,
    "/about" => AboutPage,
}
```

**Requirements:**
- Generates an Axum `Router` from a macro invocation
- Each route maps a path to a component (which implements `Component`)
- Supports nested routes (e.g., `/products/:id`)
- Supports middleware (e.g., `azumi::routes! { ... }.layer(auth_middleware)`)
- Prepends `/azumi.js` and `/_azumi/` routes automatically (actions, devtools, etc.)

**Estimated effort:** 1 session (macro ~50-100 lines)

**Dependencies:** None — pure proc macro in `azumi-macros`.

---

### P0.3 — Documentation Consolidation

**Problem:** 8+ markdown files in the root. Users don't know where to start. The framework story is scattered.

**Target structure:**
```
README.md          → Brand + 5-minute quickstart (~200 lines, down from 827)
docs/
├── guide.md       → Full framework guide (one story, one flow, ~3000 words)
└── reference.md   → API reference (auto-generated or maintained)
```

**Files to merge/delete:**
| File | Current | Fate |
|------|---------|------|
| `README.md` | 827 lines | Trim to 200 lines — just brand + quickstart |
| `AGENTS.md` | ~150 lines | Merge into `docs/guide.md` |
| `AI_GUIDE_FOR_WRITING_AZUMI.md` | ~250 lines | Merge into `docs/guide.md` |
| `AZUMI_DESCRIPTION.md` | 241 lines | Merge into `docs/guide.md` |
| `FRAMEWORK_COMPARISON.md` | 218 lines | Move to `docs/comparison.md` |
| `WHEN_TO_USE_AZUMI.md` | ~100 lines | Merge into `docs/guide.md` |
| `CHANGELOG.md` | 287 lines | Keep as-is |
| `TODO.md` | This file | Keep as-is |

**Estimated effort:** 1 session

**Dependencies:** Should be done AFTER P0.1 and P0.2 so the quickstart reflects the actual `azumi new` + `azumi::routes!` experience.

---

## Phase 1: API Polish — Make It Feel Simple (Next 2-3 Sessions)

These clean up the framework's API surface so "simple" isn't just a marketing claim.

### P1.1 — Merge `#[live]` + `#[live_impl]` into One Attribute

**Problem:** The #1 API design complaint. Two attributes for one component is confusing and doesn't match the "simple" promise.

```rust
// Current (two macros, confusing coupling)
#[azumi::live]
pub struct Counter { pub count: i32 }

#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) { self.count += 1; }
}

// Target (one macro, everything in one place)
#[azumi::live(component = "counter_view")]
pub struct Counter {
    pub count: i32,
    pub fn increment(&mut self) { self.count += 1; }
}
```

**Requirements:**
- The struct and its impl block are defined together (Rust allows this in a single `#[attr]` on a module-within-item, or via `TokenStream` manipulation)
- `LiveStateMetadata`, `LiveState` traits still generated internally — just hidden from the user
- Existing `#[azumi::live_impl]` kept for backward compat but marked `#[doc(hidden)]`
- All 20 demo lessons updated

**Estimated effort:** 1-2 sessions

**Technical approach:**
- The attribute on the struct captures both the struct def and any `impl` blocks that follow
- OR: use a helper macro that takes both struct and impl as arguments (less ergonomic but simpler to implement)

---

### P1.2 — Reduce Visible API Surface

**Problem:** 10 proc macros, 6 traits, 33 functions, 19 structs. Users see "complex" before "simple."

**Target visible surface:**
| Keep Public | Hide (`#[doc(hidden)]`) | Remove |
|------------|------------------------|--------|
| `html!` | `head!` | N/A |
| `#[component]` | `#[page]` (⊂ #[component]) | |
| `#[live]` (merged) | `#[predict]` (no-op) | |
| `#[action]` | `from_fn()`, `from_fn_once()` | |
| `json_data!` | `Raw<T>`, `TrustedHtml` | |
| `Component` trait | `LiveStateMetadata` | |
| `LiveState` trait | `FallbackRender` (merge into Component) | |
| `azumi_script()` | `session_cleanup_script()` | |
| `register_actions()` | 3+ obscure helper functions | |
| `sign_state()`, `verify_state()` | | |
| `success_fragment()`, `error_fragment()` | | |

**Estimated effort:** 1 session

---

### P1.3 — Versioning Promise

**Problem:** v47 in 6 months = breaking changes every 4 days = zero trust.

**What:**
- Ship next release as `v48.0.0` (incrementals are fine, just label it clearly)
- Publish a versioning policy:
  - Major = actual breaking changes (API removal, behavior change)
  - Minor = new features, no breaking changes
  - Patch = bug fixes only
- Add this to `CHANGELOG.md` and `README.md`

**Estimated effort:** 1 changelog entry + a few sentences in README

---

## Phase 2: Interactivity — Make It Modern (Next 3-5 Sessions)

These make the "modern with interactivity" claim production-ready.

### P2.1 — `az-action` Form Handler Ergonomics

**Problem:** Server action handlers require too much boilerplate (State extraction, Form parsing, IntoResponse wrapping).

**Target API:**
```rust
#[azumi::action]
pub async fn save_profile(
    state: &AppState,
    form: SaveProfileForm,
) -> ActionResult {
    // business logic only — no boilerplate
    Ok(html! { <div class="success">"Saved!"</div> })
}
```

**What's needed:**
- Let `#[azumi::action]` auto-generate the Axum handler boilerplate (State extraction, Form parsing, IntoResponse)
- Support standard success/error return types (`Result<Component, String>` → wrapped in `success_fragment`/`error_fragment`)
- Auto-register via `inventory` (like the current `#[action]` does)

**Estimated effort:** 1 session

---

### P2.2 — Error Message Overhaul

**Problem:** The `html!` macro produces opaque tokenizer errors. When an AI writes bad code, the error should say *what* went wrong, *where*, and *what to do instead*.

**Current:**
```
error: expected `>`
  --> src/lib.rs:10:25
   |
10 |     <div class={container>"</div>
   |                         ^
```

**Target:**
```
error: Missing closing `>` on opening tag
  --> src/lib.rs:10:25
   |
10 |     <div class={container>"</div>
   |                        ^ expected `>` here
   |
help: Did you mean?
     <div class={container}>"</div>
```

**Scope:**
- Tokenizer errors (unclosed tags, unexpected characters, unmatched braces)
- Validator errors (CSS class not found, wrong HTML nesting, missing alt text)
- Each error should include: what happened, exact span, fix suggestion

**Estimated effort:** 2-3 sessions (high-value, spans multiple validator modules)

---

### P2.3 — Client Feature Documentation + Patterns

**Problem:** The client features (`az-action`, `az-confirm`, `az-init`, `az-reveal`, `az-bind`, `az-ui`) exist and work, but:
1. No single document shows all of them with examples
2. Common patterns (tabs, modals, accordion, live search) aren't documented
3. Platform team defaults to JS because they don't know Azumi can do it

**Deliverable:** A `docs/interactivity.md` that covers:
- Decision tree: when to use each feature
- Pattern catalog: tabs, modals, accordion, live search, form submit, confirm dialogs, scroll animations
- Before/after: JS code → Azumi attribute (demonstrates the savings)
- Migration checklist: common JS patterns and their Azumi equivalents

**Estimated effort:** 1 session

---

## Phase 3: Production Readiness (Next 3-6 Months)

These make Azumi a framework you'd bet a production SaaS on.

### P3.1 — Make Axum Optional

**Problem:** `axum` is a hard dependency. Can't use with Actix, Warp, Tower directly.

**What:**
- Split `Component` trait and `render_to_string()` into a `azumi-core` crate with zero framework dependencies
- Move Axum-specific features (`register_actions`, devtools router, hot_reload) into a `azumi-axum` adapter crate
- Keep `azumi` as the "batteries-included" meta-crate that re-exports both

**Estimated effort:** 2-3 sessions

**Dependencies:** Major refactor, best done after API surface is stable (Phase 1).

---

### P3.2 — Streaming HTML

**Problem:** Large pages block the response until fully rendered. Streaming would send headers immediately and render incrementally.

**What:**
- `Component::render()` currently writes to `fmt::Formatter` (synchronous)
- Add an async `render_stream()` method that writes chunks as they're ready
- Useful for: pages with multiple sections, slow data sources, streaming layouts

**Estimated effort:** 1-2 sessions

---

### P3.3 — Benchmark Suite Expansion

**Problem:** Current benchmarks cover escape, render, and CSS scoping. Missing:
- Macro expansion time (how long does `html!` take to compile?)
- Memory usage at scale (1000+ components)
- Concurrent request handling (how many concurrent renders before contention?)
- WASM comparison (how does Idiomorph DOM patching compare to WASM?)

**Estimated effort:** 1 session

---

## Phase 4: Ecosystem Growth (Ongoing)

These are "nice to have" but not critical path.

### P4.1 — Template Hot Reload for Page-Level

**Problem:** Current hot reload works for component styles but not for full template content. Edit a component → full page reload.

**What:** Extend the hot-reload WebSocket to push full template patches (not just CSS). Use Idiomorph server-side to compute the diff.

**Estimated effort:** 1-2 sessions

---

### P4.2 — Form Validation Helpers

**Problem:** Form validation requires manual error handling in action handlers.

**What:**
- `validated!` macro or helper that wraps field validation
- Auto-generates error messages and `aria-invalid` attributes
- Integrates with `error_fragment()` for consistent error display

**Estimated effort:** 1 session

---

### P4.3 — WebSocket / SSE Support

**Problem:** Live data (notifications, real-time updates) requires external WebSocket setup.

**What:** An `az-sse` attribute or action that opens a Server-Sent Events connection. The server pushes HTML fragments, Idiomorph patches them in.

**Estimated effort:** 2 sessions

---

## Dracon Platform — Tactical Follow-ups

These are platform-specific items identified during this session's work.

### DP1. — CSS Consolidation: Remaining Flex-Column Patterns

**Status:** 18 patterns remain. All are **legitimate** non-candidates — they have background/border/padding or responsive overrides mixed in. No further action needed unless adding `vstack-0` (gap: 0) class.

**Candidate patterns that could be simplified (not urgent):**
| File | Line | Pattern | Notes |
|------|------|---------|-------|
| `render.rs` | 146 | `.page_content article` → `vstack-15` | Raw CSS string, not in html! macro. Would need Rust template changes. |
| `pages.rs` | 281 | `.home_pricing_header` → `vstack-075` | Has responsive override to row — not a pure vstack replacement. |
| `pages.rs` | 238, 557 | gap:0 patterns | Add `vstack-0` or drop no-op `gap: 0`. |

---

### DP2. — TypeScript Pipeline

**Status:** Set up and working. 2 files (ai-rankings, ai-hub-copy) + azumi.d.ts.

**Monitor:** Switch to `esbuild` if >5 TS files or >100KB output. Current `tsc` is fine.

---

### DP3. — Version Tracking

- **Azumi:** `v47.19.10` → `v47.20.20` ✅ (Axum 0.8 upgrade, client features)
- **Dracon Platform:** Updated Cargo.toml to use `v47.20.20` ✅

---

## Legend

| Icon | Meaning |
|------|---------|
| **P0** | Blocking — do this before anything else |
| **P1** | Core UX — makes the framework feel simple |
| **P2** | Features — expands capability |
| **P3** | Production — hardens for real use |
| **P4** | Nice-to-have — not critical path |
| **DP** | Dracon Platform specific |

---

## Quick Reference: What Each Phase Unlocks

| Phase | User Benefit | Business Impact |
|-------|-------------|-----------------|
| **P0** | "I can try Azumi in 5 minutes" | First-time adoption |
| **P1** | "This feels simple and clean" | Developer retention |
| **P2** | "I can build a real app with this" | Production use cases |
| **P3** | "I'd bet my company on this" | Enterprise adoption |
| **P4** | "This competes with Next.js" | Market leadership |

---

*Last updated: 2026-05-12*
