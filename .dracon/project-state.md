# Project State

## Current Focus
Improved guidance for JavaScript content patterns in Azumi's HTML structure validator

## Context
The validator now provides clearer guidance for handling JavaScript content within Azumi's secure HTML framework, replacing outdated patterns with new procedural macros.

## Completed
- [x] Updated documentation to recommend new procedural macros (`inline_script!`, `json_data!`, `azumi_script()`) instead of raw JavaScript
- [x] Removed outdated examples showing direct `<script>` tag usage
- [x] Added specific guidance for different JavaScript use cases

## In Progress
- [ ] No active work in progress

## Blockers
- None identified

## Next Steps
1. Verify all affected codebases have migrated to the new patterns
2. Update related documentation files to reflect these changes
