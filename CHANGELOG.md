# Changelog

All notable changes to Azumi will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [47.44.0] - 2026-05-18

### Added
- **Borrow-friendly component props**: `&str` and `&T` parameters in `#[azumi::component]` now work without explicit lifetime annotations. The macro auto-injects a synthetic lifetime into Props, PropsBuilder, and render().
- **`TrustedHtml` promoted to public API**: `TrustedHtml::new()` and `TrustedHtml::from_string()` are now documented, in the prelude, and the safe replacement for `Raw()`.
- **`#[live_state]` explicit attribute**: Components can use `#[live_state]` on any parameter instead of relying on the `state` naming convention.
- **`#[azumi::page(route)]` route constants**: Auto-generates `<name>_ROUTE` constant for compile-time link safety.
- **`#[azumi::action]` path constants**: Auto-generates `<name>_PATH` constant.
- **aria-* value validation**: 15+ ARIA attributes now validate their values at compile time (e.g., `aria-expanded` only accepts `true`/`false`).
- **Vendored JS integrity checks**: FNV-1a checksums for `idiomorph.js` and `azumi.js` in `build.rs` — build fails if files are tampered.
- **TypeScript definitions**: `client/azumi.d.ts` with full API type definitions.
- **`docs/migration/from-axum.md`**: 6-step incremental migration guide from plain Axum.

### Security
- **Timing attack fix**: `is_dev_token_valid` in `hot_reload.rs` uses constant-time comparison without length leak.
- **`AZUMI_DEV_TOKEN` cached**: OnceLock avoids env read per request.
- **`escape_tag_content` no-panic**: Returns content unescaped instead of panicking on empty `tag_name`.
- **`init_seo` returns `SeoError`**: Proper error type with `Error + Display` impl, replaces `Box<SeoConfig>`.
- **Devtools headers**: Use `HeaderValue::from_static` (compile-time checked).
- **Configurable `MAX_STATE_AGE_SECS`**: `AZUMI_STATE_MAX_AGE` env var.
- **Hot reload production guard**: `devtools::router()` panics in release unless `AZUMI_ALLOW_DEVTOOLS_IN_RELEASE` is set.

### Changed
- **Live state graceful degradation**: `to_scope()` returns signed empty state `{}` instead of panicking on serde failure.
- **Demo positioning**: Homepage updated to "Server-Rendered HTML with Client Interactivity" / "All Rust. Zero custom JavaScript. No ecosystem churn."
- **Demo blog actions**: `format!` building HTML replaced with `html!` + `render_to_string()`.
- **Benchmark data**: Runtime size comparison added (Azumi 10KB gzipped vs HTMX 15KB vs React 46KB).
- **Doc consolidation**: 12 docs → 3 (README, why-azumi, guide) + archive.
- **Zero clippy warnings** workspace-wide.

## [48.0.0] - 2026-05-12

