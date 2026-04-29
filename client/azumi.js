/**
 * Azumi Live - Client Runtime
 *
 * Features:
 * - Event delegation for az-on attributes
 * - DOM morphing via Idiomorph
 * - Optimistic UI via data-predict attributes (Azumi Live)
 */
class Azumi {
    constructor() {
        this.scopes = new WeakMap(); // Element -> state cache
        this.delegate();
        this.connectHotReload();
    }

    // Hot Reload Logic
    connectHotReload() {
        const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
        const wsUrl = `${protocol}//${window.location.host}/_azumi/live_reload`;

        try {
            const ws = new WebSocket(wsUrl);
            let connected = false;

            ws.onopen = () => {
                connected = true;
                console.log("🔥 Hot Reload: Connected");
            };

            ws.onmessage = (event) => {
                try {
                    const msg = JSON.parse(event.data);
                    if (msg.type === "style-update") {
                        this.handleStyleUpdate(msg);
                    } else if (msg.type === "reload") {
                        console.log("🔥 Hot Reload: Template updated, refreshing...");
                        window.location.reload();
                    }
                } catch (e) {
                    // Not a JSON message or malformed
                }
            };

            ws.onclose = () => {
                if (connected) {
                    console.log(
                        "🔥 Hot Reload: Connection lost, polling for restart..."
                    );
                    this.pollForReload();
                }
            };
        } catch (e) {
            // Hot reload likely not enabled on server
        }
    }

    pollForReload() {
        const interval = setInterval(() => {
            fetch(window.location.href, { method: "HEAD" })
                .then((res) => {
                    if (res.ok) {
                        clearInterval(interval);
                        window.location.reload();
                    }
                })
                .catch(() => {
                    /* keep polling */
                });
        }, 200);
    }

    handleStyleUpdate(msg) {
        const { scopeId, css } = msg;
        const styleTag = document.querySelector(
            `style[data-azumi-scope="${scopeId}"]`
        );
        if (styleTag) {
            styleTag.textContent = css;
            console.log(`✅ Style updated for scope: ${scopeId}`);
        } else {
            console.warn(`⚠️ Style tag not found for scope: ${scopeId}`);
        }
    }

    // Event delegation
    delegate() {
        ["click", "submit", "change", "input"].forEach((event) => {
            document.addEventListener(event, (e) => this.handleEvent(e));
        });
    }

    // Parse az-on attribute
    handleEvent(e) {
        const target = e.target.closest(`[az-on]`);
        if (!target) return;

        // Check if the event type matches the trigger (e.g. "click ...")
        // Simple parsing: "click call foo" or "submit call bar"
        const attr = target.getAttribute("az-on");
        const parts = attr.split(" ");
        const trigger = parts[0];

        if (trigger !== e.type) return;

        e.preventDefault(); // Default prevent for handled events

        // Parse the rest: "call toggle_like -> #box"
        // This is a very basic parser for the prototype
        const action = this.parseAction(parts.slice(1).join(" "), target);
        if (action) this.execute(action, target);
    }

