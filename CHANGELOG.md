# Changelog

All notable changes to Azumi will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [34.5.4] - 2026-05-01

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
