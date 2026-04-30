# Azumi: A Technical Description

> **Compiler-Driven Optimistic UI for Rust**

---

## What Is Azumi?

Azumi is a web framework that generates **client-side predictions from server-side Rust code**.

You write mutation logic once:

```rust
#[azumi::live]
pub struct Counter { pub count: i32 }

#[azumi::live_impl(component = "counter_view")]
impl Counter {
    pub fn increment(&mut self) {
        self.count += 1;
    }
}
```

The `#[azumi::live_impl]` macro analyzes the mutations and stores predictions in `LiveStateMetadata`. The component macro injects these predictions as `az-predictions` JSON on the scope div:

```html
<div az-scope="{signed_json}" az-struct="Counter"
     az-predictions='[["increment","count = count + 1"]]'>
    <button az-on="click call increment">"+1"</button>
</div>
```

The client JavaScript auto-detects and executes predictions when buttons are clicked. The server confirms (or corrects) the prediction when the request arrives.

For custom predictions or complex mutations, you can still add manual `data-predict` attributes — they take precedence over auto-detected predictions.

This is "optimistic UI" done at the language level, not as a library pattern.

---

## The Core Observation

Most web interactions are predictable:

-   Click "Like" → count goes up
-   Submit form → success or error
-   Toggle filter → list updates

Traditional frameworks either:

-   **Wait for the server** (Rails, HTMX) — feels sluggish
-   **Run everything client-side** (React, Vue) — complexity explosion
-   **Hydrate the whole page** (Next.js) — wasteful double-rendering

Azumi's answer: **compile the predictions**.

The server remains the source of truth. The client just guesses what the truth will be — and the compiler guarantees the guess is correct.

---

## What Azumi Actually Is

| Layer                   | What It Does                                              |
| ----------------------- | --------------------------------------------------------- |
| **HTML Macro**          | `html!` outputs server-rendered HTML (like Maud)          |
| **CSS Validator**       | Catches undefined classes at compile time                 |
| **Live Analysis**       | Analyzes `&mut self` methods, stores predictions in metadata |
| **Signed State**        | HMAC-signs component state to prevent tampering           |
| **Asset Pipeline**      | Content-hashes static files, rewrites paths automatically |

Azumi is _not_:

-   A client-side framework (no WASM, no Virtual DOM)
-   A meta-framework (no file-system routing, no built-in ORM)
-   A batteries-included CMS (bring your own auth, database)

---

## How It Compares

### vs Next.js (Hydration Model)

|                   | Azumi                | Next.js                   |
| ----------------- | -------------------- | ------------------------- |
| First interaction | Instant (prediction) | After JS loads + hydrates |
| JS payload        | ~3kb                 | ~80kb+                    |
| State location    | Server (signed)      | Client (trusting)         |
| CSS validation    | Compile-time         | Runtime (or none)         |

Next.js renders on server, then re-runs the same logic on client to "hydrate". Azumi never re-runs logic — the HTML is truth.

### vs HTMX (Pure SSR Model)

|               | Azumi             | HTMX                 |
| ------------- | ----------------- | -------------------- |
| Click latency | ~0ms (optimistic) | ~100ms (network RTT) |
| Complexity    | Rust macros       | HTML attributes      |
| Type safety   | Full              | None                 |

HTMX waits for the server on every click. Azumi guesses instantly.

### vs Leptos/Dioxus (WASM Model)

|              | Azumi          | Leptos                 |
| ------------ | -------------- | ---------------------- |
| Initial load | Instant (HTML) | ~300ms (WASM download) |
| Runtime size | ~3kb JS        | ~150kb WASM            |
| SEO          | Perfect        | Requires SSR setup     |

WASM frameworks front-load the cost. Azumi starts instantly.

### vs Svelte 5 (Compiled Reactivity)

|                  | Azumi         | Svelte 5     |
| ---------------- | ------------- | ------------ |
| Where logic runs | Server        | Client       |
| State management | Server-signed | Client runes |
| Hydration        | None          | Partial      |

Svelte compiles _client-side_ reactivity. Azumi compiles _server-side_ predictions.

---

## The Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      USER CLICKS                        │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│              CLIENT (azumi.js ~3kb)                     │
│                                                         │
│   1. Read az-scope JSON from element                    │
│   2. Apply compiled prediction (instant DOM update)     │
│   3. POST to server with signed state                   │
│                                                         │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼ (async)
┌─────────────────────────────────────────────────────────┐
│              SERVER (Rust/Axum)                         │
│                                                         │
│   1. Verify HMAC signature (reject tampering)           │
│   2. Execute real mutation (may hit DB)                 │
│   3. Re-render component HTML                           │
│   4. Return HTML fragment                               │
│                                                         │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│              CLIENT (reconciliation)                    │
│                                                         │
│   If server HTML differs from prediction → patch DOM    │
│   (Usually they match, so this is a no-op)              │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Unique Capabilities

### 1. CSS-HTML Co-Validation

The compiler knows your styles. Use a class that doesn't exist? **Compile error.**

```rust
html! {
    <style>
        .my_button { background: "#3b82f6"; }
    </style>

    <div class={my_buttn}>  // ❌ Error: "my_buttn" undefined
        "Oops"
    </div>
}
```

No other framework does this.

### 2. Zero Hydration Tax

Next.js sends HTML, then sends JS to rebuild the same thing in memory. Azumi sends HTML — done. The ~3kb runtime is just an event delegator.

### 3. Signed State

Every component's state is HMAC-signed. Open DevTools, edit `isAdmin: false` to `true`, click button → **400 Bad Request**. No code needed.

### 4. Optimistic Async

For async operations, add `data-predict` to show state changes instantly:

```rust
<button on:click={state.load_users} data-predict="loading = true">"Load"</button>
```

UI shows loading state _instantly_. Data arrives later from the server.

---

## What It's Best For

✅ **Excellent fit:**

-   CRUD applications (dashboards, admin panels)
-   Content sites that need interactivity (blogs with comments, e-commerce)
-   Internal tools where correctness > polish
-   SEO-critical sites

⚠️ **Outside core scope:**

-   Offline-first apps (server is truth — offline needs sync layer)
-   Continuous real-time collaboration (use dedicated WebSocket layer)
-   60fps games / heavy canvas work (use JS interop)

---

## The Philosophy

Azumi is opinionated:

1. **Server is truth** — no client-side state management
2. **Compiler is the bridge** — predictions are generated, not hand-written
3. **HTML is native** — no Virtual DOM, no WASM, no hydration
4. **Security by default** — signed state, XSS prevention

These aren't limitations waiting to be fixed. They're the design.

---

## In One Sentence

**Azumi is what you get when you take the simplicity of Rails, the speed of client-side React, and the safety of Rust — and compile them into one thing.**

---

_© 2026 Azumi Project_
