# Project State

## Current Focus
docs(architecture): document validation parameter naming convention for scoped CSS selectors

## Context
The architecture documentation needed clarification on how scoped CSS selectors are validated within the component system. The `valid_classes` and `valid_ids` parameters in `validate_nodes` were introduced to enforce allowlist validation against component-scoped styles.

## Completed
- [x] documented the purpose of `valid_classes` and `valid_ids` parameters
- [x] clarified that these are allowlists for validation, not arbitrary data structures
- [x] linked to the CSS selector extraction mechanism in `crate::css::extract_selectors`

## In Progress
- [ ] none

## Blockers
- none

## Next Steps
1. Review for consistency with other documentation sections
2. Verify that this matches the actual implementation behavior
```
