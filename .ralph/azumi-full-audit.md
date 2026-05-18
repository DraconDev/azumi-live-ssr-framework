
# Azumi Full Audit — Demos, Scripts, Docs

## Goal
Audit every user-facing file in the repo. Create a comprehensive checklist in TODO.md, then systematically verify and fix each item.

## Scope
Every file a new user would encounter:
- README.md
- docs/why-azumi.md
- docs/guide.md
- docs/migration/from-axum.md
- AGENTS.md
- src/lib.rs (crate-level doc)
- Cargo.toml descriptions/keywords/repo URLs
- demo/ (all lesson files, blog, main.rs)
- benches/
- client/ (azumi.js, azumi.d.ts)
- CHANGELOG.md
- build.rs (integrity checks)
- Any other .md files at root

## Audit Checklist (update TODO.md with this)
For EACH file, verify:
1. **Axum relationship** — Says "builds on Axum", not "replaces" or "migration from"
2. **Runtime size** — "~10KB (gzipped)", not "3KB" or stale numbers
3. **Repo URL** — `DraconDev/azumi-live-ssr-framework`, not old URLs
4. **Route constants** — Uses `#[azumi::page]` route constants where applicable
5. **Import consistency** — `use azumi::prelude::*;` in demo files
6. **Handler naming** — Consistent `*_handler` pattern
7. **No `class:external`** — Uses Azumi scoped CSS (blog done, but verify)
8. **No `Raw()`** — Uses `TrustedHtml` instead
9. **No stale references** — No mentions of removed features, old versions, dead links
10. **Positioning** — "Live SSR", not "full-stack"; "builds on Axum", not "replaces"
11. **Lesson numbering** — H1/doc comment matches file name
12. **Lesson navigation** — Has `@LessonNav` with correct prev/next

## Process
1. Scan all files and create TODO.md with the full checklist
2. Work through checklist items, fixing as found
3. Build + test after each batch of fixes
4. Update TODO.md with progress

## Exit Criteria
- Every file audited
- All issues fixed or documented as intentional
- `cargo build -p azumi-demo` passes
- `cargo test` passes (1,782+ tests)
- Zero clippy warnings
