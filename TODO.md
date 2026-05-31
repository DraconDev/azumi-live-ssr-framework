# Azumi Audit — May 31, 2026

## Status

| Check | Result |
|-------|--------|
| `cargo test` | ✅ 1,806 tests pass, 0 failures |
| `cargo build` | ✅ Zero warnings |
| `cargo clippy` | ✅ Clean |
| Source files | 34 `.rs` files (src + macros/src) |
| Test files | 50+ `.rs` test files |

## Fixed This Session

| File | Fix |
|------|-----|
| `macros/src/css_validator.rs` | Added `validate_style_tag_css` — enforces double-quoted CSS values in `<style>` tags |
| `macros/src/css_validator.rs` | Added `extract_declarations_for_validation` — parses CSS declarations from rule blocks |
| `macros/src/css_validator.rs` | Added `validate_css_values_in_text` — validates single-token CSS values are quoted |
| `macros/src/css_validator.rs` | Added 12 unit tests for CSS value validation |
| `macros/src/lib.rs` | Wired `validate_style_tag_css` into `html!` macro validation pipeline |
| `macros/src/style.rs` | Added `preserve_quotes` field to `StyleProperty` — preserves quotes for `content`, `font-family`, `string-set` |
| `macros/src/style.rs` | Added `CSS_QUOTE_REQUIRED_PROPERTIES` constant |
| `macros/src/style.rs` | Added `requote_property_values` — post-processes minified CSS to re-add quotes |
| `macros/src/style.rs` | Made `tokens_to_css_string` `pub(crate)` for cross-module access |
| `macros/src/style.rs` | Added 3 tests for content/font-family/regular-property quote handling |

## Build & Test Warnings

### Demo test crate warnings (non-blocking)
- `bind_value_tests`: 11 unused variable/import warnings (test-only, cosmetic)
- `keyed_tests`: 1 unused variable warning (test-only)
- `transition_tests`: 1 unused import warning (test-only)

These are in the `demo` crate's integration tests, not production code.

## Production Code Audit

