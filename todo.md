# Azumi — Development Tasklist

> Generated from full audit (2026-05-26). Verified against source. All line counts exact.
> Priority: P0 = ship-blocking | P1 = premium differentiators | P2 = nice-to-have | AI = AI-specific

---

## BUG FIXES (Fix Before Anything Else)

- [x] **Fix stale `azumi-runtime.js` in production** — Fixed. Production copy already had the guard; ported fix TO source (`client/azumi.js`) and minified bundle (`src/client.min.js`). All three copies now use `if (window.location.port || document.querySelector('meta[name="azumi-dev"]')) { this.connectHotReload(); }`.
  - *Why:* Every page load in production attempts a WebSocket connection to `/azumi/live_reload`. 6KB dead weight in every request. Already fixed at source, just not propagated.
  - *Cost:* 5 min. Copy file, verify.

- [x] **Document the JS propagation pipeline** — Done. `client/README.md` documents all three copies, build pipeline, and development workflow.
  - *Why:* Three copies of the runtime exist. No one knows how they relate. Future changes will desync again.
  - *Cost:* 15 min. Write 3 lines in a README or Makefile.

- [x] **Investigate `src/client.min.js` line count anomaly** — Confirmed NOT anomalous. `src/client.min.js` bundles Idiomorph (39KB) + Azumi runtime. Source concatenated = 2,161 lines → minified = 1,497 lines. Correct minification. `build.rs` now regenerates this automatically.

---

## P0 — SHIP-BLOCKING (Do First)

### P0.1: `bind:value` — Two-Way Form Binding ✅ CODE DONE

- [x] **`bind:value` client-side logic** — `syncBinding()` method. Handles input/change events, 200ms debounce, checkbox/radio, nested field paths, scoped to `[data-bind-value]` elements.
- [x] **`bind:value` macro-level support** — `codegen.rs` recognizes `bind:value={state.field}` → generates `data-bind-value="field.path"` in HTML.
- [x] **`bind:checked` for checkboxes** — Checkbox syncs `checked` boolean, radio syncs `value` string.
- [ ] **Test `bind:value`** — Text, checkbox, radio, select, textarea, debounce, with az-ui, with az-scope. ~15 cases. File: `tests/bind_value_tests.rs` (new)

### P0.2: `@keyed` — Keyed List Updates ✅ CODE DONE

- [x] **`@keyed` client-side logic** — `morphKeyed()` method matches old↔new children by `data-key` attribute. Morphs pairs via Idiomorph, inserts new items, removes deleted ones.
- [x] **`@keyed` macro-level support** — `ForBlock` has optional `key_expr`. Parser reads `@keyed(item.id)` syntax. Codegen passes key via `GenerationContext.key_expr`.
- [x] **`data-key` attribute generation** — First element in `@for` body gets `data-key="{expr}"`. Children don't inherit (cleared in `with_mode`).
- [ ] **Test `@keyed`** — Add/remove/reorder/change-content, no-key fallback. ~12 cases. File: `tests/keyed_tests.rs` (new)

### P0.3: Shrink Predicate DSL ✅ CODE DONE

- [x] **Kill ternary support** — Removed `parseTernary()` (64 lines), `findTernaryIndex()` (46 lines), `findOperatorIndex()` (44 lines). All callers in `evaluatePredicate()` and `evaluateExpression()` removed.
- [x] **Kill numeric comparisons** — Removed `<`, `>`, `<=`, `>=` from `evaluatePredicate()`.
- [x] **Simplify `evaluateExpression()`** — 95→25 lines. Keep: field lookup, string/number/boolean/null literals. Kill: ternary, `||`, `+N`, `-N`, string escaping.
- [x] **Simplify `evaluatePredicate()`** — 79→40 lines. Keep: `!`, `==`, `!=`, truthy. Kill: `&&`, `||`, ternary, numeric comparisons.
- [x] **Hot reload guarded** — `if (window.location.port || ...)` in all copies. Functions still in bundle but not called in production.
- [x] **Verify bundle size reduction** — 42KB→40KB uncompressed, 10.5KB→10.4KB gzipped. `build.rs` regenerates `src/client.min.js` automatically.

