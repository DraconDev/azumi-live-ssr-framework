# Project State

## Current Focus
Refactored procedural macros to use explicit expansion functions instead of direct proc_macro attributes

## Context
The previous implementation used `#[proc_macro]` attributes directly on the macro functions, which is now being replaced with explicit expansion functions (`expand_*`) to better control the macro expansion process and improve maintainability.

## Completed
- [x] Renamed `json_data` to `expand_json_data` and removed `#[proc_macro]` attribute
- [x] Renamed `inline_css` to `expand_inline_css` and removed `#[proc_macro]` attribute
- [x] Renamed `inline_script` to `expand_inline_script` and removed `#[proc_macro]` attribute

## In Progress
- [ ] Update macro invocations to use the new expansion functions instead of the old attribute-based syntax

## Blockers
- Macro invocations in the codebase need to be updated to use the new expansion functions

## Next Steps
1. Update all macro invocations in the codebase to use the new expansion functions
2. Verify that the refactored macros still produce the same output as before the change