### Stability Promise
Azumi now follows strict [Semantic Versioning](https://semver.org/). From v48 forward:
- Major = actual breaking changes (at most every 3 months)
- Minor = new features, backward compatible (monthly)
- Patch = bug fixes only (as needed)

### Changed
- **Axum upgrade**: `axum` dependency `0.7` → `0.8`. If you use `Message::Text(s)`, call `s.into()` for `Utf8Bytes`. `axum::async_trait` is no longer needed (use native `async fn in trait` with Rust 1.75+).
- **Axum-extra**: Companion upgrade `0.9` → `0.10`.
- **`#[azumi::live]` now handles impl blocks**: Use `#[azumi::live]` directly on impl blocks for action handlers. `#[azumi::live_impl]` still works but is deprecated.
- **Prelude cleanup**: `head`, `live_impl`, `page`, `predict` removed from `azumi::prelude`. Still accessible via `azumi::head`, etc.

### Added
- **`azumi new` scaffolding CLI**: `cargo install azumi-cli && azumi new my-app` creates a working project.
- **`azumi::routes!` macro**: Declare routes inline without manual Axum Router setup.

## [47.0.0] - 2026-05-06

### Added
- **`#[must_use]` annotations**: 13 functions now warn when their return value is discarded (escape, sign, render, scope helpers).
- **`VerifyStateError` public error type**: Replaces `Result<String, String>` anti-pattern with a typed enum (`thiserror::Error`). Accessible as `azumi::VerifyStateError`.
- **`escape_tag_content()` generic function**: Single-pass scanner replaces 4× `.replace()` chain — O(n) performance for all content sizes.
- **Property-based tests**: 4 proptest properties validate escape functions against random inputs.
- **Criterion benchmarks**: 3 benchmark suites — `escape`, `render`, `scope_css` — establish baseline for performance tracking.
- **Architecture documentation**: `docs/architecture.md` covering crate structure, validation pipeline, and rendering flow.
- **Migration guides**: `docs/migration/v42.md` and `docs/migration/v43.md` document breaking changes.
- **CI improvements**: Benchmark smoke test job, `AZUMI_SECRET` for release tests, clippy runs on all targets/features.

### Changed
- **`Context`/`GenerationContext` extracted**: Moved from `macros/src/lib.rs` (1516 lines) to `macros/src/context.rs` (59 lines).
- **`FnOnceComponent` internals**: Replaced `UnsafeCell` with `RefCell` — eliminates unsafe code, same zero-overhead single-thread behavior.
- **Doc warnings**: 4 rustdoc warnings fixed (unclosed HTML tags, empty code blocks, unresolved links).
- **Clippy warnings**: 11 warnings fixed (unused variables, unused imports, manual `Default` impls replaced with `#[derive(Default)]`).

### Documentation
- `docs/architecture.md` — crate structure, validation pipeline, rendering flow
- `docs/migration/v42.md` — Raw/format!/TrustedHtml changes
- `docs/migration/v43.md` — inline_css!/inline_script! removal

### Internal
- Benchmarks configured with `harness = false` for Criterion
- Cargo.toml: added `thiserror`, `proptest`, `criterion` dependencies

## [46.0.0] - 2026-05-05

### Breaking Changes
- **`<style>{var}</style>` auto-escaping finalized**: The `html!` macro now properly detects `<style>{expr}</style>` syntax and auto-escapes expression children via `escape_style_content()`.
- **`parse_script_content` supports bare `{expr}`**: Script and style tags now accept bare `{expr}` expressions (not just `@{expr}`), making the syntax consistent with other html! elements.
- **Node order enforcement for injected CSS/JS**: `<style>` and `<script>` with expression children are now subject to Script → Content → Style ordering validation.

### Added
- **Auto-escaping tests**: 16 new tests covering all breakout variants (lowercase, uppercase, titlecase, with space, multiple occurrences, no double-escape).
- **Documentation fully updated**: AGENTS.md, AI_GUIDE.md, README.md all reflect the bare-tag injection patterns.

### Migration from v45.x
- No changes needed — v45.x already had the macro removal. This release finalizes the implementation.

## [45.0.0] - 2026-05-05

### Breaking Changes
- **inline_css! and inline_script! macros removed**: Replaced by built-in auto-escaping.

## [43.0.0] - 2026-05-05

### Breaking Changes
- **inline_css! and inline_script! macros removed**: These thin wrappers have been replaced by built-in auto-escaping in the `html!` macro. Use `<style>{var}</style>` and `<script>{var}</script>` instead — content is automatically escaped by the `html!` renderer.
- **Context::Style added**: The html! macro now tracks both `Context::Script` and `Context::Style` to auto-escape expression children.
- **`<style>` tag parsing updated**: `parse_style_tag` now detects `<style>{expr}</style>` and creates an Element node with expression children (auto-escaped) instead of a StyleBlock.
- **`parse_script_content` updated**: Now supports bare `{expr}` expressions (not just `@{expr}`) inside `<script>` and `<style>` tags.

### Added
- **Auto-escaping for `<script>{var}</script>`**: Expression children inside `<script>` tags are automatically passed through `escape_script_content()` by the html! renderer.
- **Auto-escaping for `<style>{var}</style>`**: Expression children inside `<style>` tags are automatically passed through `escape_style_content()` by the html! renderer.

### Migration from v42.x
- `{inline_css!(CSS_VAR)}` → `<style>{CSS_VAR}</style>`
- `{inline_script!(JS_VAR)}` → `<script>{JS_VAR}</script>`
- `json_data!("VAR" = &data)` stays unchanged (it does real serde + variable naming work)

## [42.0.0] - 2026-05-05

### Breaking Changes
- **Raw() unconditionally blocked**: ALL usages of `Raw()` inside `html!` are now compile errors. Previously, only patterns with JS/CSS/HTML content were blocked. This is a complete ban.
- **format! with web patterns blocked**: `format!("<div>{}</div>", x)` inside `html!` expressions is now a compile error. Build content outside `html!` and pass as a variable.
- **TrustedHtml removed from prelude**: `TrustedHtml` is `#[doc(hidden)]` and no longer exported in the prelude. It was only for internal framework use.

### Added
- **json_data!("VAR" = &data)**: Safe JSON data injection for JavaScript. Auto-serializes with serde_json, escapes `</script>` (case-insensitive: `</script>`, `</Script>`, `</SCRIPT>`, `</ script>`).
- **inline_css!(CSS_VAR)**: Safe CSS injection. Escapes `</style>` (case-insensitive).
- **inline_script!(JS_VAR)**: Safe JavaScript injection. Escapes `</script>` (case-insensitive).
- **escape_style_content()**: New public function for CSS string escaping.
- **AZUMI_RULES**: Framework rules array for AI verification.
- **AI_GUIDE_FOR_WRITING_AZUMI.md section reordering**: Safe injection macros now prominently featured as "Three Golden Rules" at the top.

### Changed
- **escape_script_content() now escapes ALL occurrences**: Previously would stop at first match. Now properly handles multiple `</script>` tags in one payload.
- **escape_style_content() escapes ALL occurrences**: Same fix as above for CSS.
- **Documentation updated**: README.md, AI_GUIDE.md, and AGENTS.md all updated to reflect the AI-first philosophy and safe injection macros.

### Migration from v41.x
- `Raw()` anywhere in `html!` → Use `json_data!`, `inline_css!`, or `inline_script!` macros
- `format!()` building HTML/CSS/JS inside `html!` → Build content outside `html!` and pass as variable
- `TrustedHtml::new()` → Should not be needed in application code (internal use only)

### Migration from v26.x → v28
- `#[azumi::live]` + `#[azumi::live_impl]` now required together for predictions
- Predictions auto-detected from `#[azumi::live_impl]` — manual `data-predict` is optional
- See [MIGRATION.md](MIGRATION.md) for full v26 → v27+ upgrade guide

---

### Fixed
- **`property` attribute added to HTML validator**: OpenGraph meta tags (`<meta property="og:*">`) now compile correctly in `html!` macros — `property` was missing from the allowed attribute whitelist
- **`test_generate_head_with_type` initialization**: Test now properly calls `reset_seo()` + `init_seo()` before `generate_head()` to avoid test pollution from global `SITE_CONFIG`
- **SEO tests use correct HTML**: `tests/seo_tests.rs` OG tests now use `property` instead of `data-property` workaround

## [34.3.0] - 2026-04-30

### Added
- **`||` operator in `evaluateExpression`**: Field fallback/default value support — `field || 'default'` returns the field value unless it's `null`, `undefined`, or `''` (empty string), in which case it evaluates and returns the default expression
- **Parenthetical grouping in evaluators**: Both `evaluatePredicate` and `evaluateExpression` now strip outer `()` and recurse — `(a && b)` no longer treated as a literal field name
- **Depth-tracked ternary parser**: `parseTernary()` and `findTernaryIndex()` use depth tracking for `()`/`[]`/`{}` brackets instead of a simple regex — correctly handles nested ternaries like `a ? b ? c : d : e`
- **`colonBalance` for nested ternary on truthy side**: `parseTernary` tracks how many `?`/`:` pairs are open on the truthy branch, ensuring the correct `:` is matched for the outer ternary

### Fixed
- **Float comparisons in `evaluatePredicate`**: All 4 comparison operators (`<`, `>`, `<=`, `>=`) now use `[\d.]+` regex + `parseFloat` instead of `\d+` + `parseInt`
- **Float arithmetic in `evaluateExpression`**: Increment (`+`) and decrement (`-`) operators now use `parseFloat` instead of `parseInt`
- **Strict numeric literal regex**: Changed `[\d.]+` → `\d+(?:\.\d+)?` to prevent malformed floats like `1.5.2` from being silently truncated to `1.5`
- **`findOperatorIndex` escape handling**: Replaced fragile `expr[i-1] !== '\\'` check with proper `isEscaped` state-machine, consistent with `parseTernary` and `findTernaryIndex`
- **Dead code removed**: Three stale duplicate comparison blocks (integer-only `\d+`/parseInt) deleted from `evaluatePredicate`; unused `normalizedValue`, `parenDepth`, and `allAttrs` sentinel variable removed
- **String escape regex**: Fixed spurious `)` in character class `['")\\]` → `['"\\]`

### Changed
- **`applyPrediction` increment/decrement**: Now supports float operands and uses `parseFloat` — `score = count + 1.5` works correctly
- **Documentation**: `client/README.md` "Supported Expressions" updated; `AI_GUIDE_FOR_WRITING_AZUMI.md` `az-bind` syntax table updated; test descriptions clarified

---

## [30.3.1] - 2026-04-30

### Added
- **`az-ui` attribute**: Ephemeral client-side state container — unsigned, no server round-trip, lost on refresh, survives DOM morphing. Lives only in the DOM, no Rust state struct.
- **`az-bind:class`**: Expression-driven CSS class toggling (`az-bind:class:active="is_active"`, `az-bind:class.liked="liked"`)
- **`az-bind:text`**: Expression-driven text content updates (`az-bind:text="count"`, `az-bind:text="liked ? 'Unlike' : 'Like'"`)
- **`set` command restored** (client JS): `az-on="click set field = value"` mutates `az-ui` state — no HMAC, no round-trip, ephemeral
- **`evaluatePredicate()`**: Full predicate evaluator — field lookup, equality, inequality, numeric comparison, negation (`!`), AND/OR, ternary
- **`evaluateExpression()`**: Full expression evaluator — field lookup, string/number/boolean/null literals, arithmetic (`+`, `-`), ternary
- **`readState()`**: Priority resolution — WeakMap → `az-ui` → `az-scope`

### Changed
- **`az-ui` vs `az-scope`**: Clear separation — `az-scope` = server-signed HMAC state (round-trip), `az-ui` = client ephemeral state (no round-trip)

### Fixed
- **Documentation updated**: `client/README.md`, `AI_GUIDE_FOR_WRITING_AZUMI.md` now reflect full expression evaluator capabilities

### Migration
- For ephemeral UI state (toggles, counters, accordion), use `az-ui` + `set` — no server involvement
- For persistent server state, continue using `az-scope` + predictions + server actions

---

## [29.0.0] - 2026-04-30

### Added
- **Component macro refactor**: Centralized live-state scope wrapper generation in `macros/src/component.rs`
- Simplified asset pipeline: standard HTML `srcset` with automatic path rewriting (removed `asset!()` and `image!()` macro references)
- `window.azumi.execute()` JavaScript API for manual action execution

### Changed
- **Documentation sweep**: Updated README to v28.0.2, fixed features table (devtools=Optional), corrected live components diagram to show az-predictions auto-detection
- **client/README.md**: Complete rewrite — removed deprecated `set` command, documented az-predictions JSON auto-detection, updated server protocol to namespaced paths
- **AI_GUIDE**: Removed `set` command section, fixed section numbering (6→11), removed duplicate headings, updated predictions to auto-detection as primary
- **AZUMI_DESCRIPTION.md**: Opening example now shows auto-detection flow

### Fixed
- **Removed**: `az-on="click set ..."` syntax no longer documented (command was removed from client JS)
- **Removed**: `window.azumi.setState()` API documentation (method doesn't exist in client JS)
- **Removed**: Non-existent `image!()` and `asset!()` macro references from ASSET_PIPELINE_DESIGN.md

### Migration
`#[azumi::live]` + `#[azumi::live_impl]` required together. Manual `data-predict` now optional — predictions auto-detected from `#[azumi::live_impl]`.

---

## [27.0.0] - 2026-04-30

### Added
- **Auto-detected predictions**: `#[azumi::live_impl]` now analyzes method mutations and stores predictions in `LiveStateMetadata`. The component macro injects these as `az-predictions` JSON on the scope div. Client JS auto-executes predictions when buttons are clicked.
- `az-predictions` HTML attribute: Contains JSON array of `[method_name, prediction_dsl]` tuples
- `MIGRATION.md`: Guide for upgrading from v26 to v27
- Tests for predictions metadata: 8 new tests verifying `LiveStateMetadata::predictions()` returns real data
- Runtime auto-detection in client JS: Falls back from `data-predict` → `az-predictions` → no prediction

### Changed
- **Breaking**: `#[azumi::live]` no longer provides `LiveStateMetadata` or `LiveState` traits. These are now provided by `#[azumi::live_impl]`.
- **Breaking**: `#[azumi::live]` + `#[azumi::live_impl]` are now required together for predictions to work
- Manual `data-predict` attributes are now optional for simple mutations (auto-detected from `#[azumi::live_impl]`)
- Documentation updated to reflect auto-detection: AI_GUIDE, README, AZUMI_DESCRIPTION, AZUMI_LESSON_PLAN

### Fixed
- SEO test isolation: Replaced `OnceLock` with `Mutex<Option<SeoConfig>>` to prevent test pollution
- Added `reset_seo()` for test cleanup
- Compiler warnings in `macros/src/live.rs` (unused variables)

### Migration
See [MIGRATION.md](MIGRATION.md) for v26 → v27 upgrade instructions.

---

## [26.7.0] - 2026-04-29

### Added
- Comprehensive unit tests for validation modules
- Tests for `script.rs`, `asset_rewriter.rs`, `css_validator.rs`

### Changed
- Updated dependency versions in Cargo.lock

---

## [26.0.0] - 2026-04-20

### Added
- `#[azumi::predict("...")]` attribute for manual prediction DSL
- `data-predict` attribute support in HTML
- Client-side optimistic UI execution

### Changed
- `#[azumi::live]` macro now generates `LiveStateMetadata` trait
- `#[azumi::live_impl]` generates action handlers with namespaced paths

---

## [25.0.0] - 2026-04-10

### Added
- `#[azumi::live]` and `#[azumi::live_impl]` macros
- Compiler-driven optimistic UI analysis
- `az-scope` and `az-struct` attributes for live state

---

## [24.0.0] - 2026-03-15

### Added
- `#[azumi::action]` macro for server actions
- Inventory-based action registration
- HMAC-signed state validation

---

## [23.0.0] - 2026-02-28

### Added
- SEO module with `generate_head()`, sitemap builder
- OpenGraph and Twitter Card support
- XSS prevention in SEO metadata

---

## [22.0.0] - 2026-02-10

### Added
- `#[azumi::component]` macro with builder pattern
- Props generation with `#[prop(default = ...)]`
- Children support in components

---

## [21.0.0] - 2026-01-20

### Added
- CSS validation at compile time
- `class={...}` snake_case enforcement
- Inline style DSL with `--property: "value"`

---

## [20.0.0] - 2025-12-15

### Added
- `html!` macro with compile-time HTML validation
- Accessibility validator (img alt, input types, ARIA roles)
- HTML structure validator (nesting rules)

---

## [1.0.0] - 2025-11-01

### Added
- Initial release
- Type-safe HTML templating
- Server-side rendering
- Asset pipeline with content hashing