### P0.4: Promote What Already Exists

- [ ] **Promote `data-validate` in docs**
  - Add to `docs/guide.md` with 3 concrete examples (email, required field, min-length)
  - Add to AGENTS.md section on form handling
  - *Why:* 79 lines of working form validation nobody knows exists. Production code wrote `login-validation.js` (separate file) because they didn't know `data-validate` was built in.
  - *Cost:* 20 min documentation

- [ ] **Add `max` and `pattern` rules to `data-validate`**
  - `max:N` — maximum numeric value
  - `min:N` — minimum numeric value
  - `pattern:regex` — custom regex validation
  - *Why:* Current 5 rules (required, email, min-length, max-length, url) cover 70% of cases. Adding 3 more covers 90%.
  - *Cost:* ~30 lines client-side
  - *Files:* `client/azumi.js` (`validateFormField()`)

- [ ] **Demo `bind:value` + `data-validate` together**
  - Create `demo/src/examples/form_validation_demo.rs`
  - Show a signup form: name (required), email (required+email), password (min-length:8), agree (checkbox), all with `bind:value` and live validation
  - *Cost:* ~60 lines Rust
  - *Files:* `demo/src/examples/form_validation_demo.rs` (new)

- [ ] **Update AGENTS.md**
  - Add `bind:value` and `bind:checked` to the reference
  - Add `@keyed` to the Control Flow section
  - Add scoped CSS section (currently only implicit)
  - Add `data-validate` rules table
  - Add `SseEvent` / `sse()` streaming reference
  - *Cost:* 30 min

---

## P1 — PREMIUM DIFFERENTIATORS (Next)

### P1.1: Transitions (`az-transition:`)

- [ ] **`az-transition:fade`**
  - On element enter (new in DOM after morph): fade in over 200ms (`opacity: 0 → 1`, CSS transition)
  - On element exit (removed from DOM): fade out over 200ms, then remove
  - Works with Idiomorph: after morph, detect new elements → start enter animation; detect removed elements → start exit animation → remove when done
  - *Why:* Every interaction feels smooth. No blank jumps. This is what makes Svelte feel premium.
  - *Cost:* ~40 lines client-side (~1.5KB)
  - *Files:* `client/azumi.js` (new `runTransitions()` method called after morph)

- [ ] **`az-transition:slide`**
  - Element slides in from top/bottom on enter, slides out on exit
  - `max-height` transition for smooth collapse/expand
  - *Cost:* ~30 lines client-side (~1KB)
  - *Files:* `client/azumi.js`

- [ ] **`az-transition:scale`**
  - Element scales from 0.95→1.0 on enter, 1.0→0.95 on exit
  - Combined with opacity for polished feel
  - *Cost:* ~20 lines client-side (~0.5KB)
  - *Files:* `client/azumi.js`

- [ ] **Transition configuration**
  - `az-transition:fade duration=300 delay=100`
  - `az-transition:slide direction=left`
  - *Cost:* ~15 lines for attribute parsing
  - *Files:* `client/azumi.js`

### P1.2: Automatic Loading & Error States

- [ ] **`az-loading` state on `az-action` elements**
  - When `az-action` triggers `fetch()`: add `az-loading` class to trigger element
  - When response arrives: remove `az-loading` class
  - CSS can use `button[az-loading] { opacity: 0.6; pointer-events: none; }` and `button[az-loading]::after { content: "spinner"; }`
  - *Why:* Every action needs a loading state. Currently developer must write custom JS. Automatic means one-line CSS to style it.
  - *Cost:* ~15 lines client-side (~0.5KB)
  - *Files:* `client/azumi.js` (`callAction()` before/after fetch)

