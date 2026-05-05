# Project State

## Current Focus
Enhanced XSS protection in HTML injection macros by adding case-insensitive style tag validation.

## Context
This change builds on recent security improvements to Azumi's XSS protection system. The previous work focused on script tags, and this commit extends coverage to style tags with case-insensitive matching.

## Completed
- [x] Added case-insensitive style tag validation in `escape_style_content()`
- [x] Updated documentation in AGENTS.md to reflect expanded test coverage (22 tests now)

## In Progress
- [ ] None (this is a focused security enhancement)

## Blockers
- None (this is a self-contained security improvement)

## Next Steps
1. Verify test coverage for all style tag variations
2. Consider similar enhancements for other HTML tags if needed
