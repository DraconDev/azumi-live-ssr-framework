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

- [ ] **Investigate `src/client.min.js` line count anomaly** — `client/azumi.js` = 1,281 lines, `src/client.min.js` = 1,497 lines. Minified file has MORE lines than source. Either minification re-expands something, or the files diverged.
  - *Why:* If they diverged, the "minified" production runtime isn't actually the source. Could mean different behavior.
  - *Cost:* 10 min. `diff client/azumi.js src/client.min.js` and check.

---

## P0 — SHIP-BLOCKING (Do First)

### P0.1: `bind:value` — Two-Way Form Binding

- [ ] **`bind:value` client-side logic**
  - On `input` event for elements with `bind:value`, read `input.value`
  - Sync into nearest `az-scope` or `az-ui` state via `readState()` priority chain
  - Call `updateBindings()` so `az-bind:text` and `az-bind:class` reflect new value
  - Add 200ms debounce (configurable: `bind:value debounce=100`)
  - Support `<input>`, `<textarea>`, `<select>`
  - Support `type="checkbox"` as `bind:checked` (boolean toggle on state field)
  - Support `type="radio"` as `bind:value` (string set on state field)
  - *Why:* The #1 missing feature. Without this, `az-scope`, `az-bind:text`, `data-predict`, and the entire reactive pipeline have no visible entry point for form inputs. Every Svelte comparison cites this gap. For AI: collapses "write `<input>` + remember `on:input` + remember debounce + remember target" into one keyword that can't be wrong.
  - *Cost:* ~70 lines client-side (~2KB). Server: zero changes (just renders `bind:value` attribute).
  - *Files:* `client/azumi.js` (new `syncBinding()` method), `macros/src/token_parser.rs` (recognize `bind:value` attribute)

- [ ] **`bind:value` macro-level support**
  - Token parser must recognize `bind:value={state.field}` as valid attribute syntax
  - If field path doesn't exist on the state struct, emit compile error with fix suggestion
  - Generate `data-bind-value="field.path"` attribute on rendered `<input>`
  - *Cost:* ~20 lines in `token_parser.rs`, ~10 lines in `codegen.rs`
  - *Files:* `macros/src/token_parser.rs`, `macros/src/codegen.rs`

- [ ] **`bind:checked` for checkboxes**
  - Checkbox: sync `input.checked` (boolean) ↔ state field
  - Radio group: sync `input.value` ↔ state field when `input.checked` is true
  - *Cost:* ~20 lines client-side
  - *Files:* `client/azumi.js`

- [ ] **Test `bind:value`**
  - Text input: type "hello" → state.name updates → `az-bind:text` reflects "hello"
  - Debounce: rapid typing → single state update after 200ms
  - Checkbox: toggle → state.agree toggles
  - Radio: select option → state.option updates
  - Select/textarea: same as text input behavior
  - With `az-ui`: client-side state, no server roundtrip
  - With `az-scope`: server state, value sent on form submit
  - *Cost:* ~15 test cases
  - *Files:* `tests/bind_value_tests.rs` (new)

### P0.2: `@keyed` — Keyed List Updates

- [ ] **`@keyed` client-side logic**
  - When Idiomorph morphs a list, read `data-key` attribute on each list item
  - Match old DOM items to new HTML items by key
  - Preserve DOM state (scroll position, focus, CSS transitions) for matched items
  - Only morph items whose content actually changed (compare child HTML)
  - Handle adds, removes, and reorders correctly
  - *Why:* Without keys, every `@for` list change triggers full DOM replacement — scroll jumps to top, active input loses focus, CSS transitions restart. Svelte's `{#each (id)}` equivalent. For AI: prevents silent runtime breakage where AI-generated code compiles but list behavior is janky.
  - *Cost:* ~100 lines client-side (~3KB). Server: renders `data-key` on each `@for` item.
  - *Files:* `client/azumi.js` (integration into `callAction()` morph step)

- [ ] **`@keyed` macro-level support**
  - Token parser must recognize `@for item in items @keyed(item.id) { ... }` syntax
  - If no key expression, emit warning (not error — sometimes you want full morph)
  - Generate `data-key="{item.id}"` on the first element inside each `@for` iteration
  - *Cost:* ~40 lines in `token_parser.rs`, ~20 lines in `codegen.rs`
  - *Files:* `macros/src/token_parser.rs`, `macros/src/codegen.rs`