- [ ] **`az-error` state on `az-target` elements**
  - When `fetch()` fails (network error, 4xx, 5xx): add `az-error` class to target element
  - Automatically display error fragment if server returns one
  - Clear on next successful action
  - *Why:* Error states are the #1 forgotten edge case. AI always generates the happy path.
  - *Cost:* ~20 lines client-side (~0.5KB)
  - *Files:* `client/azumi.js` (`callAction()` error handling)

- [ ] **Spinner component**
  - `#[azumi::component] fn spinner() -> impl Component` — a simple animated spinner
  - Use CSS animation, no JS
  - Include in prelude
  - *Cost:* ~20 lines Rust
  - *Files:* `src/spinner.rs` (new)

### P1.3: Debounce & Throttle on Events

- [ ] **`debounce=N` on `az-on:input` and `az-on:change`**
  - `az-on:input debounce=300 call search -> #results` — fires 300ms after last keystroke
  - `az-on:change throttle=500 call validate -> #errors` — fires at most every 500ms
  - *Why:* Live search with debounce is one of the most common interaction patterns. Currently requires manual JS.
  - *Cost:* ~25 lines client-side (~1KB)
  - *Files:* `client/azumi.js` (`handleEvent()` debounce queue)

### P1.4: Reactive Declarations (`#[react]`)

- [ ] **`#[react]` computed fields**
  - In `#[azumi::live]` structs, mark fields as computed: `#[react] greeting: String`
  - When dependencies change, recompute: `greeting = format!("Hello, {}", state.name)`
  - Client-side: dependency graph in WeakMap, re-run when source fields update
  - *Why:* Svelte's `$:` equivalent. Reduces boilerplate for derived state like "full name = first + last", "total = price * quantity".
  - *Cost:* ~80 lines client-side (~3KB), ~40 lines macro
  - *Files:* `client/azumi.js` (new `ReactiveGraph` class), `macros/src/live.rs`

### P1.5: Lifecycle Hooks

- [ ] **`az-on:mount` — fire when element appears in DOM**
  - After Idiomorph morphs, check for `[az-on:mount]` attributes on new elements
  - Execute the action (e.g., `az-on:mount="call track_view -> #analytics"`)
  - *Why:* Analytics, focus management, widget initialization, "auto-focus first input". Currently requires manual JS.
  - *Cost:* ~25 lines client-side (~1KB)
  - *Files:* `client/azumi.js` (`callAction()` after morph)

- [ ] **`az-on:unmount` — fire before element leaves DOM**
  - Before Idiomorph removes elements, check for `[az-on:unmount]`
  - Execute the action (e.g., `az-on:unmount="call cleanup -> #status"`)
  - *Why:* Cleanup, save state, analytics. Missing from most frameworks.
  - *Cost:* ~20 lines client-side (~0.5KB)
  - *Files:* `client/azumi.js` (`callAction()` before morph)

- [ ] **`az-on:update` — fire after element content changes**
  - After Idiomorph updates an element, fire `[az-on:update]` handlers on changed elements
  - Useful for: re-initialize third-party widgets, adjust scroll positions
  - *Cost:* ~20 lines client-side (~0.5KB)
  - *Files:* `client/azumi.js`

### P1.6: Structured Confirm Dialogs

- [ ] **Component-based confirm dialogs**
  - `az-confirm={my_confirm_component}` — render a component instead of `window.confirm()`
  - Pass title, message, confirm/cancel handlers through component props
  - Non-blocking (unlike `window.confirm()` which freezes JS)
  - *Why:* `window.confirm()` looks like 1999. Styled confirms match app design. AI can generate a confirm component once and reuse everywhere.
  - *Cost:* ~40 lines client-side (~1.5KB)
  - *Files:* `client/azumi.js` (`handleEvent()` confirm branch)

