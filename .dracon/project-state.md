# Project State

## Current Focus
Add comprehensive unit tests for the `asset_rewriter::rewrite_path` function and associated node rewriting behavior.

## Completed
- [x] Added test ensuring relative paths (e.g., `"logo.png"`, `"static/logo.png"`) return `None`.
- [x] Added test ensuring absolute paths without a manifest (e.g., `"/logo.png"`) return `None`.
- [x] Added test ensuring an empty string path returns `None`.
- [x] Added test verifying that an empty vector of nodes remains empty after calling `rewrite_nodes`.
