# Project State

## Current Focus
Enforce stricter safety guarantees by completely banning Raw() usage in html! expressions

## Context
The previous implementation had complex validation rules for Raw() usage that were difficult to maintain and understand. This change simplifies the validation by completely prohibiting Raw() in favor of safer alternatives.

## Completed
- [x] Removed all specific validation rules for Raw() content patterns
- [x] Added comprehensive documentation explaining why Raw() is dangerous and what to use instead
- [x] Standardized error messages to point developers to the safe alternatives
- [x] Simplified the validation logic to a single check for Raw() presence

## In Progress
- [ ] Update all existing codebases to replace Raw() with the new safe injection macros

## Blockers
- Existing codebases may contain Raw() usage that needs to be migrated

## Next Steps
1. Update all documentation to reflect the new Raw() prohibition
2. Create migration tools to help developers replace Raw() with safe alternatives
3. Update the AI_GUIDE_FOR_WRITING_AZUMI.md with detailed migration instructions
