# Project State

## Current Focus
Simplify attribute filtering by removing intermediate collections and directly filtering out `local` and `computed` attributes.

## Completed
- [x] Eliminate `filtered_attrs` and `filtered_fields` variables and associated logic
- [x] Filter out `local` and `computed` attributes inline when building `filtered_named_fields`
- [x] Replace previous multi‑step filtering with a single `filter_map` call that retains only relevant fields
