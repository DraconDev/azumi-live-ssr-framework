# Azumi Open Items — Systematic Completion

## Completed this loop:

### FN-034: Borrow-friendly component props ✅
- `&str`/`&T` params auto-inject lifetime, zero-clone rendering from `&self`
- 8 new tests, 1,782 total passing

### FN-035: Migration guide ✅
- `docs/migration/from-axum.md` — 6-step incremental adoption path

### FN-033: html_content! evaluation ✅
- Evaluated and closed — TrustedHtml already solves CMS injection, `html_content!` would be over-engineering

### P2: Benchmark data ✅
- Real runtime size comparison: Azumi 10KB gzipped vs HTMX 15KB vs React 46KB
- Updated README.md and docs/why-azumi.md with real measurements
- Azumi is 64% of HTMX's size, 22% of React's size

## Remaining:

- [ ] P2: Production example app (replace toy demo with blog+auth+admin)
- [ ] P3: token_parser.rs modularization (1,349 lines → submodules)
  - Proposed split: ast.rs (types 1-167), style_parser.rs (170-250), core_parser.rs (324-836), helpers.rs (837-1076), control_flow.rs (1077-1349)
  - Decision: deferred — internal refactoring with no user benefit, risk of introducing bugs
