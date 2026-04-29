# Project State

## Current Focus
Clarify prediction attribute auto‑injection and update documentation table for method mutation predictions

## Completed
- [x] Add clarification that `data-predict` attributes are not auto‑injected and must be added manually, while noting `#[azumi::live_impl]` auto‑analyzes simple mutations
- [x] Update the table entry for `state.counter = 42` to specify usage of `#[azumi::predict]` on a method within `#[azumi::live_impl]`
