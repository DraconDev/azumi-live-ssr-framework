# Azumi Client (azumi.js)

The `azumi.js` client is a lightweight, framework-agnostic runtime that powers Azumi's interactivity. It provides declarative event delegation, optimistic UI updates, and intelligent DOM morphing without requiring a heavy frontend framework.

While designed to work seamlessly with `DraconDev/azumi-live-ssr-framework`, it can be used independently with any backend that follows the Azumi protocol.

## Features

-   **Event Delegation**: Declarative `az-on` attributes for click, submit, change, and input events.
-   **Optimistic UI**: Instant state updates via auto-detected `az-predictions` or manual `data-predict` overrides.
-   **DOM Morphing**: Uses [Idiomorph](https://github.com/bigskysoftware/idiomorph) for smooth DOM transitions, preserving focus and input state.
-   **Hot Reload**: Built-in WebSocket connection for instant development feedback.
-   **Micro-State Management**: Embed JSON state directly in the DOM with `az-scope`.

## Installation

The client expects `Idiomorph` to be available globally (optional but recommended for morphing).

```html
<!-- 1. Include Idiomorph (recommended) -->
<script src="https://unpkg.com/idiomorph"></script>

<!-- 2. Include Azumi Client -->
<!-- You can use the raw file or the minified version from the release -->
<script src="/path/to/azumi.js"></script>
```

The client initializes automatically as `window.azumi`.

## Core Concepts

### 1. Scopes (`az-scope`)

Components store their state as a JSON string in the `az-scope` attribute. This serves as the single source of truth for the client.

```html
<div id="counter" az-scope='{"count": 0}' az-struct="CounterState">
    <span data-bind="count">0</span>
    ...
</div>
```

-   **`data-bind="field"`**: Automatically updates text content when the scope state changes (optimistically or via server response).

### 2. Events (`az-on`)

Azumi uses a declarative syntax for event handling. There are two command types:

**Syntax**: `az-on="{trigger} call {action_name} -> {target_selector}"`
**Syntax**: `az-on="{trigger} set {field} = {value}"`

**Examples**:

```html
<!-- Server Action with auto-detected prediction -->
<button az-on="click call increment -> #counter">Increment</button>

<!-- Form Submission -->
<form az-on="submit call login -> #auth-box">...</form>

<!-- Client-Side UI State (az-ui) -->
<button az-on="click set active_tab = 'rust'">Rust</button>
```

The `call` command sends a POST request to the server action endpoint and morphs the response into the target element.

The `set` command mutates `az-ui` state locally without a server round-trip.

### 3. Optimistic UI (`az-predictions`)

Predictions are **auto-detected** by the `#[azumi::live_impl]` macro and injected as JSON into the scope div via the `az-predictions` attribute. The client reads this attribute and executes predictions automatically when buttons are clicked.

**How it works**:

```html
<!-- Server-rendered HTML -->
<div az-scope='{"count": 0}' az-struct="CounterState"
     az-predictions='[["increment","count = count + 1"],["toggle","active = !active"]]'>
    <span data-bind="count">0</span>
    <button az-on="click call increment">+1</button>
    <button az-on="click call toggle">Toggle</button>
</div>
```

When the user clicks the "+1" button:
1. Client parses `az-predictions` JSON
2. Looks up "increment" → finds `"count = count + 1"`
3. Executes prediction instantly (0ms latency)
4. Sends request to server with original signed state

**Manual override with `data-predict`**:

For custom predictions or when auto-detection isn't sufficient, add `data-predict` to the element. Manual `data-predict` takes precedence over auto-detected predictions.

```html
<button az-on="click call reset" data-predict="count = 0">Reset</button>
```

**Supported prediction DSL**:

| Prediction | Effect |
| :--------- | :----- |
| `field = !field` | Toggle boolean |
| `field = true` | Set to literal |
| `field = field + 1` | Increment |
| `field = field - 1` | Decrement |
| `field = value` | Assignment |

### 4. Client-Side UI State (`az-ui`)

For ephemeral UI state that doesn't need to persist or round-trip to the server (tabs, accordions, toggles), use the `az-ui` attribute with the `set` command.

**Key differences**:

| Attribute | Purpose | Server Round-Trip? | Survives Refresh? |
| :-------- | :------ | :----------------- | :--------------- |
| `az-scope` | Server data (signed, HMAC-protected) | ✅ Yes | ✅ Yes (re-renders) |
| `az-ui` | UI chrome (client-only) | ❌ No | ❌ No (ephemeral) |

**Example**:

```html
<div az-ui='{"active_tab": "rust", "is_open": false}'>
    <button az-on="click set active_tab = 'rust'">Rust</button>
    <button az-on="click set active_tab = 'python'">Python</button>
    <div az-bind:class:active="active_tab == 'rust'">Rust content</div>
    <div az-bind:class:active="active_tab == 'python'">Python content</div>
</div>
```

**How it works**:
1. User clicks a tab button
2. Client finds parent `[az-ui]` element
3. Parses JSON, applies mutation via prediction DSL
4. Writes updated JSON back to `az-ui` attribute
5. Updates bound elements (`az-bind:class`, `az-bind:text`)

**Binding syntax**: `az-bind:class:{classname}="expression"` works with `az-ui` values.

**Supported Expressions**: The expression evaluator supports field lookup (`active_tab`), equality (`== 'rust'`), inequality (`!= 'val'`), negation (`!is_open`), truthy checks, ternary expressions (`field ? 'a' : 'b'`), and compound operators (`&&`, `||`).

**Preserved across morphs**: `az-ui` state survives Idiomorph DOM morphing (e.g., when a sibling server action completes).

### 5. Server Protocol

If you are using `azumi.js` without `DraconDev/azumi-live-ssr-framework`, your server must implement the following:

1.  **Endpoint**: `POST /_azumi/action/{StructName}/{MethodName}` (namespaced)
2.  **Request Body**:
    -   **Forms**: `JSON` object of form fields with `_azumi_scope` key.
    -   **Others**: `JSON` object of the current `az-scope` state.
3.  **Response**:
    -   **Success (200)**: HTML fragment to morph into the target.
    -   **Error (4xx/5xx)**: Client rolls back optimistic updates.

### 6. Hot Reload

The client automatically attempts to connect to `ws://{host}/_azumi/live_reload`.
When the connection is lost (server restart), it polls the current page via `HEAD` requests and refreshes when the server is back up.

## JavaScript API

While declarative attributes cover 90% of use cases, you can access the runtime via `window.azumi`.

```javascript
// Manually execute an action and morph response into target
window.azumi.execute(
    {
        type: "call",
        actionName: "refresh",
        url: "/_azumi/action/refresh",
        target: "#my-component",
    },
    elementReference
);
```
