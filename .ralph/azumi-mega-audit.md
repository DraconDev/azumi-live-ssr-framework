
# Azumi Full Audit — Every File, Every Line

## Goal
Systematically audit every user-facing and internal file in the repo. No assumptions about what's clean — verify everything.

## Audit Checklist

### 1. Naming & Conventions
- [ ] No `_azumi` or `__azumi` prefixes anywhere (routes, JS, Rust, macros)
- [ ] No `_az-` prefixes in HTML attributes
- [ ] `az-` attributes are consistent between Rust codegen, JS client, and docs
- [ ] `data-*` attributes follow HTML5 convention (no underscore in custom part)
- [ ] No `_` prefix on things that don't need it (unused vars ok, but `_azumi_*` no)
- [ ] Crate name = `azumi-live-ssr-framework`, lib name = `azumi`
- [ ] All `cargo add` / dependency refs use new crate name
- [ ] All feature flags reference new crate name

### 2. Runtime Size
- [ ] No "3KB" or "~3KB" or "<3KB" in any non-archive file
- [ ] All say "~10KB (gzipped)" or "~10KB gzipped"
- [ ] Check src/, benches/, demo/, docs/, macros/, client/, CHANGELOG, TODO

### 3. Axum Relationship
- [ ] No "replaces Axum" / "migration from Axum" / "instead of Axum" language
- [ ] Says "builds on Axum" or "on top of Axum" everywhere
- [ ] Migration guide says "Adding Azumi to Your Axum App"
- [ ] Checklist says "Adoption" not "Migration"

### 4. Positioning
- [ ] No "full-stack" describing Azumi (only ok for competitors like Next.js)
- [ ] "Live SSR" is the primary descriptor
- [ ] No "compiled web" or "compiled HTML" as primary tagline

### 5. Repo & URLs
- [ ] All URLs point to `DraconDev/azumi-live-ssr-framework`
- [ ] No references to old repo name `DraconDev/azumi` (without -live-ssr-framework)
- [ ] No `azumi.dev` domain references (unverified domain)
- [ ] No `dracon.uk` references (not project-related)

### 6. Versions
- [ ] Cargo.toml versions match between crates where appropriate
- [ ] No stale version "47" in docs (should be "48")
- [ ] CHANGELOG has entries for all recent versions
- [ ] CLI uses `env!("CARGO_PKG_VERSION")` not hardcoded

### 7. Demo / Lessons
- [ ] All 21 lessons: file name matches doc comment / H1
- [ ] All lessons: `use azumi::prelude::*;`
- [ ] All lessons: `lesson{N}_handler` naming
- [ ] All lessons: `#[azumi::page(route)]` where applicable
- [ ] All lessons: route constants in main.rs
- [ ] All lessons: `@LessonNav` with correct prev/next
- [ ] No `class:external` anywhere
- [ ] No `Raw()` usage anywhere
- [ ] Blog uses scoped CSS, TrustedHtml, no global styles

### 8. Code Quality
- [ ] No `dbg!` traces in production code
- [ ] No TODO/FIXME/HACK in production source (not in .ralph/ or .internal/)
- [ ] No leftover `println!` in library code (devtools/hot_reload ok)
- [ ] No panicking `unwrap()` in production paths (unwrap_or ok)
- [ ] `#[allow(dead_code)]` only where justified
- [ ] Zero clippy warnings

### 9. Docs Accuracy
- [ ] README: `cargo add azumi-live-ssr-framework` first
- [ ] guide.md: correct crate name, feature flags, version
- [ ] from-axum.md: "Adoption Checklist", "Convert" not "Replace"
- [ ] AGENTS.md: references match current API
- [ ] lib.rs crate doc: "Builds on Axum"
- [ ] No broken internal links between docs

### 10. Security & Integrity
- [ ] build.rs FNV hash matches current azumi.js
- [ ] idiomorph.js hash is current
- [ ] client.min.js is regenerated from azumi.js
- [ ] No hardcoded secrets in production code

## Process
1. Read every file category by category
2. Flag issues with file:line:current→suggested
3. Fix immediately if trivial
4. Build + test after each batch
5. Update this checklist with results

## Exit Criteria
- All checklist items verified ✅ or documented as intentional
- `cargo build -p azumi-demo` passes
- `cargo test` passes (1,782+)
- Zero clippy warnings