- [ ] **Test `@keyed`**
  - Add item to start of list → only new item created, existing items preserved
  - Remove item from middle → only that item removed, others unchanged
  - Reorder items → DOM moved, not recreated
  - Change item content → only that item morphed
  - No key → full morph (backward compatible)
  - With transitions: moved items animate to new position (stretch goal)
  - *Cost:* ~12 test cases
  - *Files:* `tests/keyed_tests.rs` (new)

### P0.3: Shrink Predicate DSL

- [ ] **Kill ternary support (~154 lines)**
  - Remove `parseTernary()` function (64 lines)
  - Remove `findTernaryIndex()` function (46 lines)
  - Remove `findOperatorIndex()` function (44 lines)
  - Remove all ternary branches from `evaluatePredicate()` and `evaluateExpression()`
  - *Why:* Nobody writes `"active && !disabled ? 'on' : 'off'"` in an HTML attribute. It's an AI hallucination factory — wrong quotes, wrong precedence, wrong nesting. Remove the capability entirely.
  - *Cost:* Deletion only. No replacement needed.
  - *Files:* `client/azumi.js`

- [ ] **Kill numeric comparisons (~10 lines)**
  - Remove `<`, `>`, `<=`, `>=` branches from `evaluatePredicate()`
  - *Why:* If you need `count > 5` to decide a CSS class, compute it in Rust and pass a boolean field. Don't embed arithmetic in HTML attributes.
  - *Files:* `client/azumi.js`

- [ ] **Simplify `evaluateExpression()` (~95 → ~20 lines)**
  - Keep only: field lookup (including nested paths `user.name`), string literals (`'hello'`, `"hello"`), number literals (`123`, `3.14`), boolean literals (`true`, `false`, `null`)
  - Remove: ternary support, `||` default operator, `+ N` increment (use prediction DSL instead), `- N` decrement (use prediction DSL instead), string escaping
  - *Files:* `client/azumi.js`

- [ ] **Simplify `evaluatePredicate()` (~79 → ~40 lines)**
  - Keep only: `!field` (negation), `field == 'val'`, `field != 'val'`, `field` (truthy check)
  - Remove: `<`, `>`, `<=`, `>=`, `&&`, `||`, ternary
  - *Why:* `az-bind:class:active="!disabled"` or `az-bind:class:selected="tab == 'rust'"` covers 95% of real use cases. Anything more complex belongs in Rust code.
  - *Files:* `client/azumi.js`

- [ ] **Remove hot reload from production bundle (~85 lines)**
  - Feature-gate `connectHotReload()`, `pollForReload()`, `handleStyleUpdate()` behind Azumi dev mode check
  - Already done in source (`client/azumi.js`) — just needs propagation to production copy
  - Remove `data-azumi-scope` selector from hot reload (keep for CSS updates)
  - *Files:* `client/azumi.js`

- [ ] **Verify bundle size reduction**
  - Before: ~42KB uncompressed, ~10KB gzipped
  - Target after P0.3: ~28KB uncompressed, ~7KB gzipped
  - Run `gzip -c src/client.min.js | wc -c` before and after
  - *Cost:* 5 min verification

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

| Milestone | Features Added | Features Removed | Uncompressed | Gzipped |
|-----------|----------------|-----------------|-------------|---------|
| **Today** | — | — | 42KB | 10KB |
| **After P0** | `bind:value` (+2KB), `@keyed` (+3KB) | Hot reload (-6KB), ternary DSL (-8KB), numeric ops (-1KB) | **~28KB** | **~7KB** |
| **After P1** | Transitions (+3KB), loading/error (+1KB), debounce (+1KB), `#[react]` (+3KB), lifecycle (+2KB), confirms (+1.5KB) | — | **~38KB** | **~9KB** |
| **After P2** | Optimistic lists (+3KB), intersect (+1KB), undo anim (+1KB), transition presets (+1KB) | — | **~44KB** | **~10KB** |

**Net at P1: 50% more features, same bundle size as today.** Premium interactive experience at no byte cost increase.

---

*Tasklist generated 2026-05-26. All estimates verified against current source. Priority reflects strategic analysis above.*