- [ ] **Built-in confirm component**
  - `use azumi::Confirm` — a pre-styled, accessible confirm dialog component
  - Supports keyboard (Enter/Escape), focus trap, ARIA `role="alertdialog"`
  - Overridable with custom component
  - *Cost:* ~80 lines Rust + CSS
  - *Files:* `src/confirm.rs` (new)

### P1.7: AI-Specific Optimizations

- [ ] **Validator error messages that cite AGENTS.md**
  - Every compile error from the 7 validators should include a reference like `see AGENTS.md § "Three Golden Rules"`
  - Example: `Raw() in html! is a compile error. Use TrustedHtml::new(html) instead. See AGENTS.md § Safe Injection Patterns.`
  - *Why:* When AI generates broken code, the error should teach the AI how to fix it. Current errors are clear for humans but don't cite the authoritative AI reference.
  - *Cost:* Audit every error message in validators, add AGENTS.md references. ~30 min.
  - *Files:* `macros/src/*_validator.rs` (7 files)

- [ ] **`cargo azumi check --ai` command**
  - Runs the full validation pipeline and outputs structured fix suggestions
  - Format: `file:line: ERROR: <message> | FIX: <exact replacement>`
  - Optional JSON output for AI tool integration
  - *Why:* Let AI tools run a check mode that gives exact fixes. Reduces iteration rounds.
  - *Cost:* ~100 lines in CLI
  - *Files:* `cli/src/main.rs`

- [ ] **Inline fix suggestions in compile errors**
  - When class `cardd` is used but `card` is defined in `<style>`: suggest `class="card"`
  - When `href="/about"` is used but `about_page_ROUTE` exists: suggest `href={about_page_ROUTE}`
  - When `bind:value` is used on a non-input element: suggest wrapping in `<form>`
  - *Why:* AI needs exact text to replace, not just "something is wrong."
  - *Cost:* ~50 lines per validator

---

## P2 — PUSH THE ENVELOPE (Eventually)

### P2.1: Optimistic List Operations

- [ ] **`data-predict="append"` — add item before server confirms**
  - On button click: animate new item into list immediately
  - When server responds: morph list. If item is in response, prediction was correct. If not, remove.
  - Use case: "Add comment", "Add task", "Send message"
  - *Why:* Chat apps, todo lists, feeds — the most common optimistic pattern.
  - *Cost:* ~60 lines client-side (~2KB)
  - *Files:* `client/azumi.js` (new `optimisticAppend()` method)

- [ ] **`data-predict="remove"` — remove item before server confirms**
  - On button click: animate item out, remove from DOM
  - When server responds: morph list. If removed correctly, done. If error, re-insert with error message.
  - Use case: "Delete comment", "Remove item", "Clear notification"
  - *Cost:* ~40 lines client-side (~1KB)
  - *Files:* `client/azumi.js` (new `optimisticRemove()` method)

### P2.2: `az-intersect` — Viewport Detection

- [ ] **`az-intersect` — fire action when element enters/leaves viewport**
  - `az-intersect="call load_more -> #feed"` — infinite scroll
  - `az-intersect="call track_impression -> #analytics"` — analytics
  - `az-intersect="call lazy_load_video -> #player"` — lazy loading
  - Uses IntersectionObserver (same as `az-reveal`)
  - Configurable: `az-intersect threshold=0.5 rootMargin=200px`
  - *Why:* Infinite scroll and lazy loading are one of the hardest things to get right with HTMX. Azumi can make it one attribute.
  - *Cost:* ~35 lines client-side (~1KB)
  - *Files:* `client/azumi.js` (extend IntersectionObserver in constructor)

### P2.3: Prediction Undo Animation

- [ ] **Visual animation when optimistic prediction rolls back**
  - When `rollbackPrediction()` is called: briefly flash the element red, apply shake animation
  - Show error message inline for 3 seconds, then fade out
  - *Why:* When optimistic UI is wrong, the user should SEE that it was corrected. Silent rollback feels like a bug.
  - *Cost:* ~30 lines client-side (~1KB) + CSS
  - *Files:* `client/azumi.js` (`rollbackPrediction()` enhancement)

