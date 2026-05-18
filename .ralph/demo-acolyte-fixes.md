
# Fix Azumi Demos for New Acolytes

## Goal
Make the demo lessons consistent, idiomatic, and aligned with the framework's own best practices. A new user should be able to learn from these demos without confusion.

## Critical Fixes (must do)
1. **Fix lesson numbering** — Doc comments say "Lesson N" but file is `lesson{N-1}.rs`. Make content match the file: lesson0 = "Lesson 0", lesson1 = "Lesson 1", etc.
2. **Add `#[azumi::page(route)]` to all lessons** — Generate route constants, use them in main.rs and homepage links
3. **Standardize imports** — All lessons use `use azumi::prelude::*;`
4. **Standardize handler names** — All use `{name}_handler` pattern
5. **Fix lesson5 "6.5" → just make it lesson 5** — Remove the weird 6.5 numbering
6. **Blog: remove `class:external`** — Use Azumi scoped CSS instead (the whole point of the framework)

## Medium Fixes (should do)
7. **Add Previous/Next navigation** to each lesson page
8. **Extract shared LessonPage component** — DRY up the repeated key_points/examples/style pattern
9. **Verify lesson12 @Image exists** — If not, replace with a real topic
10. **Fix lesson20 slider TODO** — Make it work or remove the placeholder

## Checklist
- [ ] Fix all lesson doc comment numbering (lesson0="Lesson 0", lesson1="Lesson 1", ...)
- [ ] Add #[azumi::page(route)] to all 21 lesson pages + homepage
- [ ] Update main.rs to use route constants everywhere
- [ ] Update homepage.rs lesson cards to use route constants
- [ ] Standardize all imports to `use azumi::prelude::*;`
- [ ] Standardize all handler names to `lesson{N}_handler`
- [ ] Fix lesson5 "6.5" → "Lesson 5: @let Pattern"
- [ ] Blog: replace class:external with scoped CSS
- [ ] Extract LessonPage shared component (DRY)
- [ ] Add prev/next navigation to lessons
- [ ] Verify/fix lesson12 content
- [ ] Fix lesson20 placeholder
- [ ] All 1,782+ tests still pass
- [ ] cargo build -p azumi-demo succeeds
