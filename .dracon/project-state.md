# Project State

## Current Focus
Added new procedural macros for inline injection of JSON data, CSS, and scripts to the macros library.

## Context
This change expands the existing procedural macro functionality to support inline injection of JSON data, CSS, and scripts, which were recently added as new features. The changes ensure these new macros are properly exposed in the library's prelude and public interface.

## Completed
- [x] Added `inline_css`, `inline_script`, and `json_data` macros to the prelude exports
- [x] Updated the public macro re-exports to include the new inline injection macros
- [x] Maintained backward compatibility with existing macro exports

## In Progress
- [ ] None (this change is complete)

## Blockers
- None (this change is complete)

## Next Steps
1. Verify the new macros work correctly in downstream projects
2. Update documentation to reflect the new inline injection capabilities