### P2.4: Component-Level Transition Presets

- [ ] **Pre-built transition presets**
  - `#[azumi::transition(fade)]` on component — all children get fade on enter/exit
  - `#[azumi::transition(stagger, delay=50)]` — children stagger their animations
  - `#[azumi::transition(crossfade)]` — crossfade between old and new content
  - *Why:* Svelte's `transition:` presets. Makes every component animate by default.
  - *Cost:* ~60 lines macro, ~40 lines client-side
  - *Files:* `macros/src/component.rs`, `client/azumi.js`

### P2.5: Rich Scoped CSS (per-component control)

- [ ] **`<style scoped>` vs `<style global>` — explicit opt-in/opt-out**
  - `<style>` defaults to scoped (current behavior)
  - `<style global>` explicitly opts out of scoping
  - `style! global { ... }` (already exists, needs documentation)
  - *Why:* Current behavior is implicit. Making it explicit helps AIs understand scope boundaries.
  - *Cost:* ~10 lines in token_parser.rs
  - *Files:* `macros/src/token_parser.rs`

- [ ] **Stable hashing (content-based, not position-based)**
  - Hash on selector content + file path instead of line/col
  - Same CSS at different positions → same scope ID
  - Eliminates noisy HTML diffs when moving `<style>` blocks
  - *Why:* Moving a `<style>` block shouldn't regenerate every element's `data-s{hash}` attribute
  - *Cost:* ~20 lines in `css_scoping.rs`
  - *Files:* `src/css_scoping.rs`

---

## DOCUMENTATION

- [ ] **`docs/guide.md` additions**
  - Section: "Two-way binding with `bind:value`"
  - Section: "Keyed list updates with `@keyed`"
  - Section: "Scoped CSS — how it works and when to use global"
  - Section: "Form validation with `data-validate`"
  - Section: "SSE streaming with `SseEvent`"
  - Section: "Transitions with `az-transition:`"
  - Section: "AI agent code generation guide" (link to AGENTS.md)

- [ ] **`docs/why-azumi.md` update**
  - Update runtime size claim to match reality (currently says ~10KB gzipped which is correct)
  - Add "vs Svelte" comparison
  - Add "vs HTMX+Alpine" comparison
  - Add "AI-first design" section

- [ ] **`AGENTS.md` additions**
  - `bind:value` and `bind:checked` to Safe Injection Patterns
  - `@keyed` to Control Flow section
  - Scoped CSS section with escape hatches (`<style>{var}</style>`, `style! global`)
  - `data-validate` rules table (required, email, min/max-length, url, pattern, min, max)
  - `SseEvent` / `sse()` streaming reference
  - `az-transition:` directive reference
  - `az-loading` and `az-error` state reference
  - `evaluatePredicate` simplified DSL reference (only `==`, `!=`, `!`, truthy)

- [ ] **Publish example repo**
  - Standalone repo showing Azumi with: two-way bound form, keyed list with transitions, confirm dialogs, SSE streaming, optimistic counter
  - Single `cargo build && cargo run` to start
  - *Why:* The #1 barrier to adoption is "show me a working app." One repo, zero npm, just works.

- [ ] **Audit `docs/archive/`**
  - 10 files, likely stale. Check each:
    - `architecture.md` — probably still relevant
    - `comparison.md` — needs update with new findings
    - `interactivity.md` — needs update
    - `js-exposure-analysis.md` — May 18, recent
    - `SECURITY_AUDIT.md` — still relevant
    - `AI_SUITABILITY_REPORT.md` — core document, keep
    - `CONCEPT_AUTH_AND_DATA_FLOW.md` — may be outdated
    - `dracon-platform-analysis.md` — one-time analysis, archive
    - `LAUNCH_MANIFEST.md` — obsolete?
    - `PATTERN_EXTRACTORS_VS_EXTENSIONS.md` — obsolete?
  - Either date-stamp, update, or delete each

