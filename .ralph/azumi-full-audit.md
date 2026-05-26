## Goal
Complete systematic audit of every Azumi feature, macro, client-side capability, and Rust API. Nothing skipped. Every claim verified against source.

## Checklist

### Phase 1: Macros Surface Area
- [ ] `html!` macro — full token parser, validators, what it accepts
- [ ] `#[azumi::component]` — all options, props builder, children, live_state, defaults
- [ ] `#[azumi::page]` — route constants, what it generates
- [ ] `#[azumi::action]` — action path generation, error_fragment, success_fragment
- [ ] `#[azumi::live]` — full analysis pipeline, LiveStateMetadata, predictions
- [ ] `#[azumi::schema]` — what it does (gated behind feature)
- [ ] `json_data!` — what it generates

### Phase 2: Client JS Surface Area  
- [ ] Class structure (Azumi constructor, properties, methods)
- [ ] Event delegation (az-on parsing, handleEvent, handleFormSubmit)
- [ ] Action execution (callAction, execute, parseAction)
- [ ] State management (az-scope, az-ui, WeakMap, readState)
- [ ] Optimistic predictions (executePrediction, applyPrediction, rollbackPrediction)
- [ ] State bindings (updateBindings, data-bind, az-bind:text, az-bind:class)
- [ ] Form validation (validateFormField, isValidEmail, isValidUrl)
- [ ] Scroll reveal (setupReveal, observeRevealElements)
- [ ] Hot reload (connectHotReload, pollForReload, handleStyleUpdate)
- [ ] Predicate DSL (evaluatePredicate, evaluateExpression, parseTernary, findTernaryIndex, findOperatorIndex)
- [ ] Exact line counts per subsystem

### Phase 3: Rust Public API
- [ ] `Component` trait
- [ ] `TrustedHtml` 
- [ ] `escape_html`, `escape_xml`, `escape_css_string`
- [ ] `render_to_string`, `render_to_writer`
- [ ] `FormValidator`, `ValidatedForm`, `ValidationErrors`
- [ ] `ActionResult`, `error_fragment`, `success_fragment`
- [ ] `CspNonce`, ContentSecurityPolicy
- [ ] `SseEvent`, `sse()`
- [ ] `FnComponent`
- [ ] `AzumiScript`, `azumi_script()`, `session_cleanup_script()`
- [ ] `LiveState`, `LiveStateMetadata` traits
- [ ] Security: HMAC signing, `sign_state()`
- [ ] Prelude: what `use azumi::prelude::*` brings in

### Phase 4: Attribute Catalog
- [ ] Every attribute the client JS recognizes (complete list)
- [ ] Every attribute the macros recognize
- [ ] What each attribute does
- [ ] Which are used in production (dracon-platform) vs unused

### Phase 5: Validation Pipeline
- [ ] CSS validator
- [ ] HTML structure validator  
- [ ] Raw usage detector
- [ ] Format detector
- [ ] Class/ID validator
- [ ] Attribute validator
- [ ] Node ordering

### Phase 6: Documentation & Examples
- [ ] docs/guide.md
- [ ] docs/why-azumi.md
- [ ] docs/archive/ (10 files — what's stale?)
- [ ] demo/src/ — what examples exist
- [ ] AGENTS.md — what it covers

### Phase 7: Production Usage
- [ ] Every azumi attribute/property used in dracon-platform
- [ ] Every external JS file and what it does
- [ ] Patterns used (TrustedHtml + format!, az-action, etc.)
- [ ] What's conspicuously absent

### Phase 8: Client JS Subsystem Line Counts
- [ ] Exact LOC per function/method group
- [ ] Calculate removable vs critical code
- [ ] Identify the hot reload bug precisely

### Deliverable
Updated audit-tasks.md or audit-report.md with verified facts and strategic recommendations.