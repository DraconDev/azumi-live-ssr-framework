# Azumi Demo Review (FN-036)

Review all demo code for consistency with new features and positioning. The demo is the first thing users see — it must showcase "server-rendered HTML with client interactivity, zero custom JS."

10 areas to check:
1. Route constants — pages using `#[azumi::page(route)]` + `_ROUTE` constants
2. `{&field}` borrow — replace unnecessary `.clone()` 
3. `TrustedHtml` vs old patterns
4. az- directive showcase — demonstrate all "no custom JS" features
5. Lesson progression — 20 lessons checked for currency
6. Main.rs routing — `_ROUTE` constants everywhere
7. SEO/SeoConfig — new `SeoError` enum
8. CSS class naming — snake_case convention
9. aria attributes — new validator may catch invalid values
10. Positioning alignment — landing page text reflects new pitch

Checklist:
- [ ] 1. Audit route constants in demo
- [ ] 2. Audit .clone() patterns → {&field}
- [ ] 3. Audit Raw()/format!() → TrustedHtml/@for
- [ ] 4. Audit az- directive coverage
- [ ] 5. Audit lesson currency
- [ ] 6. Audit main.rs routing
- [ ] 7. Audit SEO usage
- [ ] 8. Audit CSS naming
- [ ] 9. Audit aria values
- [ ] 10. Audit landing page positioning
- [ ] 11. Fix everything found
- [ ] 12. All tests pass