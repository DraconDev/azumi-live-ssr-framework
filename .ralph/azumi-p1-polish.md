## Goal
Implement P1 premium polish: loading/error states, debounce, lifecycle hooks, structured confirms, #[react].

## Checklist
- [x] az-loading class — add/remove during fetch, aria-busy attribute
- [x] az-error class — added to target on fetch failure, cleared on next success
- [x] Spinner component — demo-only (cannot use html! inside crate), at demo/src/examples/spinner.rs
- [x] debounce=N on az-on:input — parsed from az-on attr, setTimeout-based queue
- [x] az-on:mount — fired on new elements after morph completes
- [x] az-on:unmount — fired on elements before morph removes them
- [x] az-on:update — fired on changed elements after morph
- [x] Confirm component — demo-only pattern at demo/src/examples/confirm.rs (accessible, keyboard support)
- [x] Built-in Confirm — demo pattern, not framework feature (too complex for universal bundle)
- [SKIP] #[react] computed fields — requires macro changes to live.rs, best deferred to P2