    parseAction(cmd, element) {
        // Format: "{event} call {action} -> {target} {swap}"
        // or "{event} set {key} = {value}"
        // NOTE: TokenStream adds spaces around punctuation, so "-> #id" becomes "- > # id"

        // Remove extra spaces and reconstruct operators
        // Ensure arrow has spaces around it to be a separate token
        cmd = cmd.replace(/\s*-\s*>\s*/g, " -> ");
        // Ensure ID selector has no internal spaces (e.g. "# myid" -> "#myid")
        cmd = cmd.replace(/#\s+/g, "#");

        const tokens = cmd.split(" ").filter((t) => t.trim() !== "");
        const actionType = tokens[0]; // "call" or "set"

        if (actionType === "call") {
            let actionName = tokens[1]; // The actual action function name

            // NAMESPACING: Find parent scope to get struct name
            let namespace = "";
            const scopeEl = element.closest("[az-scope]");
            if (scopeEl) {
                const structName = scopeEl.getAttribute("az-struct");
                if (structName) {
                    namespace = `/${structName}`;
                }
            }

            let url = `/_azumi/action${namespace}/${actionName}`;
            let targetSelector = null;
            let swap = "morph";

            const arrowIndex = tokens.indexOf("->");
            if (arrowIndex !== -1) {
                targetSelector = tokens[arrowIndex + 1];
                if (tokens[arrowIndex + 2]) {
                    swap = tokens[arrowIndex + 2];
                }
            }

            return {
                type: "call",
                actionName,
                url,
                target: targetSelector,
                swap,
            };
        }

        // Deprecated: 'set' command removed
        if (actionType === "set") {
            console.error(
                "Azumi Error: 'set' command is deprecated and removed. Use server actions instead."
            );
            return null;
        }

        return null;
    }

    // Execute: "call toggle_like -> #box" or "set open = true"
    async execute(action, element) {
        if (action.type === "call") {
            await this.callAction(action, element);
        } else if (action.type === "set") {
            this.setState(action, element);
        }
    }

    /**
     * Azumi Live: Execute optimistic prediction
     *
     * Prediction DSL format: "field = expression"
     * Expressions:
     *   - "!field" -> toggle boolean
     *   - "field + value" -> increment
     *   - "field - value" -> decrement
     *   - literal -> direct assignment
     *
     * Multiple predictions separated by ";"
     */
    executePrediction(scopeElement, prediction) {
        if (!prediction || !scopeElement) return null;

        const scopeAttr = scopeElement.getAttribute("az-scope");
        if (!scopeAttr) return null;

        try {
            // Handle signed state: "{json}|{signature}"
            let jsonStr = scopeAttr;
            if (scopeAttr.includes("|")) {
                const parts = scopeAttr.split("|");
                // JSON is the part before the last pipe (to handle pipes in JSON strings, though rare)
                // Actually, Azumi security uses "last pipe" logic.
                const lastPipe = scopeAttr.lastIndexOf("|");
                jsonStr = scopeAttr.substring(0, lastPipe);
            }

            const state = JSON.parse(jsonStr);
            const originalState = JSON.parse(jsonStr); // Keep copy for rollback

            // Parse multiple predictions separated by ;
            const predictions = prediction
                .split(";")
                .map((p) => p.trim())
                .filter((p) => p);

            for (const pred of predictions) {
                this.applyPrediction(state, pred);
            }

            // Update the scope attribute with new state
            scopeElement.setAttribute("az-scope", JSON.stringify(state));

            // Update any bound elements
            this.updateBindings(scopeElement, state);

            console.log("🚀 Prediction executed:", prediction, state);

            return {
                originalState,
                newState: state,
                originalScopeAttr: scopeAttr,
            };
        } catch (err) {
            console.warn("Prediction execution failed:", err);
            return null;
        }
    }

    /**
     * Apply a single prediction to state
     * Format: "field = expression" or "field.sub.path = expression"
     */
    applyPrediction(state, pred) {
        // Parse: "field = expr" (supports nested paths like "user.count")
        const match = pred.match(/^([\w.]+)\s*=\s*(.+)$/);
        if (!match) return;

        const [, fieldPath, expr] = match;
        const trimmedExpr = expr.trim();
        const pathParts = fieldPath.split(".");

        // Guard against prototype pollution: reject dangerous path segments
        const dangerous = ["__proto__", "constructor", "prototype", "prototype__", "__defineGetter__", "__defineSetter__", "hasOwnProperty", "isPrototypeOf", "propertyIsEnumerable", "toLocaleString", "toString", "valueOf", "__lookupGetter__", "__lookupSetter__"];
        if (pathParts.some(p => dangerous.includes(p))) {
            console.warn("Azumi:Blocked prototype-polluting path:", fieldPath);
            return;
        }

        // Helper: get nested property
        const getNested = (obj, path) =>
            path.reduce((o, k) => (o != null ? o[k] : undefined), obj);
        // Helper: set nested property
        const setNested = (obj, path, value) => {
            const last = path[path.length - 1];
            const target = path.slice(0, -1).reduce((o, k) => (o != null ? o[k] : undefined), obj);
            if (target != null) target[last] = value;
        };

        const currentVal = getNested(state, pathParts);

        // Toggle: "!field"
        if (trimmedExpr.startsWith("!")) {
            const togglePath = trimmedExpr.slice(1).trim().split(".");
            if (togglePath.join(".") === fieldPath) {
                setNested(state, pathParts, !currentVal);
                return;
            }
        }

        // Increment: "field + value"
        const addMatch = trimmedExpr.match(/^([\w.]+)\s*\+\s*(\d+)$/);
        if (addMatch && addMatch[1] === fieldPath) {
            setNested(state, pathParts, (currentVal || 0) + parseInt(addMatch[2], 10));
            return;
        }

        // Decrement: "field - value"
        const subMatch = trimmedExpr.match(/^([\w.]+)\s*-\s*(\d+)$/);
        if (subMatch && subMatch[1] === fieldPath) {
            setNested(state, pathParts, (currentVal || 0) - parseInt(subMatch[2], 10));
            return;
        }

        // Literal assignment
        if (trimmedExpr === "true") {
            setNested(state, pathParts, true);
        } else if (trimmedExpr === "false") {
            setNested(state, pathParts, false);
        } else if (/^-?\d+$/.test(trimmedExpr)) {
            setNested(state, pathParts, parseInt(trimmedExpr, 10));
        } else if (/^-?\d+\.\d+$/.test(trimmedExpr)) {
            setNested(state, pathParts, parseFloat(trimmedExpr));
        } else if (trimmedExpr.startsWith('"') && trimmedExpr.endsWith('"')) {
            setNested(state, pathParts, trimmedExpr.slice(1, -1).replace(/\\(["\\])/g, '$1'));
        } else {
            // Fallback: treat as string
            setNested(state, pathParts, trimmedExpr);
        }
    }

    /**
     * Update DOM elements that display state values
     * Looks for elements with data-bind="fieldName" or data-bind="user.profile.name" attribute
     */
    updateBindings(scopeElement, state) {
        // Find all elements with data-bind within the scope
        const bindings = scopeElement.querySelectorAll("[data-bind]");
        bindings.forEach((el) => {
            const field = el.getAttribute("data-bind");
            if (!field) return;

            // Support nested paths like "user.profile.name"
            if (field.includes(".")) {
                const parts = field.split(".");
                const val = parts.reduce(
                    (o, k) => (o != null ? o[k] : undefined),
                    state
                );
                if (val !== undefined) el.textContent = val;
            } else if (state.hasOwnProperty(field)) {
                el.textContent = state[field];
            }
        });
    }

    /**
     * Rollback prediction if server response differs
     */
    rollbackPrediction(scopeElement, originalState, originalScopeAttr) {
        if (!scopeElement) return;

        if (originalScopeAttr) {
            scopeElement.setAttribute("az-scope", originalScopeAttr);
        } else if (originalState) {
            scopeElement.setAttribute(
                "az-scope",
                JSON.stringify(originalState)
            );
        }

        if (originalState) {
            this.updateBindings(scopeElement, originalState);
        }
        console.log("⏪ Prediction rolled back");
    }

    // Server action with optimistic prediction
    async callAction(action, element) {
        // Find scope element
        const scopeElement = element.closest("[az-scope]");

        // PREVENT CONCURRENT UPDATES:
        // In a signed-state system, we CANNOT send a second request until we get the
        // signature for the first result. Pipelining is mathematically impossible
        // without client-side signing keys.
        if (scopeElement) {
            if (scopeElement._azumi_pending) {
                // Clear stale locks after 30 seconds (server crash / network hang)
                if (Date.now() - (scopeElement._azumi_pending_time || 0) > 30000) {
                    console.warn("🔓 Clearing stale action lock (>30s timeout)");
                    scopeElement._azumi_pending = false;
                } else {
                    console.warn(
                        "🚫 Action ignored: Request already pending for this component."
                    );
                    return;
                }
            }
            scopeElement._azumi_pending = true;
            scopeElement._azumi_pending_time = Date.now();
        }

        // IMPORTANT: Capture state BEFORE prediction
        // We must send the original, signed state to the server.
        // If we predict first, we might dirty the state or invalidly sign it.
        let body = null;
        if (element.tagName === "FORM") {
            // For forms, we send the form data alongside the parent scope's signed state.
            // This allows the server to verify the request context.
            body = new FormData(element);
            const data = Object.fromEntries(body.entries());
            if (scopeElement) {
                data._azumi_scope = scopeElement.getAttribute("az-scope") || "";
            }
            body = JSON.stringify(data);
        } else {
            if (scopeElement) {
                // Get the raw attribute value (including signature if present)
                let scopeData = scopeElement.getAttribute("az-scope");
                body = scopeData || "{}";
            } else {
                body = "{}";
            }
        }

        // Check for prediction attribute (Azumi Live)
        const prediction = element.getAttribute("data-predict");
        let predictionResult = null;

        if (prediction && scopeElement) {
            console.log("[Azumi] Executing Optimistic Prediction:", prediction);
            // Execute prediction. This updates the DOM optimistically.
            // But we already captured 'body' (original state) above, so we are safe!
            predictionResult = this.executePrediction(scopeElement, prediction);
        }

        try {
            console.log(
                "[Azumi] Fetching Action:",
                action.url,
                "Payload:",
                body
            );
            const res = await fetch(action.url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body, // Sends the ORIGINAL, validly signed state
            });

            console.log("[Azumi] Server Response Status:", res.status);

            if (!res.ok) throw new Error(`Action failed: ${res.status}`);

            const html = await res.text();
            console.log("[Azumi] Received HTML length:", html.length);

            // OPTIMIZATION: Check if server state matches prediction
            // If prediction was correct, skip morphing to prevent flicker
            /* 
            // DISABLED: This optimization prevents structural updates (e.g. @if blocks) from rendering
            if (predictionResult && scopeElement) {
                // ... (omitted for brevity)
            }
            */

            // FIXED: Default target to scopeElement (component root), then element
            let target = scopeElement || element;
            if (action.target) {
                target = document.querySelector(action.target);
            }

            if (target && window.Idiomorph) {
                // Morph will reconcile prediction with server truth
                // Use outerHTML to replace component wrapper
                window.Idiomorph.morph(target, html, {
                    morphStyle: "outerHTML",
                });
            } else if (target) {
                console.warn(
                    "Idiomorph not loaded, falling back to outerHTML replacement"
                );
                target.outerHTML = html;
            }
        } catch (err) {
            console.error("Action Call Error:", err);
            // Rollback optimistic update
            if (predictionResult && scopeElement) {
                this.rollbackPrediction(
                    scopeElement,
                    predictionResult.originalState,
                    predictionResult.originalScopeAttr
                );
            }
        } finally {
            if (scopeElement) {
                scopeElement._azumi_pending = false;
            }
        }
    }

    // Local state change (no server roundtrip)
    setState(action, element) {
        const scopeElement = element.closest("[az-scope]");
        if (!scopeElement) {
            console.warn("setState: No az-scope found");
            return;
        }

        const scopeAttr = scopeElement.getAttribute("az-scope");
        if (!scopeAttr) return;

        try {
            // Handle signed state: "{json}|{signature}"
            let jsonStr = scopeAttr;
            let signature = "";

            if (scopeAttr.includes("|")) {
                const lastPipe = scopeAttr.lastIndexOf("|");
                jsonStr = scopeAttr.substring(0, lastPipe);
                signature = scopeAttr.substring(lastPipe); // Keep signature ("|sig")
            }

            const state = JSON.parse(jsonStr);

            // Apply the prediction DSL (reuse existing logic)
            const prediction = `${action.field} = ${action.value}`;
            this.applyPrediction(state, prediction);

            // Update the scope attribute (preserve signature!)
            // We can't re-sign on client, so we just append the old signature.
            // WARNING: This invalidates the signature technically, but for local-only state it might be fine?
            // Actually, for Server Actions, the server will reject this if we send it back.
            // But setState is for local components or temporary toggles.

            const newStateStr = JSON.stringify(state) + signature;
            scopeElement.setAttribute("az-scope", newStateStr);

            // Update bound elements
            this.updateBindings(scopeElement, state);

            console.log(
                "🎯 Client set:",
                action.field,
                "=",
                action.value,
                state
            );
        } catch (err) {
            console.warn("setState failed:", err);
        }
    }
}

// Initialize
window.azumi = new Azumi();
console.log("Azumi Live Client Initialized 🚀");
