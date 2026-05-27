## Goal
Implement P0 transitions: az-transition:fade, :slide, :scale. Client-side + docs.

## Checklist
- [x] az-transition:fade — fade in/out on DOM enter/exit
- [x] az-transition:slide — slide open/closed (max-height + opacity)
- [x] az-transition:scale — scale 0.95↔1.0 with opacity
- [x] Config: duration=N via attribute parsing
- [x] Tests for transitions — 8 tests pass (fade, slide, scale, duration, multi, keyed, nested, preserve attrs)
- [x] Update AGENTS.md — transitions section added
- [x] Update docs/guide.md — transitions added to Feature Catalog + dedicated section