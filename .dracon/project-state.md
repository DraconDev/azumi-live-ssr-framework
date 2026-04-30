# Project State

## Current Focus
Shift optimistic prediction from manual `data-predict` to auto-detected `az-predictions` injected by `#[azumi::live_impl]`, streamlining server-driven UI updates.

## Completed
- [x] Replace manual prediction syntax with server-generated `az-predictions` JSON that maps actions to DSL expressions.
- [x] Restrict `az-on` to `call` commands only, removing local `set` in favor of server roundtrips and rollback-capable predictions.
- [x] Namespace server action endpoints to `POST /_azumi/action/{StructName}/{MethodName}` and require `_azumi_scope` in form payloads.
- [x] Update documentation and examples to reflect auto-detection, precedence rules, and rollback behavior on server error.
