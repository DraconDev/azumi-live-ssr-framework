# Project State

## Current Focus
Document the prediction mechanism and its auto‑detection workflow for optimistic UI updates.

## Completed
- [x] Expanded **How It Works** section to describe the prediction lifecycle in detail
- [x] Added a visual **Prediction Flow** diagram illustrating server‑side generation and client‑side execution
- [x] Clarified **auto‑detection** requirements (both `#[azumi::live]` and `#[azumi::live_impl]` must be present)
- [x] Documented **manual override** behavior using `data-predict` attributes
- [x] Added guidance on **optimizer flow** (optimistic UI update → server reconciliation → morphing)
- [x] Included a note about the necessity of both macros for non‑empty predictions
- [x] Highlighted precedence rules when manual `data-predict` coexists with auto‑detected predictions
- [x] Updated reference to **data‑bind** for optimistic updates within the guide
