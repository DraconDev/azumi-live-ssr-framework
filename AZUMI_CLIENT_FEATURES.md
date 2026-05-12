# Azumi Client Features Guide

When building Azumi apps, you can often avoid writing custom JavaScript by using
Azumi's built-in client features. This guide explains when to use each feature.

## Quick Decision Tree

```
Need interactivity?
├─ Simple server action? → az-action on <form>
├─ Client-only state toggle? → az-ui + set
├─ Need conditional classes? → az-bind:class
├─ Need dynamic text? → az-bind:text
├─ Confirmation before action? → az-confirm
├─ Run action on page load? → az-init
├─ Scroll-triggered reveal? → az-reveal
├─ Smooth scroll to top? → scroll-top
├─ Full custom JS widget? → TypeScript in src/ts/
└─ External API (Stripe/Paddle)? → inline <script>
```

---

## 1. Form Actions (`az-action` + `az-target`)

**Purpose:** Submit a form to a server action and swap in the result HTML.

```html
<form az-action="save_settings" az-target="#result" az-swap="morph">
    <input name="theme" type="text">
    <button type="submit">Save</button>
</form>
<div id="result"></div>
```

**When to use:** Any form submission that should update a portion of the page
without a full navigation. Saves ~20-40 lines of custom JS per form.

**How it works:** Intercepts form submit, serializes form data, POSTs to
`/_azumi/action/{name}`, swaps response HTML into `az-target`.

**Server handler returns an HTML fragment:**
```rust
async fn save_settings(/* ... */) -> impl IntoResponse {
    // on success:
    azumi::action::success_fragment("<p>Settings saved!</p>")
}
```

---

## 2. Client-Side State (`az-ui` + `set` command)

**Purpose:** Manage state entirely on the client without server round-trips.

```html
<div az-ui='{"count":0,"active_tab":"rust"}'>
    <p>Count: <span az-bind:text="count">0</span></p>
    <button az-on="click set count = count + 1">+</button>
    <button az-on="click set count = count - 1">-</button>
    <button az-on="click set active_tab = 'js'">Show JS</button>
</div>
```

**When to use:** Tabs, accordions, counters, toggles, UI state that doesn't
need to persist on the server. Saves 10-30 lines of custom JS per widget.

**State priority chain (highest to lowest):**
1. Optimistic predictions (WeakMap, ephemeral)
2. `az-ui` attribute (client-set state via `set` command)
3. `az-scope` attribute (server-signed state)

**Supported expressions in `set`:**
- `set count = count + 1` — arithmetic
- `set active_tab = 'rust'` — string literals
- `set user.name = 'Alice'` — nested paths
- `set items.0.done = true` — array index + field

---

## 3. Event Handlers (`az-on`)

**Purpose:** Trigger actions on DOM events without writing JS.

```html
<!-- Click to call a server action -->
<button az-on="click call toggle_like -> #like-area">
    Toggle Like
</button>

<!-- Change event -->
<select az-on="change call filter -> #results">
    <option value="all">All</option>
    <option value="active">Active</option>
</select>
```

**When to use:** Any element that needs to trigger a server action on click,
change, input, or submit. More flexible than `az-action` (works on any
element, not just forms).

**Syntax:** `{event} call {action_name} -> {target} {swap_method}`

---

## 4. Text Bindings (`az-bind:text`)

**Purpose:** Dynamically display state values as text content.

```html
<div az-ui='{"name":"World"}'>
    <h1 az-bind:text="name">Placeholder</h1>
    <p az-bind:text="'Hello, ' + name + '!'"></p>
</div>
```

**When to use:** Any text that depends on client state. Combines with `set`
command for reactive updates.

---

## 5. Conditional Classes (`az-bind:class`)

**Purpose:** Toggle CSS classes based on state expressions.

```html
<style>
    .active { background: blue; color: white; }
    .highlight { outline: 2px solid gold; }
</style>

<div az-ui='{"liked":false,"selected":false}'>
    <!-- Colon syntax -->
    <button az-on="click set liked = !liked"
            az-bind:class:active="liked">
        ♥ Like
    </button>

    <!-- Dot syntax (alternative) -->
    <div az-bind:class.active="liked"
         az-bind:class.highlight="selected">
        Content
    </div>
</div>
```

**When to use:** Toggling classes based on client state. Saves ~10-15 lines
of JS per element (querySelector + classList.toggle).

**Expression evaluation:** Any valid expression that evaluates to truthy/falsy:
- `liked` — boolean field
- `count > 5` — comparison
- `tab == 'settings'` — string comparison
- `!hidden` — negation

---

## 6. Confirmation Dialogs (`az-confirm`)

**Purpose:** Show a confirmation dialog before executing an action.

```html
<form az-action="delete_account" az-confirm="Delete your account permanently?">
    <button type="submit">Delete</button>
</form>

<!-- Also works with az-on -->
<button az-on="click call revoke_key"
        az-confirm="Revoke this API key?">
    Revoke
</button>
```

**When to use:** Destructive actions (delete, revoke, cancel). Saves the need
for custom modal/dialog code.

---

## 7. Auto-Init (`az-init`)

**Purpose:** Execute an action automatically when the page loads.

```html
<div az-init="call load_data -> #feed">
    <!-- Content loaded on page load -->
    <div id="feed">Loading...</div>
</div>
```

**When to use:** Loading initial data, checking auth status, any action that
should fire on page load without user interaction. Saves `DOMContentLoaded`
boilerplate.

---

## 8. Scroll Reveal (`az-reveal`)

**Purpose:** Trigger CSS animations when elements scroll into viewport.

```html
<style>
    [az-reveal] {
        opacity: 0;
        transform: translateY(20px);
        transition: opacity 0.6s, transform 0.6s;
    }
    [az-reveal][data-revealed] {
        opacity: 1;
        transform: translateY(0);
    }
</style>

<div az-reveal>
    <h2>Fades in when visible</h2>
</div>
```

**When to use:** Scroll-triggered entrance animations. Uses IntersectionObserver
with a fallback for older browsers. Respects `prefers-reduced-motion`.

---

## 9. Scroll Top (`scroll-top`)

**Purpose:** Smooth scroll to top without JavaScript.

```html
<button az-on="click scroll-top">Back to Top</button>
```

**When to use:** Back-to-top buttons. Replaces `window.scrollTo()` calls.

---

## What Requires Custom JS

These cases still need TypeScript/JS:

| Feature | Why |
|---------|-----|
| Canvas/WebGL | Browser API, not DOM-based |
| WebSocket chat | Persistent connections |
| External API integration | Stripe, Paddle, Google Maps |
| Complex drag-and-drop | HTML5 DnD API |
| File handling | FileReader, drag-drop |
| Audio/video processing | Media APIs |

## Migration Checklist

When you find yourself writing custom JS for interactivity, ask:

1. **Form submission?** → `az-action` + `az-target`
2. **Toggling visibility/class?** → `az-ui` + `az-bind:class` + `set`
3. **Displaying dynamic text?** → `az-bind:text`
4. **Confirmation prompt?** → `az-confirm`
5. **Run on page load?** → `az-init`
6. **Scroll animation?** → `az-reveal`
7. **Scroll to top?** → `scroll-top`

If none of these fit, you genuinely need custom JS. Write it in TypeScript
and compile to `static/`.
