# Project State

## Current Focus
Add debug logging of SEO global state and OG availability in `generate_head`

## Completed
- [x] Add `eprintln!("DEBUG generate_head: global = {:?}", global.is_some());` after the canonical link write
- [x] Add nested `eprintln!("DEBUG: global is Some, og = {:?}", g.open_graph.is_some());` inside the `global` guard
- [x] Add `eprintln!("DEBUG: og block entered");` before writing the OG meta tag
- [x] Leave `Cargo.lock` in an unmerged conflicted state (no resolved changes)
