# Project State

## Current Focus
docs(ai_guide): shift prediction from manual `data-predict` to auto‑detected via `#[azumi::live_impl]` and refine examples

## Completed
- [x] docs(ai_guide): explain Zombie State and removal of bare client state
- [x] docs(ai_guide): clarify that `#[azumi::live_impl]` now auto‑detects simple mutations and injects `az-predictions` JSON
- [x] docs(ai_guide): update examples to omit `data-predict` for common cases and show manual overrides
- [x] docs(ai_guide): add `#[azumi::predict("...")]` syntax for complex predictions
- [x] docs(ai_guide): detail auto‑detection flow and precedence rules
- [x] docs(ai_guide): enhance prediction table with new examples
- [x] docs(ai_guide): adjust rendering examples to use new helper functions (`my_page`, `view`)
- [x] docs(ai_guide): remove obsolete `set` command example
- [x] docs(ai_guide): update development server section placeholder
- [x] docs(ai_guide): update error messages section to reflect new prediction behavior
- [x] docs(ai_guide): correct section numbering and ordering after previous refactors
- [x] docs(ai_guide): add migration guide snippets for auto‑prediction changes
- [x] docs(ai_guide): align with new component macro that centralizes live‑state scope wrapper generics
- [x] docs(ai_guide): include examples of manual `data-predict` taking precedence over auto‑detected predictions
- [x] docs(ai_guide): add notes on compilation-time prediction analysis and runtime execution order
- [x] docs(ai_guide): adjust examples for `#[azumi::predict]` on methods
- [x] docs(ai_guide): update complex logic section to reflect new prediction workflow
- [x] docs(ai_guide): remove duplicate “Client Runtime Integration” section
- [x] docs(ai_guide): update changelog and migration guide references for v28.0.2 release
- [x] docs(ai_guide): reflect new auto‑detection of predictions in all relevant subsections
- [x] docs(ai_guide): improve clarity on when predictions are sent (original signed state) and reconciled
- [x] docs(ai_guide): provide concise auto‑detection flow chart and bullet points
- [x] docs(ai_guide): correct rendering examples to use `azumi::render_to_string` for consistency
- [x] docs(ai_guide): update `current_user` example to reflect new rendering approach
- [x] docs(ai_guide): add guidance on manual `data-predict` for custom or complex cases
