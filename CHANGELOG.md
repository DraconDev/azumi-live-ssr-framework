# Changelog

All notable changes to Azumi will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
