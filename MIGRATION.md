# Migration Guide

## v26 → v27 (Breaking Change)

### What Changed

In v27.0.0, the `#[azumi::live]` and `#[azumi::live_impl]` macros were separated:

| Before (v26) | After (v27) |
|-------------|-------------|
| `#[azumi::live]` provided `LiveStateMetadata` + `LiveState` traits | `#[azumi::live]` only adds derives + field constants |
| `#[azumi::live_impl]` added action handlers | `#[azumi::live_impl]` now provides `LiveStateMetadata` + `LiveState` + handlers |
| Manual `data-predict` required on every button | Predictions auto-detected from `#[azumi::live_impl]` via `az-predictions` JSON |

### Required Changes

#### 1. Add `#[azumi::live_impl]` if you only had `#[azumi::live]`

**Before (v26):**
```rust
#[azumi::live]
pub struct MyState {
    pub count: i32,
}

// LiveStateMetadata was provided by #[azumi::live]
// This used to work
```

**After (v27):**
```rust
#[azumi::live]
pub struct MyState {
    pub count: i32,
}

#[azumi::live_impl]  // ← NOW REQUIRED for LiveStateMetadata
impl MyState {
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
```

#### 2. Remove manual `data-predict` (optional but recommended)

**Before (v26):**
```rust
<button on:click={state.increment} data-predict="count = count + 1">"+1"</button>
```

**After (v27):**
```rust
// Predictions are auto-detected from #[azumi::live_impl]!
<button on:click={state.increment}>"+1"</button>
```

**Keep manual `data-predict` only for:**
- Complex mutations that can't be auto-detected
- Custom prediction logic
- Overriding auto-detected predictions

#### 3. Verify predictions are working

Check your rendered HTML for the `az-predictions` attribute:
```html
<div az-scope="..." az-struct="MyState" az-predictions='[["increment","count = count + 1"]]'>
```

If `az-predictions` is missing, you probably forgot `#[azumi::live_impl]`.

### Why This Changed

The previous architecture had `#[azumi::live]` implementing `LiveStateMetadata` with empty predictions. `#[azumi::live_impl]` collected predictions but couldn't update the trait impl (Rust forbids duplicate implementations).

The new architecture:
- `#[azumi::live]` = data shape (struct fields, serialization)
- `#[azumi::live_impl]` = behavior (methods, predictions, handlers, traits)

This enables:
- ✅ Auto-detected predictions via `az-predictions` JSON
- ✅ No manual `data-predict` for simple mutations
- ✅ Cleaner separation of concerns

### Rollback

If you need to stay on v26:
```toml
[dependencies]
azumi-live-ssr-framework = "=26.7.0"  # Pin to v26
```