### Dead Code
- `src/lib.rs:641` — `#[allow(dead_code)]` on `HotReloadClosure` (used by macro expansion, compiler can't see it) ✅ acceptable
- `macros/src/style.rs:42` — `#[allow(dead_code)]` on `span` field in `StyleProperty` ✅ acceptable
- `macros/src/token_parser.rs` — 16 `#[allow(dead_code)]` on AST struct fields ✅ acceptable (parser structs, fields used by different consumers)

### `#[doc(hidden)]` Public Items
- `src/lib.rs:69` — `AZUMI_RULES` constant ✅
- `src/lib.rs:100` — `AZUMI_AI_HASH` constant ✅
- `src/lib.rs:338` — `from_fn_once` function ⚠️ should be `pub(crate)` (see P3 below)
- `src/lib.rs:408` — `from_fn` function ⚠️ should be `pub(crate)` (see P3 below)
- `src/lib.rs:438` — `Raw<T>` struct ✅ internal only
- `src/lib.rs:636` — `TrustedHtml` struct ✅ public escape hatch (documented in AGENTS.md)
- `macros/src/lib.rs:26` — `head` proc macro ✅ internal
- `macros/src/lib.rs:32` — `page` proc macro ✅ internal
- `macros/src/lib.rs:61` — `live_impl` proc macro ✅ backward compat
- `macros/src/lib.rs:66` — `predict` proc macro ✅ no-op marker

### `eprintln!` in Production Code (non-test)
- `src/security.rs:34,42,50,71,92,96,107` — Secret management warnings ✅ intentional (dev-only paths via `#[cfg(debug_assertions)]`)
- `src/csp.rs:429` — CSP header invalid character warning ✅ intentional
- `macros/src/component.rs:269` — Dashed CSS class warning ✅ intentional
- `macros/src/live.rs:569` — Serialization failure warning ✅ intentional
- `macros/src/schema.rs:94` — Schema field serialization warning ✅ intentional
- `macros/src/style.rs:513` — Dashed CSS class binding warning ✅ intentional
- `macros/src/token_parser.rs:972-1041` — Debug mode only (`AZUMI_DEBUG` env var) ✅ intentional

### `unwrap()` Calls in Production Code (non-test)
- `macros/src/component.rs:302` — `children_type.as_ref().unwrap()` ⚠️ safe because `has_children` guard ensures it's `Some`
- `macros/src/schema.rs:45` — `field.ident.as_ref().unwrap()` ✅ safe on named struct fields
- `src/hot_reload.rs:115` — `NonZeroUsize::new(MAX_REGISTRY_SIZE).unwrap()` ✅ constant is always > 0

### Stale References
- `src/security.rs:12` — `DEFAULT_SECRET` contains "azumi-dev-secret-do-not-use-in-prod" ✅ intentional (dev-only fallback)
- `src/lib.rs:12` — Doc comment references `azumi-live-ssr-framework` ⚠️ old package name (see P6 below)
- `src/devtools.rs:26` — Doc comment references `azumi-live-ssr-framework` ⚠️ old package name (see P6 below)
- `client/azumi.js:573` — Checks `meta[name="azumi-dev"]` ✅ intentional (devtools detection)
- `client/azumi.js.backup` — ⚠️ stale backup file (see P7 below)

---

## Follow-Up Tasks

### P1 — Security: Action handler error messages leak internal state
- **File**: `macros/src/live.rs` (lines ~464, 468)
- **Problem**: Generated handlers return `format!("Security Error: {}", e)` and `format!("State Deserialization Error: {}", e)`
- **Fix**: Return generic `"Invalid request"` / `"Internal error"` — log details server-side
- **Risk**: Attackers can use error details to probe state structure

### P2 — Correctness: `LiveState::to_scope()` panics on serialization failure
- **Files**: `macros/src/live.rs` (line ~564), `src/lib.rs` (lines 133-139)
- **Problem**: `to_scope()` calls `panic!` on serde failure. Macro generates `to_scope()` not `try_to_scope()`
- **Fix**: Generate graceful fallback or use `try_to_scope()` with logging
- **Risk**: Production panic if a field doesn't implement `Serialize`

### P3 — API safety: `from_fn`/`from_fn_once` should be `pub(crate)`
- **Files**: `src/lib.rs:408` (`from_fn`), `src/lib.rs:338` (`from_fn_once`)
- **Problem**: `#[doc(hidden)]` but fully `pub` — user code can bypass safety guarantees
- **Fix**: Change to `pub(crate)` so only macro-generated code can access them
- **Note**: Keep `TrustedHtml` public — it's the documented escape hatch in AGENTS.md

### P4 — Security: `escape_css_string` should escape `(` and `)`
- **File**: `src/lib.rs` (lines 509-528)
- **Problem**: CSS `url()` can be used for data exfiltration via unescaped parens
- **Fix**: Add `'(' => result.push_str("\\(")` and `')' => result.push_str("\\)")`
- **Risk**: Low (requires prior CSS injection), but defense-in-depth

### P5 — Code quality: Duplicate `escape_html` implementations
- **Files**: `src/lib.rs:478-502` (`Escaped<T>`) and `src/script.rs:26-39` (`escape_html()`)
- **Problem**: Two separate implementations of the same 5-char escape logic could diverge
- **Fix**: Make `Escaped<T>` delegate to `escape_html()` or extract shared function

### P6 — Stale package name references
- **Files**: `src/lib.rs:12`, `src/devtools.rs:26`
- **Problem**: Doc comments reference `azumi-live-ssr-framework` (old package name)
- **Fix**: Update to current package name `azumi`

### P7 — Stale backup file
- **File**: `client/azumi.js.backup`
- **Problem**: Backup file committed to repo
- **Fix**: Remove from repo and add `*.backup` to `.gitignore`

### P8 — Test crate warnings
- **Files**: `demo/tests/bind_value_tests.rs`, `demo/tests/keyed_tests.rs`, `demo/tests/transition_tests.rs`
- **Problem**: 13 unused variable/import warnings in test code
- **Fix**: Prefix unused variables with `_` or remove unused imports