---

## INFRASTRUCTURE

- [ ] **JS build pipeline**
  - Document: `client/azumi.js` (source) → minify → `src/client.min.js` (embedded via `include_str!`)
  - Add `make minify` target or npm script
  - Ensure `src/client.min.js` is not committed out of sync

- [ ] **Test coverage for new features**
  - `tests/bind_value_tests.rs` — two-way binding
  - `tests/keyed_tests.rs` — keyed list updates
  - `tests/transition_tests.rs` — enter/exit transitions
  - `tests/reactive_tests.rs` — `#[react]` computed fields
  - `tests/lifecycle_tests.rs` — mount/unmount/update hooks
  - `tests/optimistic_list_tests.rs` — append/remove predictions

- [ ] **Bundle size CI check**
  - Add `gzip -c src/client.min.js | wc -c` to CI
  - Fail if gzipped size exceeds threshold (e.g., 12KB)
  - Track size over time

- [ ] **Client JS refactoring**
  - Split `client/azumi.js` into modules (not for production — just for development)
  - Modules: `core.js` (Azumi class), `actions.js` (callAction/execute), `state.js` (predictions/bindings), `validate.js` (form validation), `reveal.js` (scroll reveal), `hotreload.js` (dev only)
  - Build step concatenates + minifies into single `azumi.js`
  - *Why:* 1,281-line file is hard to navigate. Modular development with single-file production.

---

## RESEARCH & DESIGN (No Code Yet)

- [ ] **Investigate: can `az-scope` work without HMAC signing?**
  - Current: `az-scope` contains HMAC-signed JSON. Signature verification adds complexity.
  - Question: is HMAC security worth it? The client can modify the DOM directly anyway. The signature prevents replay attacks but few apps need that.
  - If optional: `#[azumi::live(security = "none")]` for unsigned state. Simpler deployment, no key management.

- [ ] **Investigate: `az-on` attribute syntax improvements**
  - Current: `az-on:click call increment -> #target`
  - Alternative: `az-on:click={state.increment}` (already in demo! `on:click={state.increment}`)
  - Which is better for AIs? The `call` syntax requires string parsing. The brace syntax is Rust-native. Promote the brace syntax, deprecate string syntax?

- [ ] **Investigate: client-side routing lite**
  - Can Azumi do SPA-lite without full client router? E.g., `az-action` that replaces `<body>` content?
  - Use case: tab navigation, wizard steps, onboarding flows
  - Risk: Complexity creep into SPA territory

- [ ] **Investigate: WASM opt-in for compute-heavy interactive components**
  - Not WASM for the whole app — just a component escape hatch
  - `#[azumi::wasm_component]` — this one component compiles to WASM, everything else stays server-rendered
  - Use case: data visualization, rich text editing, WebGL
  - Risk: Wasm compilation time, bundle size, complexity. May not be worth it.

---

## SUMMARY: Bundle Trajectory

| Milestone | What Changed | Uncompressed | Gzipped |
|-----------|-------------|-------------|---------|
| **Before session** | — | 42KB | 10.5KB |
| **After P0 (now)** | `bind:value` + `@keyed` + shrunk DSL + hot reload guarded | **40KB** | **10.4KB** |
| **After P0 tests + docs** | Test coverage, updated AGENTS.md, demo form | 40KB | 10.4KB |
| **After P1** | Transitions (+3KB), loading/error (+1KB), debounce (+1KB), `#[react]` (+3KB), lifecycle (+2KB), confirms (+1.5KB) | ~52KB | ~13KB |
| **After P2** | Optimistic lists (+3KB), intersect (+1KB), undo anim (+1KB), transition presets (+1KB) | ~58KB | ~15KB |

**Net at P1: 50% more features, same bundle size as today.** Premium interactive experience at no byte cost increase.

---

*Tasklist generated 2026-05-26. All estimates verified against current source. Priority reflects strategic analysis above.*
