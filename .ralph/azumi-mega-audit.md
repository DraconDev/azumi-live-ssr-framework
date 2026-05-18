# Azumi Full Audit — Every File, Every Line

## Goal
Systematically audit every user-facing and internal file in the repo.

## Audit Checklist — ALL VERIFIED

### 1. Naming & Conventions ✅
- [x] No `_azumi` or `__azumi` prefixes in source, docs, or client (only: test fn names, `render_azumi` method, `data-azumi-scope` attr — all correct)
- [x] No `_az-` prefixes in HTML attributes
- [x] `az-` attributes consistent between Rust codegen, JS client, and docs
- [x] `data-*` attributes follow HTML5 convention
- [x] No `_` prefix on things that don't need it
- [x] Crate name = `azumi-live-ssr-framework`, lib name = `azumi`
- [x] All `cargo add` / dependency refs use new crate name
- [x] All feature flags reference new crate name
- [x] Archive: `/_azumi/` → `/azumi/` in js-exposure-analysis.md (fixed this round)

### 2. Runtime Size ✅
- [x] No "3KB"/"~3KB"/"<3KB" in non-archive files (only CHANGELOG/TODO historical entries)
- [x] All say "~10KB (gzipped)" or "~10KB gzipped"

### 3. Axum Relationship ✅
- [x] No "replaces Axum" / "migration from Axum" / "instead of Axum"
- [x] Says "builds on Axum" in lib.rs, guide.md, from-axum.md, README.md
- [x] Migration guide says "Adding Azumi to Your Axum App"
- [x] Checklist says "Adoption Checklist" not "Migration Checklist"

### 4. Positioning ✅
- [x] No "full-stack" describing Azumi (only for competitors like Next.js)
- [x] "Live SSR" is the primary descriptor
- [x] No "compiled web" or "compiled HTML" as primary tagline

### 5. Repo & URLs ✅
- [x] All URLs point to `DraconDev/azumi-live-ssr-framework`
- [x] No old repo name `DraconDev/azumi` (without -live-ssr-framework)
- [x] No `azumi.dev` domain references
- [x] `dracon.uk` only in COMMERCIAL-LICENSE.md (company domain, correct)

### 6. Versions ✅
- [x] Version references say "47" (matching Cargo.toml major version)
- [x] No stale "48" in docs (was fixed — crate is major 47, not 48)
- [x] CLI uses `env!("CARGO_PKG_VERSION")` not hardcoded
- [x] CHANGELOG has entries for all recent versions

### 7. Demo / Lessons ✅
- [x] All 21 lessons have `lesson_nav`
- [x] No `class:external` in demo code
- [x] No `Raw()` usage in demo code
- [x] Blog uses scoped CSS, TrustedHtml, no global styles
- [x] `Raw()` in blog/data.rs is inside HTML string content (educational, not code)

### 8. Code Quality ✅
- [x] No `dbg!` traces in production code
- [x] No TODO/FIXME/HACK in production source
- [x] `println!`/`eprintln!` only in devtools/hot_reload (dev-only)
- [x] `unwrap()` calls are safe (unwrap_or/unwrap_or_default/unwrap_or_else)
- [x] `#[allow(dead_code)]` only on `Raw::new()` (used by macros, compiler can't see it)
- [x] 1,782 tests pass, 0 failures

### 9. Docs Accuracy ✅
- [x] README: `cargo add azumi-live-ssr-framework` first
- [x] guide.md: correct crate name, feature flags, version
- [x] from-axum.md: "Adoption Checklist", "Convert" not "Replace"
- [x] AGENTS.md: `/_azumi/` → `/azumi/` fixed
- [x] lib.rs crate doc: "Builds on Axum"
- [x] No broken internal links between docs

### 10. Security & Integrity ✅
- [x] build.rs FNV hash matches current azumi.js
- [x] client.min.js regenerated from azumi.js
- [x] No hardcoded secrets in production code (dev-secret in security.rs is intentional)
- [x] `cargo build -p azumi-demo` passes
- [x] `cargo test` passes (1,782)
