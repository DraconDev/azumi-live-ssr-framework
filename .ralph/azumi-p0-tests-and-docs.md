## Goal
Complete remaining P0 tasks: tests for bind:value and @keyed, then P0.4 docs/AGENTS.md updates.

## Checklist

### Tests
- [x] `tests/bind_value_tests.rs` — 8 tests pass
- [x] `tests/keyed_tests.rs` — 8 tests pass

### P0.4: Promote
- [x] Update AGENTS.md — added bind:value/bind:checked, @keyed, Scoped CSS, Form Validation sections
- [x] Demo form: `demo/src/examples/form_validation_demo.rs` — signup form with bind:value + data-validate (4 fields, 4 tests)
- [x] Add max/min/pattern rules to data-validate — `validateFormField` now supports 8 rules (was 5)
- [x] Update docs/guide.md — added @keyed section under @for, bind:value section under Forms