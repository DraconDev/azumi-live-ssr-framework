## Goal
Complete remaining P0 tasks: tests for bind:value and @keyed, then P0.4 docs/AGENTS.md updates.

## Checklist

### Tests
- [x] `tests/bind_value_tests.rs` — 8 tests pass (attribute generation, checkbox, nested, select/textarea, multi-input, preserve attrs)
- [x] `tests/keyed_tests.rs` — 8 tests pass (data-key generation, no-key fallback, empty list, string keys, first-element-only, @if inside, component integration)

### P0.4: Promote
- [ ] Update AGENTS.md with bind:value, @keyed, scoped CSS, data-validate
- [ ] Demo form: bind:value + data-validate together (signup form)
- [ ] Update docs/guide.md with bind:value and @keyed sections
- [ ] Add max/min/pattern rules to data-validate