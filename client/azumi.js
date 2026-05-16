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
        this.debug = false; // Enable with window.azumi.debug = true
        this.revealObserver = null; // IntersectionObserver for az-reveal
        this.delegate();
        this.connectHotReload();
        this.init();
    }

    // Auto-init: execute az-init attributes on DOMContentLoaded
    init() {
        const runInit = () => {
            document.querySelectorAll("[az-init]").forEach((el) => {
                const cmd = el.getAttribute("az-init");
                if (!cmd) return;
                this.log("Auto-init:", cmd);
                const action = this.parseAction(cmd, el);
                if (action) this.execute(action, el);
            });
        };

        if (document.readyState === "loading") {
            document.addEventListener("DOMContentLoaded", runInit);
        } else {
            runInit();
        }

        this.setupReveal();
    }

    // Scroll reveal: add data-revealed when elements enter viewport
    setupReveal() {
        const prefersReduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
        if (prefersReduced) {
            document.querySelectorAll('[az-reveal]').forEach((el) => el.setAttribute('data-revealed', ''));
            return;
        }

        if ('IntersectionObserver' in window) {
            this.revealObserver = new IntersectionObserver((entries) => {
                entries.forEach((entry) => {
                    if (entry.isIntersecting) {
                        entry.target.setAttribute('data-revealed', '');
                        this.revealObserver.unobserve(entry.target);
                    }
                });
            }, { threshold: 0.1 });
            this.observeRevealElements();
        } else {
            const revealAll = () => {
                document.querySelectorAll('[az-reveal]:not([data-revealed])').forEach((el) => {
                    const rect = el.getBoundingClientRect();
                    if (rect.top < window.innerHeight * 0.92 && rect.bottom > 0) {
                        el.setAttribute('data-revealed', '');
                    }
                });
                if (document.querySelectorAll('[az-reveal]:not([data-revealed])').length === 0) {
                    window.removeEventListener('scroll', revealAll);
                    window.removeEventListener('resize', revealAll);
                }
            };
            window.addEventListener('scroll', revealAll, { passive: true });
            window.addEventListener('resize', revealAll, { passive: true });
            revealAll();
        }
    }

    // Observe all current az-reveal elements (call after DOM morphing to pick up new ones)
    observeRevealElements() {
        if (!this.revealObserver) return;
        document.querySelectorAll('[az-reveal]:not([data-revealed])').forEach((el) => {
            this.revealObserver.observe(el);
        });
    }

    log(...args) {
        if (this.debug) console.log("[Azumi]", ...args);
    }

    warn(...args) {
        if (this.debug) console.warn("[Azumi]", ...args);
    }

    error(...args) {
        if (this.debug) console.error("[Azumi]", ...args);
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
                this.log("Hot Reload: Connected");
            };

            ws.onmessage = (event) => {
                try {
                    const msg = JSON.parse(event.data);
                    if (msg.type === "style-update") {
                        this.handleStyleUpdate(msg);
                    } else if (msg.type === "reload") {
                        this.log("Hot Reload: Template updated, refreshing...");
                        window.location.reload();
                    }
                } catch (e) {
                    // Not a JSON message or malformed
                }
            };

            ws.onclose = () => {
                if (connected) {
                    this.log("Hot Reload: Connection lost, polling for restart...");
                    this.pollForReload();
                }
            };
        } catch (e) {
            // Hot reload likely not enabled on server
        }
    }

    pollForReload() {
        let attempts = 0;
        const maxAttempts = 150; // 30 seconds at 200ms intervals
        const interval = setInterval(() => {
            attempts++;
            if (attempts >= maxAttempts) {
                clearInterval(interval);
                this.warn("Hot reload: max polling attempts reached, giving up.");
                return;
            }
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
        if (typeof css !== 'string' || typeof scopeId !== 'string') {
            this.warn('Invalid style update message: missing css or scopeId');
            return;
        }
        if (css.length > 100000) {
            this.warn('Style update rejected: CSS too large');
            return;
        }
        const styleTag = document.querySelector(
            `style[data-azumi-scope="${scopeId}"]`
        );
        if (styleTag) {
            styleTag.textContent = css;
            this.log(`Style updated for scope: ${scopeId}`);
        } else {
            this.warn(`Style tag not found for scope: ${scopeId}`);
        }
    }

    // Event delegation
    delegate() {
        ["click", "submit", "change", "input"].forEach((event) => {
            document.addEventListener(event, (e) => this.handleEvent(e));
        });

        // Dedicated form submission handler for az-action forms
        // Only fires if the event hasn't already been handled by handleEvent
        document.addEventListener('submit', (e) => {
            if (e.defaultPrevented) return;
            this.handleFormSubmit(e);
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

        // Check for confirmation dialog
        const confirmMsg = target.getAttribute("az-confirm");
        if (confirmMsg && !window.confirm(confirmMsg)) {
            return; // User cancelled
        }

        // Parse the rest: "call toggle_like -> #box"
        // This is a very basic parser for the prototype
        const action = this.parseAction(parts.slice(1).join(" "), target);
        if (!action) return; // Malformed action, don't prevent default

        // Only prevent default for click and submit events AFTER validating action
        if (e.type === "click" || e.type === "submit") {
            e.preventDefault();
        }

        this.execute(action, target);
    }

    // Handle form submissions with az-action attribute (simpler than az-on)
    handleFormSubmit(e) {
        const form = e.target.closest('form[az-action]');
        if (!form) return;

        e.preventDefault();

        // Check for confirmation dialog
        const confirmMsg = form.getAttribute("az-confirm");
        if (confirmMsg && !window.confirm(confirmMsg)) {
            return; // User cancelled
        }

        const actionName = form.getAttribute('az-action');
        const targetSelector = form.getAttribute('az-target');
        const swap = form.getAttribute('az-swap') || 'morph';

        if (!actionName) return;

        // NAMESPACING: Find parent scope to get struct name
        let namespace = "";
        const scopeEl = form.closest("[az-scope]");
        if (scopeEl) {
            const structName = scopeEl.getAttribute("az-struct");
            if (structName) {
                namespace = `/${structName}`;
            }
        }

        const action = {
            type: "call",
            actionName,
            url: `/_azumi/action${namespace}/${actionName}`,
            target: targetSelector,
            swap,
        };

        this.execute(action, form);
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

        // 'set' command: mutate az-ui state (client-side only, no server round-trip)
        // Format: "set {field} = {value}" e.g. "set count = count + 1" or "set active_tab = 'rust'"
        if (actionType === "set") {
            // Reconstruct the assignment expression from remaining tokens
            const rest = tokens.slice(1).join(" ");
            const setMatch = rest.match(/^([\w.]+)\s*=\s*(.+)$/);
            if (setMatch) {
                const field = setMatch[1].trim();
                const rawValue = setMatch[2].trim();
                return {
                    type: "set",
                    field: field,
                    rawValue: rawValue,
                };
            }
            this.warn("Invalid 'set' command format:", cmd);
            return null;
        }

        // 'scroll-top' command: smooth scroll to top
        if (actionType === "scroll-top") {
            return { type: "scroll-top" };
        }

        return null;
    }

    // Execute: "call toggle_like -> #box" or "set active_tab = 'rust'" or "scroll-top"
    async execute(action, element) {
        if (action.type === "call") {
            await this.callAction(action, element);
        } else if (action.type === "set") {
            this.executeLocalState(action, element);
        } else if (action.type === "scroll-top") {
            window.scrollTo({ top: 0, behavior: 'smooth' });
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

            this.log("Prediction executed:", prediction, state);

            return {
                originalState,
                newState: state,
            };
        } catch (err) {
            this.warn("Prediction execution failed:", err);
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
            this.warn("Blocked prototype-polluting path:", fieldPath);
            return;
        }

        // Helper: get nested property
        const getNested = (obj, path) =>
            path.reduce((o, k) => (o != null ? o[k] : undefined), obj);
        // Helper: set nested property (auto-creates intermediate objects)
        const setNested = (obj, path, value) => {
            const last = path[path.length - 1];
            const target = path.slice(0, -1).reduce((o, k) => {
                if (o == null) return undefined;
                if (o[k] == null) o[k] = {};
                return o[k];
            }, obj);
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
        const addMatch = trimmedExpr.match(/^([\w.]+)\s*\+\s*(\d+(?:\.\d+)?)$/);
        if (addMatch && addMatch[1] === fieldPath) {
            setNested(state, pathParts, (parseFloat(currentVal) || 0) + parseFloat(addMatch[2]));
            return;
        }

        // Decrement: "field - value"
        const subMatch = trimmedExpr.match(/^([\w.]+)\s*-\s*(\d+(?:\.\d+)?)$/);
        if (subMatch && subMatch[1] === fieldPath) {
            setNested(state, pathParts, (parseFloat(currentVal) || 0) - parseFloat(subMatch[2]));
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
     * Execute a 'set' action on az-ui state (client-side only, no server round-trip)
     * Finds the closest [az-ui] element, applies the state mutation, and updates bindings.
     */
    executeLocalState(action, element) {
        // Find the closest az-ui element
        const uiElement = element.closest("[az-ui]");
        if (!uiElement) {
            this.warn("'set' command requires a parent [az-ui] element");
            return;
        }

        // Parse az-ui JSON
        const uiAttr = uiElement.getAttribute("az-ui");
        if (!uiAttr) {
            this.warn("az-ui attribute is empty");
            return;
        }

        let state;
        try {
            state = JSON.parse(uiAttr);
        } catch (err) {
            this.warn("Failed to parse az-ui JSON:", err);
            return;
        }

        // Construct prediction string from action
        const prediction = `${action.field} = ${action.rawValue}`;

        // Apply the prediction to state (reuses existing logic)
        this.applyPrediction(state, prediction);

        // Write back to az-ui attribute
        uiElement.setAttribute("az-ui", JSON.stringify(state));

        this.log("az-ui state updated:", action.field, "=", action.rawValue, "->", state);

        // Update all bindings within this az-ui scope
        this.updateBindings(uiElement);
    }

    /**
     * Read state from az-ui (client state) or az-scope (server state)
     * Priority: WeakMap (ephemeral predictions) -> az-ui -> az-scope
     */
    readState(scopeElement) {
        // Priority 1: WeakMap (optimistic predictions)
        const weakState = this.scopes.get(scopeElement);
        if (weakState) return weakState;

        // Priority 2: az-ui (client-side state from 'set' command)
        const uiAttr = scopeElement.getAttribute("az-ui");
        if (uiAttr) {
            try { return JSON.parse(uiAttr); } catch { /* fall through */ }
        }

        // Priority 3: az-scope (server state)
        const scopeAttr = scopeElement.getAttribute("az-scope");
        if (!scopeAttr) return null;
        let jsonStr = scopeAttr;
        if (scopeAttr.includes("|")) {
            jsonStr = scopeAttr.substring(0, scopeAttr.lastIndexOf("|"));
        }
        try { return JSON.parse(jsonStr); } catch { return null; }
    }

    /**
     * Safely evaluate a predicate expression against state (returns boolean)
     * Supports: !field, field == 'val', field != 'val', field < N, field > N,
     *           field <= N, field >= N, A && B, A || B, field ? 'a' : 'b'
     */
    evaluatePredicate(expr, state) {
        if (!expr || !state) return false;
        expr = expr.trim();

        if (expr.startsWith("(") && expr.endsWith(")")) {
            return this.evaluatePredicate(expr.slice(1, -1), state);
        }

        if (expr.startsWith("!")) {
            const field = expr.slice(1).trim();
            return !this.evaluatePredicate(field, state);
        }

        // Compound AND: A && B
        const andIdx = this.findOperatorIndex(expr, "&&");
        if (andIdx !== -1) {
            const left = expr.slice(0, andIdx).trim();
            const right = expr.slice(andIdx + 2).trim();
            return this.evaluatePredicate(left, state) && this.evaluatePredicate(right, state);
        }

        // Compound OR: A || B
        const orIdx = this.findOperatorIndex(expr, "||");
        if (orIdx !== -1) {
            const left = expr.slice(0, orIdx).trim();
            const right = expr.slice(orIdx + 2).trim();
            return this.evaluatePredicate(left, state) || this.evaluatePredicate(right, state);
        }

        // Ternary: field ? 'a' : 'b'
        const ternaryIdx = this.findTernaryIndex(expr);
        if (ternaryIdx !== -1) {
            const ternary = this.parseTernary(expr);
            if (ternary) {
                const cond = this.evaluatePredicate(ternary.cond, state);
                const result = cond ? ternary.truthy : ternary.falsy;
                return !!this.evaluateExpression(result, state);
            }
        }

// Helper: get nested property value
        const getNestedValue = (obj, path) =>
            path.reduce((o, k) => (o != null ? o[k] : undefined), obj);

        // Numeric comparisons: field < N, field > N, field <= N, field >= N
        const numMatch = expr.match(/^([\w.]+)\s*(<|>|<=|>=)\s*(\d+(?:\.\d+)?)$/);
        if (numMatch) {
            const fieldPath = numMatch[1];
            const op = numMatch[2];
            const limit = parseFloat(numMatch[3]);
            const val = parseFloat(getNestedValue(state, fieldPath.split('.')) || 0);
            switch (op) {
                case '<': return val < limit;
                case '>': return val > limit;
                case '<=': return val <= limit;
                case '>=': return val >= limit;
            }
        }

        // Equality: field == 'value' or field == "value"
        const eqMatch = expr.match(/^([\w.]+)\s*==\s*['"]([^'"]*)['"]$/);
        if (eqMatch) {
            return getNestedValue(state, eqMatch[1].split('.')) === eqMatch[2];
        }

        // Inequality: field != 'value' or field != "value"
        const neqMatch = expr.match(/^([\w.]+)\s*!=\s*['"]([^'"]*)['"]$/);
        if (neqMatch) {
            return getNestedValue(state, neqMatch[1].split('.')) !== neqMatch[2];
        }

        // Simple field name: truthy check
        return !!getNestedValue(state, [expr]);
    }

    /**
     * Find the outermost ternary ? at depth 0, respecting nested ternaries
     * Returns { cond, truthy, falsy } or null
     */
    parseTernary(expr) {
        let questionIdx = -1;
        let colonIdx = -1;
        let inString = false;
        let stringChar = '';
        let depth = 0;
        let isEscaped = false;
        let colonBalance = 0;

        for (let i = 0; i < expr.length; i++) {
            const ch = expr[i];

            if (isEscaped) {
                isEscaped = false;
                continue;
            }

            if (ch === '\\') {
                isEscaped = true;
                continue;
            }

            if (inString) {
                if (ch === stringChar) {
                    inString = false;
                }
                continue;
            }

            if (ch === '"' || ch === "'") {
                inString = true;
                stringChar = ch;
                continue;
            }

            if (ch === '(' || ch === '[' || ch === '{') {
                depth++;
            } else if (ch === ')' || ch === ']' || ch === '}') {
                depth--;
            } else if (ch === '?' && depth === 0) {
                if (questionIdx === -1) {
                    questionIdx = i;
                } else {
                    colonBalance++;
                }
            } else if (ch === ':' && depth === 0) {
                if (colonBalance > 0) {
                    colonBalance--;
                } else if (colonIdx === -1) {
                    colonIdx = i;
                    break;
                }
            }
        }

        if (questionIdx === -1 || colonIdx === -1) return null;

        return {
            cond: expr.slice(0, questionIdx).trim(),
            truthy: expr.slice(questionIdx + 1, colonIdx).trim(),
            falsy: expr.slice(colonIdx + 1).trim()
        };
    }

    findTernaryIndex(expr) {
        let inString = false;
        let stringChar = '';
        let depth = 0;
        let isEscaped = false;

        for (let i = expr.length - 1; i >= 0; i--) {
            const ch = expr[i];

            if (isEscaped) {
                isEscaped = false;
                continue;
            }

            if (ch === '\\') {
                isEscaped = true;
                continue;
            }

            if (inString) {
                if (ch === stringChar) {
                    inString = false;
                }
                continue;
            }

            if (ch === '"' || ch === "'") {
                inString = true;
                stringChar = ch;
                continue;
            }

            if (ch === '(' || ch === '[' || ch === '{') depth--;
            if (ch === ')' || ch === ']' || ch === '}') depth++;

            if (depth === 0 && ch === '?') {
                return i;
            }
        }
        return -1;
    }

    /**
     * Find index of an operator in expression, respecting string literals and nesting
     * Returns -1 if not found or inside quotes
     */
    findOperatorIndex(expr, op) {
        let inString = false;
        let stringChar = '';
        let depth = 0;
        let isEscaped = false;

        for (let i = expr.length - 1; i >= 0; i--) {
            const ch = expr[i];

            if (isEscaped) {
                isEscaped = false;
                continue;
            }
            if (ch === '\\') {
                isEscaped = true;
                continue;
            }

            if (inString) {
                if (ch === stringChar) {
                    inString = false;
                }
                continue;
            }

            if (ch === '"' || ch === "'") {
                inString = true;
                stringChar = ch;
                continue;
            }

            if (ch === '(' || ch === '[' || ch === '{') depth--;
            if (ch === ')' || ch === ']' || ch === '}') depth++;

            if (depth === 0 && ch === op[0]) {
                if (op.length === 1 || expr.slice(i, i + op.length) === op) {
                    return i;
                }
            }
        }
        return -1;
    }

    /**
     * Safely evaluate an expression against state (returns any value)
     * Supports: 'literal', "literal", field, field + N, field - N,
     *           field ? 'a' : 'b'
     */
    evaluateExpression(expr, state) {
        if (!expr || !state) return '';
        expr = expr.trim();

        // Helper: get nested property value
        const getNestedValue = (obj, path) =>
            path.reduce((o, k) => (o != null ? o[k] : undefined), obj);

        // Helper: check if nested path exists (distinguishes missing from undefined)
        const hasNestedPath = (obj, path) => {
            let current = obj;
            for (const key of path) {
                if (current == null || !(key in current)) return false;
                current = current[key];
            }
            return true;
        };

        // Empty
        if (expr === '') return '';

        // Paren grouping
        if (expr.startsWith("(") && expr.endsWith(")")) {
            return this.evaluateExpression(expr.slice(1, -1), state);
        }

        // String literal: '...' or "..."
        if ((expr.startsWith("'") && expr.endsWith("'")) ||
            (expr.startsWith('"') && expr.endsWith('"'))) {
            return expr.slice(1, -1).replace(/\\(['"\\])/g, '$1');
        }

        // Ternary: field ? 'a' : 'b'
        const ternaryIdx = this.findTernaryIndex(expr);
        if (ternaryIdx !== -1) {
            const ternary = this.parseTernary(expr);
            if (ternary) {
                const condVal = this.evaluatePredicate(ternary.cond, state);
                return condVal
                    ? this.evaluateExpression(ternary.truthy, state)
                    : this.evaluateExpression(ternary.falsy, state);
            }
        }

        // OR: field || 'default'
        const orIdx = this.findOperatorIndex(expr, "||");
        if (orIdx !== -1) {
            const field = expr.slice(0, orIdx).trim();
            const defaultVal = expr.slice(orIdx + 2).trim();
            const fieldVal = this.evaluateExpression(field, state);
            return fieldVal !== null && fieldVal !== undefined && fieldVal !== ''
                ? fieldVal
                : this.evaluateExpression(defaultVal, state);
        }

        // Increment: field + N
        const incMatch = expr.match(/^([\w.]+)\s*\+\s*(\d+(?:\.\d+)?)$/);
        if (incMatch) {
            const fieldPath = incMatch[1].split('.');
            return (parseFloat(getNestedValue(state, fieldPath)) || 0) + parseFloat(incMatch[2]);
        }

        // Decrement: field - N
        const decMatch = expr.match(/^([\w.]+)\s*-\s*(\d+(?:\.\d+)?)$/);
        if (decMatch) {
            const fieldPath = decMatch[1].split('.');
            return (parseFloat(getNestedValue(state, fieldPath)) || 0) - parseFloat(decMatch[2]);
        }

        // Field lookup (supports nested paths)
        const val = getNestedValue(state, expr.split('.'));
        if (val !== undefined || hasNestedPath(state, expr.split('.'))) {
            return val;
        }

        // Number literal
        if (/^-?\d+$/.test(expr)) {
            return parseInt(expr, 10);
        }
        if (/^-?\d+\.\d+$/.test(expr)) {
            return parseFloat(expr);
        }

        // Boolean literals
        if (expr === 'true') return true;
        if (expr === 'false') return false;
        if (expr === 'null') return null;

        return expr; // Fallback: return as-is
    }

    /**
     * Update DOM elements that display state values
     * Handles: data-bind, az-bind:text, az-bind:class:*, az-bind:class.*
     */
    updateBindings(scopeElement) {
        const state = this.readState(scopeElement);
        if (!state) return;

        // 1. Legacy data-bind support (textContent only)
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
            } else if (Object.prototype.hasOwnProperty.call(state, field)) {
                el.textContent = state[field];
            }
        });

        // 2. az-bind:text support (textContent with expression evaluation)
        const textBindings = scopeElement.querySelectorAll("[az-bind\\:text]");
        textBindings.forEach((el) => {
            const expr = el.getAttribute("az-bind:text");
            if (!expr) return;
            const result = this.evaluateExpression(expr, state);
            el.textContent = result !== undefined ? result : '';
        });

        // 3. az-bind:class:* support (colon syntax, e.g. az-bind:class:active="expr")
        const classColonBindings = scopeElement.querySelectorAll("[az-bind\\:class\\:active]");
        classColonBindings.forEach((el) => {
            const expr = el.getAttribute("az-bind:class:active");
            const shouldAdd = this.evaluatePredicate(expr, state);
            el.classList.toggle("active", shouldAdd);
        });

        // Generic: handle all az-bind:class:* attributes dynamically
        if (!scopeElement.getAttributeNames) return;

        // Query all elements with az-bind:class:* pattern
        const allClassBindings = scopeElement.querySelectorAll("[az-bind\\:class]");
        allClassBindings.forEach((el) => {
            el.getAttributeNames().forEach((attrName) => {
                if (attrName.startsWith("az-bind:class:")) {
                    const className = attrName.slice("az-bind:class:".length);
                    const expr = el.getAttribute(attrName);
                    const shouldAdd = this.evaluatePredicate(expr, state);
                    el.classList.toggle(className, shouldAdd);
                }
            });
        });

        // 4. az-bind:class.* support (dot syntax, e.g. az-bind:class.liked="liked")
        const dotClassBindings = scopeElement.querySelectorAll("[az-bind\\.class\\.liked]");
        dotClassBindings.forEach((el) => {
            const expr = el.getAttribute("az-bind.class.liked");
            const shouldAdd = this.evaluatePredicate(expr, state);
            el.classList.toggle("liked", shouldAdd);
        });

        // Generic: handle all az-bind:class.* attributes dynamically
        const allDotBindings = scopeElement.querySelectorAll("[az-bind\\.class]");
        allDotBindings.forEach((el) => {
            el.getAttributeNames().forEach((attrName) => {
                if (attrName.startsWith("az-bind.class.")) {
                    const className = attrName.slice("az-bind.class.".length);
                    const expr = el.getAttribute(attrName);
                    const shouldAdd = this.evaluatePredicate(expr, state);
                    el.classList.toggle(className, shouldAdd);
                }
            });
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
        this.log("Prediction rolled back");
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
            let scopeState = this.scopes.get(scopeElement);
            if (!scopeState) {
                scopeState = {};
                this.scopes.set(scopeElement, scopeState);
            }
            if (scopeState._azumi_pending) {
                // Clear stale locks after 30 seconds (server crash / network hang)
                if (Date.now() - (scopeState._azumi_pending_time || 0) > 30000) {
                    this.warn("Clearing stale action lock (>30s timeout)");
                    scopeState._azumi_pending = false;
                } else {
                    this.warn("Action ignored: Request already pending for this component.");
                    return;
                }
            }
            scopeState._azumi_pending = true;
            scopeState._azumi_pending_time = Date.now();
        }

        // IMPORTANT: Capture state BEFORE prediction
        // We must send the original, signed state to the server.
        // If we predict first, we might dirty the state or invalidly sign it.
        let body = null;
        if (element.tagName === "FORM") {
            // For forms, we send the form data alongside the parent scope's signed state.
            // This allows the server to verify the request context.
            body = new FormData(element);
            const hasFiles = element.querySelector('input[type="file"]') !== null;
            if (!hasFiles) {
                // No file inputs: convert to JSON for simple server handling
                // Use a loop instead of Object.fromEntries to preserve multiple values
                const data = {};
                for (const [key, value] of body.entries()) {
                    if (data.hasOwnProperty(key)) {
                        // Multiple values for same key: convert to array
                        if (!Array.isArray(data[key])) {
                            data[key] = [data[key]];
                        }
                        data[key].push(value);
                    } else {
                        data[key] = value;
                    }
                }
                if (scopeElement) {
                    data._azumi_scope = scopeElement.getAttribute("az-scope") || "";
                }
                body = JSON.stringify(data);
            } else {
                // File inputs present: send FormData directly
                if (scopeElement) {
                    body.append("_azumi_scope", scopeElement.getAttribute("az-scope") || "");
                }
            }
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
        let prediction = element.getAttribute("data-predict");
        let predictionResult = null;

        // Auto-detect prediction from az-predictions if data-predict is not set
        if (!prediction && scopeElement) {
            const predictionsAttr = scopeElement.getAttribute("az-predictions");
            if (predictionsAttr) {
                try {
                    const predictionsMap = JSON.parse(predictionsAttr);
                    // az-on format: "click call increment" or "click call reset"
                    const azOn = element.getAttribute("az-on");
                    if (azOn) {
                        const parts = azOn.split(" ");
                        // Format: "{event} call {method}"
                        if (parts.length >= 3 && parts[1] === "call") {
                            const methodName = parts[2];
                            const found = predictionsMap.find(
                                (p) => p[0] === methodName
                            );
                            if (found) {
                                prediction = found[1];
                                this.log("Auto-detected prediction for", methodName, ":", prediction);
                            }
                        }
                    }
                } catch (e) {
                    this.warn("Failed to parse az-predictions:", e);
                }
            }
        }

        if (prediction && scopeElement) {
            this.log("Executing Optimistic Prediction:", prediction);
            // Execute prediction. This updates the DOM optimistically.
            // But we already captured 'body' (original state) above, so we are safe!
            predictionResult = this.executePrediction(scopeElement, prediction);
        }

        try {
            this.log("Fetching Action:", action.url, "Payload:", body);
            const fetchOptions = {
                method: "POST",
                body, // Sends the ORIGINAL, validly signed state
            };
            // Only set Content-Type for JSON payloads; FormData needs browser-set multipart boundary
            if (!(body instanceof FormData)) {
                fetchOptions.headers = { "Content-Type": "application/json" };
            }
            const res = await fetch(action.url, fetchOptions);

            this.log("Server Response Status:", res.status);

            if (!res.ok) throw new Error(`Action failed: ${res.status}`);

            const html = await res.text();
            this.log("Received HTML length:", html.length);

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
            const targetIsScope = target === scopeElement;
            if (action.target) {
                target = document.querySelector(action.target);
            }

            if (target && window.Idiomorph) {
                // Save local state before morphing (preserves both az-local-state and az-ui)
                const localStateElement = target.querySelector("[az-local-state]");
                const savedLocalState = localStateElement ? localStateElement.getAttribute("az-local-state") : null;
                const savedUiState = target.getAttribute("az-ui") || null;

                // Morph will reconcile prediction with server truth
                // Use outerHTML to replace component wrapper
                const morphed = window.Idiomorph.morph(target, html, {
                    morphStyle: "outerHTML",
                });

                // Update target to the morphed element (original may have been replaced)
                if (morphed && morphed.length > 0) {
                    target = morphed[0];
                    // Also update scopeElement if it was the morphed target
                    if (targetIsScope) {
                        scopeElement = target;
                    }
                }

                // Restore local state after morphing
                if (savedLocalState) {
                    const newLocalStateEl = target.querySelector("[az-local-state]");
                    if (newLocalStateEl) {
                        newLocalStateEl.setAttribute("az-local-state", savedLocalState);
                    }
                }
                // Restore az-ui state after morphing
                if (savedUiState) {
                    target.setAttribute("az-ui", savedUiState);
                }

                // Re-observe any new az-reveal elements added by morphing
                this.observeRevealElements();
            } else if (target) {
                this.warn("Idiomorph not loaded, falling back to outerHTML replacement");
                target.outerHTML = html;
            }
        } catch (err) {
            this.error("Action Call Error:", err);
            // Rollback optimistic update
            if (predictionResult && scopeElement) {
                this.rollbackPrediction(
                    scopeElement,
                    predictionResult.originalState
                );
            }
        } finally {
            if (scopeElement) {
                const scopeState = this.scopes.get(scopeElement);
                if (scopeState) {
                    scopeState._azumi_pending = false;
                }
            }
        }
    }

// Initialize
window.azumi = new Azumi();
window.azumi.log("Azumi Live Client Initialized 🚀");
