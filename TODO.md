# Azumi Full Audit — TODO

## Critical ✅
- [x] CLI version constant stale → now uses `env!("CARGO_PKG_VERSION")` — never drifts
- [x] Archive docs 3KB → `docs/archive/architecture.md`: `(~10KB gzipped)`
- [x] Archive comparison 3KB → `docs/archive/comparison.md`: `~10KB`

## Medium ✅
- [x] Internal review 3KB (5 occurrences) → `.internal/REVIEW_2026-05-17.md` — all fixed
- [x] Internal proposals 3KB (2 occurrences) → `.internal/PROPOSALS.md` — all fixed
- [x] Internal SEO doc repo name → `.internal/GITHUB_SEO.md` → `azumi-live-ssr-framework`
- [x] Internal SEO settings URL → updated to current repo name
- [x] Internal review dead links → fixed to `docs/archive/` paths
- [x] Internal review "full-stack" → changed to "Live SSR"
- [x] CLI Cargo.toml version drift → aligned to 47.45.0
- [x] Docs version "47" → "48" → `docs/guide.md`, `docs/migration/from-axum.md`
- [x] Source version "47" → "48" → `src/devtools.rs`

## Low ✅
- [x] `azumi.dev` domain → replaced with GitHub URL in demo main.rs
- [x] CHANGELOG.md → added v47.45.0 entry

## Already Correct (verified by audit)
- ✅ All public docs say "builds on Axum" — no replacement language
- ✅ No `class:external` in demo code — blog uses scoped CSS
- ✅ No `Raw()` usage in demo code — uses TrustedHtml
- ✅ No TODO/FIXME/HACK in production source
- ✅ README, why-azumi, guide, lib.rs all say "~10KB gzipped"
- ✅ All Cargo.toml repo URLs: `DraconDev/azumi-live-ssr-framework`
- ✅ Lesson numbering matches file names
- ✅ All lessons have `@LessonNav` with prev/next
- ✅ `use azumi::prelude::*` consistent in all lessons
- ✅ All handlers use `lesson{N}_handler` naming
- ✅ Route constants used in main.rs and homepage.rs where applicable
