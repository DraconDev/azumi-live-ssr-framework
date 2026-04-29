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

    // Execute: "call toggle_like -> #box"
    async execute(action, element) {
        if (action.type === "call") {
            await this.callAction(action, element);
        }
    }

    /**
     * Azumi Live: Execute optimistic prediction
     *
     * Predictions are stored in `this.scopes` WeakMap (ephemeral, in-memory).
     * The `az-scope` attribute remains immutable (server-signed) after initial render.
     * Predictions do NOT modify `az-scope` — they live only in JS memory.
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
            let jsonStr = scopeAttr;
            if (scopeAttr.includes("|")) {
                const lastPipe = scopeAttr.lastIndexOf("|");
                jsonStr = scopeAttr.substring(0, lastPipe);
            }

            const state = JSON.parse(jsonStr);
            const originalState = JSON.parse(jsonStr);

            const predictions = prediction
                .split(";")
                .map((p) => p.trim())
                .filter((p) => p);

            for (const pred of predictions) {
                this.applyPrediction(state, pred);
            }

            this.scopes.set(scopeElement, state);
            this.updateBindings(scopeElement);

            console.log("🚀 Prediction executed:", prediction, state);

            return {
                originalState,
                newState: state,
            };
        } catch (err) {
            console.warn("Prediction execution failed:", err);
            return null;
        }
    }

    /**
     * Apply a single prediction to state
     * Format: "field = expression"
     */
    applyPrediction(state, pred) {
        // Parse: "field = expr"
        const match = pred.match(/^(\w+)\s*=\s*(.+)$/);
        if (!match) return;

        const [, field, expr] = match;
        const trimmedExpr = expr.trim();

        // Toggle: "!field"
        if (trimmedExpr.startsWith("!")) {
            const toggleField = trimmedExpr.slice(1).trim();
            if (toggleField === field) {
                state[field] = !state[field];
                return;
            }
        }

        // Increment: "field + value"
        const addMatch = trimmedExpr.match(/^(\w+)\s*\+\s*(\d+)$/);
        if (addMatch && addMatch[1] === field) {
            state[field] = (state[field] || 0) + parseInt(addMatch[2], 10);
            return;
        }

        // Decrement: "field - value"
        const subMatch = trimmedExpr.match(/^(\w+)\s*-\s*(\d+)$/);
        if (subMatch && subMatch[1] === field) {
            state[field] = (state[field] || 0) - parseInt(subMatch[2], 10);
            return;
        }

        // Literal assignment
        if (trimmedExpr === "true") {
            state[field] = true;
        } else if (trimmedExpr === "false") {
            state[field] = false;
        } else if (/^-?\d+$/.test(trimmedExpr)) {
            state[field] = parseInt(trimmedExpr, 10);
        } else if (/^-?\d+\.\d+$/.test(trimmedExpr)) {
            state[field] = parseFloat(trimmedExpr);
        } else if (trimmedExpr.startsWith('"') && trimmedExpr.endsWith('"')) {
            state[field] = trimmedExpr.slice(1, -1).replace(/\\(["\\])/g, '$1');
        } else {
            // Fallback: treat as string
            state[field] = trimmedExpr;
        }
    }

    /**
     * Update DOM elements that display state values
     * Reads from WeakMap first (ephemeral predictions), falls back to az-scope (server state)
     */
    updateBindings(scopeElement) {
        const state = this.scopes.get(scopeElement) || (() => {
            const scopeAttr = scopeElement.getAttribute("az-scope");
            if (!scopeAttr) return null;
            let jsonStr = scopeAttr;
            if (scopeAttr.includes("|")) {
                jsonStr = scopeAttr.substring(0, scopeAttr.lastIndexOf("|"));
            }
            try { return JSON.parse(jsonStr); } catch { return null; }
        })();

        if (!state) return;

        const bindings = scopeElement.querySelectorAll("[data-bind]");
        bindings.forEach((el) => {
            const field = el.getAttribute("data-bind");
            if (!field) return;

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
     * Clears the WeakMap entry — az-scope remains untouched (server-signed)
     */
    rollbackPrediction(scopeElement, originalState) {
        if (!scopeElement) return;

        if (originalState) {
            this.scopes.set(scopeElement, originalState);
            this.updateBindings(scopeElement);
        } else {
            this.scopes.delete(scopeElement);
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
                console.warn(
                    "🚫 Action ignored: Request already pending for this component."
                );
                return;
            }
            scopeElement._azumi_pending = true;
        }

        // IMPORTANT: Capture state BEFORE prediction
        // We must send the original, signed state to the server.
        // If we predict first, we might dirty the state or invalidly sign it.
        let body = null;
        if (element.tagName === "FORM") {
            // For forms, we send the form data, not the state
            body = new FormData(element);
            const data = Object.fromEntries(body.entries());
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
                    predictionResult.originalState
                );
            }
        } finally {
            if (scopeElement) {
                scopeElement._azumi_pending = false;
            }
        }
    }
}

// Initialize
window.azumi = new Azumi();
console.log("Azumi Live Client Initialized 🚀");
