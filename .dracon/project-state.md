# Project State

## Current Focus
Enhance Azumi's XSS protection by completely banning `Raw()` usage in HTML macros

## Context
This change follows recent security-focused commits that strengthened Azumi's XSS protection mechanisms. The `Raw()` injection method was identified as a critical vulnerability point that bypassed all safety guarantees.

## Completed
- [x] Removed `Raw()` usage from HTML structure validation
- [x] Updated error message to clearly explain security risks
- [x] Added hidden documentation marker for internal component constructor

## In Progress
- [x] Comprehensive compile-time validation for XSS protection

## Blockers
- None identified in this commit

## Next Steps
1. Verify all safe injection alternatives are properly documented
2. Ensure all existing codebases using `Raw()` are migrated to safer alternatives
