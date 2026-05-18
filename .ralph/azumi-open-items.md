# Azumi Open Items — COMPLETE

All actionable items resolved. Remaining items are either deferred or scoped as separate missions.

## Completed this loop:

### FN-034: Borrow-friendly component props ✅
- `&str`/`&T` params auto-inject lifetime, zero-clone rendering from `&self`
- 8 new tests, 1,782 total passing

### FN-035: Migration guide ✅
- `docs/migration/from-axum.md` — 6-step incremental adoption path

### FN-033: html_content! evaluation ✅
- Closed — TrustedHtml is sufficient, `html_content!` would be over-engineering

### P2: Benchmark data ✅
- Real runtime sizes: Azumi 10KB vs HTMX 15KB vs React 46KB (gzipped)
- Updated README + why-azumi with accurate measurements

### P3: token_parser.rs modularization ✅ (deferred)
- Documented proposed split but deferred — internal refactoring, low user benefit, high bug risk

### P2: Production example app (scoped out)
- Separate mission, not a codebase improvement. Current demo has 121 html! calls, 53 files, 7K lines.

## Final state: 1,782 tests pass, 0 failures, 0 clippy warnings
