/**
 * Azumi Client JS - Runtime Evaluator Tests
 *
 * Tests evaluatePredicate, evaluateExpression, parseTernary, findTernaryIndex,
 * findOperatorIndex, applyPrediction, and readState using a stub that mirrors
 * the production logic in client/azumi.js exactly.
 *
 * Run with: node tests/azumi_js_tests.js
 */

// Minimal Azumi stub matching client/azumi.js evaluators exactly
class AzumiTest {
    constructor() {
        this.debug = false;
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

    // Shared helper: get nested property value from object by path parts
    getNestedValue(obj, path) {
        return path.reduce((o, k) => (o != null ? o[k] : undefined), obj);
    }

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

        const andIdx = this.findOperatorIndex(expr, "&&");
        if (andIdx !== -1) {
            const left = expr.slice(0, andIdx).trim();
            const right = expr.slice(andIdx + 2).trim();
            return this.evaluatePredicate(left, state) && this.evaluatePredicate(right, state);
        }

        const orIdx = this.findOperatorIndex(expr, "||");
        if (orIdx !== -1) {
            const left = expr.slice(0, orIdx).trim();
            const right = expr.slice(orIdx + 2).trim();
            return this.evaluatePredicate(left, state) || this.evaluatePredicate(right, state);
        }

        const ternaryIdx = this.findTernaryIndex(expr);
        if (ternaryIdx !== -1) {
            const ternary = this.parseTernary(expr);
            if (ternary) {
                const cond = this.evaluatePredicate(ternary.cond, state);
                const result = cond ? ternary.truthy : ternary.falsy;
                return !!this.evaluateExpression(result, state);
            }
        }

        // Numeric comparisons: field < N, field > N, field <= N, field >= N
        const numMatch = expr.match(/^([\w.]+)\s*(<|>|<=|>=)\s*(\d+(?:\.\d+)?)$/);
        if (numMatch) {
            const fieldPath = numMatch[1].split('.');
            const op = numMatch[2];
            const limit = parseFloat(numMatch[3]);
            const val = parseFloat(this.getNestedValue(state, fieldPath) || 0);
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
            return this.getNestedValue(state, eqMatch[1].split('.')) === eqMatch[2];
        }

        // Inequality: field != 'value' or field != "value"
        const neqMatch = expr.match(/^([\w.]+)\s*!=\s*['"]([^'"]*)['"]$/);
        if (neqMatch) {
            return this.getNestedValue(state, neqMatch[1].split('.')) !== neqMatch[2];
        }

        // Simple field name: truthy check (supports nested paths)
        return !!this.getNestedValue(state, [expr]);
    }

    evaluateExpression(expr, state) {
        if (!expr || !state) return '';
        expr = expr.trim();

        if (expr === '') return '';

        if (expr.startsWith("(") && expr.endsWith(")")) {
            return this.evaluateExpression(expr.slice(1, -1), state);
        }

        if ((expr.startsWith("'") && expr.endsWith("'")) ||
            (expr.startsWith('"') && expr.endsWith('"'))) {
            return expr.slice(1, -1).replace(/\\(['"\\])/g, '$1');
        }

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
            return (parseFloat(this.getNestedValue(state, fieldPath)) || 0) + parseFloat(incMatch[2]);
        }

        // Decrement: field - N
        const decMatch = expr.match(/^([\w.]+)\s*-\s*(\d+(?:\.\d+)?)$/);
        if (decMatch) {
            const fieldPath = decMatch[1].split('.');
            return (parseFloat(this.getNestedValue(state, fieldPath)) || 0) - parseFloat(decMatch[2]);
        }

        // Field lookup (supports nested paths)
        const val = this.getNestedValue(state, expr.split('.'));
        if (val !== undefined) {
            return val;
        }

        // Number literals
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

        return expr;
    }

    /**
     * Apply a single prediction to state.
     * Format: "field = expression" or "field.sub.path = expression"
     */
    applyPrediction(state, pred) {
        const match = pred.match(/^([\w.]+)\s*=\s*(.+)$/);
        if (!match) return;

        const [, fieldPath, expr] = match;
        const trimmedExpr = expr.trim();
        const pathParts = fieldPath.split('.');

        const dangerous = [
            "__proto__", "constructor", "prototype", "prototype__",
            "__defineGetter__", "__defineSetter__", "hasOwnProperty",
            "isPrototypeOf", "propertyIsEnumerable", "toLocaleString",
            "toString", "valueOf", "__lookupGetter__", "__lookupSetter__"
        ];
        if (pathParts.some(p => dangerous.includes(p))) {
            this.warn("Blocked prototype-polluting path:", fieldPath);
            return;
        }

        const getNested = (obj, path) =>
            path.reduce((o, k) => (o != null ? o[k] : undefined), obj);
        const setNested = (obj, path, value) => {
            const last = path[path.length - 1];
            const target = path.slice(0, -1).reduce((o, k) => (o != null ? o[k] : undefined), obj);
            if (target != null) target[last] = value;
        };

        const currentVal = getNested(state, pathParts);

        // Toggle: "!field"
        if (trimmedExpr.startsWith("!")) {
            const togglePath = trimmedExpr.slice(1).trim().split('.');
            if (togglePath.join('.') === fieldPath) {
                setNested(state, pathParts, !currentVal);
                return;
            }
        }

        // Increment: "field + N"
        const addMatch = trimmedExpr.match(/^([\w.]+)\s*\+\s*(\d+(?:\.\d+)?)$/);
        if (addMatch && addMatch[1] === fieldPath) {
            setNested(state, pathParts, (parseFloat(currentVal) || 0) + parseFloat(addMatch[2]));
            return;
        }

        // Decrement: "field - N"
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
        } else if (trimmedExpr.startsWith("'") && trimmedExpr.endsWith("'")) {
            setNested(state, pathParts, trimmedExpr.slice(1, -1).replace(/\\(['"\\])/g, '$1'));
        } else {
            setNested(state, pathParts, trimmedExpr);
        }
    }

    readStateFromElement(el) {
        const uiAttr = el.getAttribute("az-ui");
        if (uiAttr) {
            try {
                return JSON.parse(uiAttr);
            } catch { return null; }
        }
        const scopeAttr = el.getAttribute("az-scope");
        if (!scopeAttr) return null;
        let jsonStr = scopeAttr;
        if (scopeAttr.includes("|")) {
            jsonStr = scopeAttr.substring(0, scopeAttr.lastIndexOf("|"));
        }
        try { return JSON.parse(jsonStr); } catch { return null; }
    }
}

// Test utilities
let passed = 0;
let failed = 0;

function assert(condition, message) {
    if (condition) {
        console.log(`  ✅ ${message}`);
        passed++;
    } else {
        console.log(`  ❌ FAIL: ${message}`);
        failed++;
    }
}

function assertEqual(actual, expected, message) {
    if (actual === expected) {
        console.log(`  ✅ ${message}`);
        passed++;
    } else {
        console.log(`  ❌ FAIL: ${message} (expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)})`);
        failed++;
    }
}

function section(name) {
    console.log(`\n${name}`);
    console.log("─".repeat(50));
}

// ─── findOperatorIndex ───────────────────────────────────────────────────────

section("findOperatorIndex");

const az = new AzumiTest();

assertEqual(az.findOperatorIndex("a && b", "&&"), 2, "finds && at index 2");
assertEqual(az.findOperatorIndex("a || b", "||"), 2, "finds || at index 2");
assertEqual(az.findOperatorIndex("a < b", "<"), 2, "finds < at index 2");
assertEqual(az.findOperatorIndex("a > b", ">"), 2, "finds > at index 2");
assertEqual(az.findOperatorIndex("a <= b", "<="), 2, "finds <= at index 2");
assertEqual(az.findOperatorIndex("a >= b", ">="), 2, "finds >= at index 2");
assertEqual(az.findOperatorIndex("a == 'val'", "=="), 2, "finds == at index 2");
assertEqual(az.findOperatorIndex("a != 'val'", "!="), 2, "finds != at index 2");
assertEqual(az.findOperatorIndex("a && b && c", "&&"), 7, "finds rightmost && in chain at index 7");
assertEqual(az.findOperatorIndex("(a && b) || c", "||"), 9, "ignores && inside parens for ||");
assertEqual(az.findOperatorIndex("a < b && c > d", "&&"), 6, "finds && avoiding < and >");
assertEqual(az.findOperatorIndex("no operator here", "&&"), -1, "returns -1 when not found");
assertEqual(az.findOperatorIndex("!flag ? 'a' : 'b'", "!"), 0, "finds ! at index 0 for negation in ternary");

// ─── evaluatePredicate ─────────────────────────────────────────────────────

section("evaluatePredicate");

assertEqual(az.evaluatePredicate("flag", { flag: true }), true, "truthy field → true");
assertEqual(az.evaluatePredicate("flag", { flag: false }), false, "falsy field → false");
assertEqual(az.evaluatePredicate("flag", { flag: "yes" }), true, "non-empty string → truthy");
assertEqual(az.evaluatePredicate("flag", { flag: 0 }), false, "zero → falsy");
assertEqual(az.evaluatePredicate("flag", {}), false, "missing field → false");
assertEqual(az.evaluatePredicate("", {}), false, "empty expression → false");
assertEqual(az.evaluatePredicate(null, {}), false, "null expression → false");

assertEqual(az.evaluatePredicate("!flag", { flag: true }), false, "negation: !true → false");
assertEqual(az.evaluatePredicate("!flag", { flag: false }), true, "negation: !false → true");
assertEqual(az.evaluatePredicate("!!flag", { flag: true }), true, "double negation");
assertEqual(az.evaluatePredicate("!!!flag", { flag: true }), false, "triple negation");

assertEqual(az.evaluatePredicate("count == '5'", { count: "5" }), true, "string equality match");
assertEqual(az.evaluatePredicate("count == '5'", { count: "6" }), false, "string equality no match");
assertEqual(az.evaluatePredicate("count != '5'", { count: "6" }), true, "string inequality match");
assertEqual(az.evaluatePredicate("count != '5'", { count: "5" }), false, "string inequality no match");
assertEqual(az.evaluatePredicate("name == 'Alice'", { name: "Alice" }), true, "string equality with name");
assertEqual(az.evaluatePredicate("name == \"Bob\"", { name: "Bob" }), true, "double-quoted string equality");

assertEqual(az.evaluatePredicate("count < 10", { count: 5 }), true, "less than: 5 < 10");
assertEqual(az.evaluatePredicate("count < 10", { count: 10 }), false, "less than: 10 !< 10");
assertEqual(az.evaluatePredicate("count < 10", { count: 15 }), false, "less than: 15 !< 10");
assertEqual(az.evaluatePredicate("count > 10", { count: 15 }), true, "greater than: 15 > 10");
assertEqual(az.evaluatePredicate("count > 10", { count: 10 }), false, "greater than: 10 !> 10");
assertEqual(az.evaluatePredicate("count > 10", { count: 5 }), false, "greater than: 5 !> 10");
assertEqual(az.evaluatePredicate("count <= 10", { count: 10 }), true, "less or equal: 10 <= 10");
assertEqual(az.evaluatePredicate("count <= 10", { count: 5 }), true, "less or equal: 5 <= 10");
assertEqual(az.evaluatePredicate("count >= 10", { count: 10 }), true, "greater or equal: 10 >= 10");
assertEqual(az.evaluatePredicate("count >= 10", { count: 15 }), true, "greater or equal: 15 >= 10");

assertEqual(az.evaluatePredicate("a && b", { a: true, b: true }), true, "AND both true");
assertEqual(az.evaluatePredicate("a && b", { a: true, b: false }), false, "AND one false");
assertEqual(az.evaluatePredicate("a && b", { a: false, b: true }), false, "AND first false");
assertEqual(az.evaluatePredicate("a && b", { a: false, b: false }), false, "AND both false");
assertEqual(az.evaluatePredicate("a && b && c", { a: true, b: true, c: true }), true, "AND triple chain");
assertEqual(az.evaluatePredicate("a && b && c", { a: true, b: true, c: false }), false, "AND triple last false");

assertEqual(az.evaluatePredicate("a || b", { a: true, b: true }), true, "OR both true");
assertEqual(az.evaluatePredicate("a || b", { a: true, b: false }), true, "OR one true");
assertEqual(az.evaluatePredicate("a || b", { a: false, b: true }), true, "OR second true");
assertEqual(az.evaluatePredicate("a || b", { a: false, b: false }), false, "OR both false");

assertEqual(az.evaluatePredicate("a && b || c", { a: true, b: false, c: true }), true, "AND/OR precedence");
assertEqual(az.evaluatePredicate("a || b && c", { a: false, b: true, c: true }), true, "OR/AND precedence");
assertEqual(az.evaluatePredicate("a || b && c", { a: false, b: false, c: false }), false, "OR/AND all false");

assertEqual(az.evaluatePredicate("flag ? 'yes' : 'no'", { flag: true }), true, "ternary truthy → truthy string result");
assertEqual(az.evaluatePredicate("flag ? 'yes' : 'no'", { flag: false }), true, "ternary falsy → truthy string result (both 'yes' and 'no' are truthy)");
assertEqual(az.evaluatePredicate("count > 5 ? 'high' : 'low'", { count: 10 }), true, "ternary with comparison truthy → truthy string");
assertEqual(az.evaluatePredicate("count > 5 ? 'high' : 'low'", { count: 3 }), true, "ternary with comparison falsy → truthy string (both branches are truthy)");

assertEqual(az.evaluatePredicate("count < 10 && active", { count: 5, active: true }), true, "AND with comparison");
assertEqual(az.evaluatePredicate("count < 10 && active", { count: 5, active: false }), false, "AND with comparison falsy active");
assertEqual(az.evaluatePredicate("count < 10 || active", { count: 15, active: true }), true, "OR with comparison");
assertEqual(az.evaluatePredicate("count < 10 || active", { count: 15, active: false }), false, "OR with comparison all false");

assertEqual(az.evaluatePredicate("count > 0 && count < 100", { count: 50 }), true, "range check with AND");
assertEqual(az.evaluatePredicate("name == 'Alice' && active", { name: "Alice", active: true }), true, "string equality in AND");

// ─── evaluateExpression ───────────────────────────────────────────────────

section("evaluateExpression");

assertEqual(az.evaluateExpression("field", { field: "hello" }), "hello", "field lookup string");
assertEqual(az.evaluateExpression("field", { field: 42 }), 42, "field lookup number");
assertEqual(az.evaluateExpression("field", { field: true }), true, "field lookup boolean");
assertEqual(az.evaluateExpression("field", { field: null }), null, "field lookup null");
assertEqual(az.evaluateExpression("missing", {}), "missing", "missing field returns as-is");

assertEqual(az.evaluateExpression("'hello'", {}), "hello", "single-quoted string literal");
assertEqual(az.evaluateExpression('"hello"', {}), "hello", "double-quoted string literal");
assertEqual(az.evaluateExpression("'it\\'s working'", {}), "it's working", "escaped single quote in string");
assertEqual(az.evaluateExpression("'a\\'b'", {}), "a'b", "escaped quote mid-string");

assertEqual(az.evaluateExpression("42", {}), 42, "integer literal");
assertEqual(az.evaluateExpression("-42", {}), -42, "negative integer literal");
assertEqual(az.evaluateExpression("3.14", {}), 3.14, "float literal");
assertEqual(az.evaluateExpression("0", {}), 0, "zero literal");

assertEqual(az.evaluateExpression("true", {}), true, "boolean true literal");
assertEqual(az.evaluateExpression("false", {}), false, "boolean false literal");
assertEqual(az.evaluateExpression("null", {}), null, "null literal");

assertEqual(az.evaluateExpression("count + 1", { count: 5 }), 6, "field + number");
assertEqual(az.evaluateExpression("count + 1", { count: -1 }), 0, "field + number negative");
assertEqual(az.evaluateExpression("count + 1", {}), 1, "missing field + number → 0 + 1");
assertEqual(az.evaluateExpression("count - 1", { count: 5 }), 4, "field - number");
assertEqual(az.evaluateExpression("count - 1", {}), -1, "missing field - 1 → 0 - 1");

assertEqual(az.evaluateExpression("count > 5 ? 'high' : 'low'", { count: 10 }), "high", "ternary truthy branch");
assertEqual(az.evaluateExpression("count > 5 ? 'high' : 'low'", { count: 3 }), "low", "ternary falsy branch");
assertEqual(az.evaluateExpression("flag ? 'yes' : 'no'", { flag: true }), "yes", "ternary with flag true");
assertEqual(az.evaluateExpression("flag ? 'yes' : 'no'", { flag: false }), "no", "ternary with flag false");
assertEqual(az.evaluateExpression("liked ? 'Unlike' : 'Like'", { liked: true }), "Unlike", "ternary with liked true");
assertEqual(az.evaluateExpression("liked ? 'Unlike' : 'Like'", { liked: false }), "Like", "ternary with liked false");
assertEqual(az.evaluateExpression("section1_open ? '−' : '+'", { section1_open: true }), "−", "ternary with unicode");
assertEqual(az.evaluateExpression("section1_open ? '−' : '+'", { section1_open: false }), "+", "ternary with unicode falsy");

assertEqual(az.evaluateExpression("count", { count: 0 }), 0, "field with zero value");
assertEqual(az.evaluateExpression("count", { count: "0" }), "0", "field with string zero");
assertEqual(az.evaluateExpression("name", { name: "" }), "", "empty string field");
assertEqual(az.evaluateExpression("", {}), "", "empty expression");

assertEqual(az.evaluateExpression("a && b", { a: true, b: true }), "a && b", "AND not supported in evaluateExpression → returns as-is");
assertEqual(az.evaluateExpression("a || b", { a: false, b: true }), false, "|| with false left operand → false (not defaulted)");
assertEqual(az.evaluateExpression("!flag", { flag: true }), "!flag", "NOT not supported in evaluateExpression → returns as-is");

// ─── readStateFromElement ──────────────────────────────────────────────────

section("readStateFromElement");

function makeElement(attrs) {
    if (typeof document !== "undefined") {
        const el = document.createElement("div");
        for (const [k, v] of Object.entries(attrs)) {
            el.setAttribute(k, v);
        }
        return el;
    }
    return { getAttribute: (k) => attrs[k] || null };
}

const uiEl = makeElement({ "az-ui": '{"count":5,"name":"Alice"}' });
assertEqual(az.readStateFromElement(uiEl).count, 5, "reads count from az-ui");
assertEqual(az.readStateFromElement(uiEl).name, "Alice", "reads name from az-ui");

const scopeEl = makeElement({ "az-scope": '{"active":true}' });
assertEqual(az.readStateFromElement(scopeEl).active, true, "reads active from az-scope");

const signedScopeEl = makeElement({ "az-scope": '{"x":1}|abc123sig' });
assertEqual(az.readStateFromElement(signedScopeEl).x, 1, "strips HMAC signature from az-scope");

const emptyEl = makeElement({});
assertEqual(az.readStateFromElement(emptyEl), null, "returns null for element with no state");

const badJsonEl = makeElement({ "az-ui": "not valid json" });
assertEqual(az.readStateFromElement(badJsonEl), null, "returns null for malformed JSON");

// ─── Nested Path Tests ─────────────────────────────────────────────────────

section("Nested path: evaluatePredicate comparisons");

assertEqual(az.evaluatePredicate("user.age > 18", { user: { age: 21 } }), true, "nested: user.age > 18, age=21");
assertEqual(az.evaluatePredicate("user.age > 18", { user: { age: 15 } }), false, "nested: user.age > 18, age=15");
assertEqual(az.evaluatePredicate("user.age >= 21", { user: { age: 21 } }), true, "nested: user.age >= 21, exact");
assertEqual(az.evaluatePredicate("user.age >= 22", { user: { age: 21 } }), false, "nested: user.age >= 22, one less");
assertEqual(az.evaluatePredicate("user.age < 65", { user: { age: 50 } }), true, "nested: user.age < 65, valid");
assertEqual(az.evaluatePredicate("user.age < 30", { user: { age: 35 } }), false, "nested: user.age < 30, too high");
assertEqual(az.evaluatePredicate("user.age <= 100", { user: { age: 100 } }), true, "nested: user.age <= 100, boundary");
assertEqual(az.evaluatePredicate("user.score >= 50", { user: { score: 49.9 } }), false, "nested: user.score >= 50, just under");

assertEqual(az.evaluatePredicate("a.b.c > 5", { a: { b: { c: 10 } } }), true, "deep nested: a.b.c > 5, c=10");
assertEqual(az.evaluatePredicate("a.b.c > 5", { a: { b: { c: 3 } } }), false, "deep nested: a.b.c > 5, c=3");
assertEqual(az.evaluatePredicate("user.count <= 100", { user: { count: 0 } }), true, "nested: user.count <= 100, zero");
assertEqual(az.evaluatePredicate("config.debug == 'true'", { config: { debug: "true" } }), true, "nested: config.debug == 'true'");
assertEqual(az.evaluatePredicate("config.debug == 'true'", { config: { debug: "false" } }), false, "nested: config.debug != 'true'");
assertEqual(az.evaluatePredicate("user.balance >= 0", { user: { balance: -5 } }), false, "nested: negative balance");
assertEqual(az.evaluatePredicate("stats.total > 0", { stats: { total: 1 } }), true, "nested: stats.total > 0");

section("Nested path: evaluatePredicate boolean AND/OR chains");

assertEqual(az.evaluatePredicate("user.age > 18 && user.active", { user: { age: 21, active: true } }), true, "nested AND: both true");
assertEqual(az.evaluatePredicate("user.age > 18 && user.active", { user: { age: 21, active: false } }), false, "nested AND: second false");
assertEqual(az.evaluatePredicate("user.age > 18 && user.active", { user: { age: 15, active: true } }), false, "nested AND: first false");
assertEqual(az.evaluatePredicate("user.admin || user.moderator", { user: { admin: true, moderator: false } }), true, "nested OR: first true");
assertEqual(az.evaluatePredicate("user.admin || user.moderator", { user: { admin: false, moderator: true } }), true, "nested OR: second true");
assertEqual(az.evaluatePredicate("user.admin || user.moderator", { user: { admin: false, moderator: false } }), false, "nested OR: both false");
assertEqual(az.evaluatePredicate("user.age > 18 && user.role == 'admin'", { user: { age: 21, role: "admin" } }), true, "nested: comparison AND equality");
assertEqual(az.evaluatePredicate("user.age > 18 && user.role == 'admin'", { user: { age: 21, role: "user" } }), false, "nested: comparison true, equality false");
assertEqual(az.evaluatePredicate("config.enabled && config.visible", { config: { enabled: true, visible: true } }), true, "nested AND: both true");
assertEqual(az.evaluatePredicate("config.enabled && config.visible", { config: { enabled: false, visible: true } }), false, "nested AND: first false");

section("Nested path: evaluateExpression field lookup and arithmetic");

assertEqual(az.evaluateExpression("user.name", { user: { name: "Alice" } }), "Alice", "nested field lookup: user.name");
assertEqual(az.evaluateExpression("user.age", { user: { age: 30 } }), 30, "nested field lookup: user.age number");
assertEqual(az.evaluateExpression("user.active", { user: { active: true } }), true, "nested field lookup: user.active boolean");
assertEqual(az.evaluateExpression("config.theme", { config: { theme: "dark" } }), "dark", "nested field lookup: config.theme");
assertEqual(az.evaluateExpression("a.b.c", { a: { b: { c: "deep" } } }), "deep", "deep nested: a.b.c");
assertEqual(az.evaluateExpression("user.count + 1", { user: { count: 5 } }), 6, "nested inc: user.count + 1");
assertEqual(az.evaluateExpression("user.count + 1", { user: { count: -1 } }), 0, "nested inc: user.count + 1, from negative");
assertEqual(az.evaluateExpression("user.score - 5", { user: { score: 15 } }), 10, "nested dec: user.score - 5");
assertEqual(az.evaluateExpression("stats.total + 1", { stats: { total: 0 } }), 1, "nested inc: stats.total + 1 from zero");
assertEqual(az.evaluateExpression("user.count + 1", {}), 1, "nested inc with missing parent → 0 + 1");
assertEqual(az.evaluateExpression("user.count + 1", { user: {} }), 1, "nested inc with empty user object → 0 + 1");
assertEqual(az.evaluateExpression("a.b.c + 10", { a: { b: { c: 5 } } }), 15, "deep nested arithmetic: a.b.c + 10");
assertEqual(az.evaluateExpression("user.score - 0.5", { user: { score: 5.5 } }), 5, "float arithmetic: user.score - 0.5");

section("Nested path: evaluateExpression || defaults");

assertEqual(az.evaluateExpression("user.name || 'Guest'", { user: { name: "Alice" } }), "Alice", "|| with nested: truthy value passes through");
assertEqual(az.evaluateExpression("user.name || 'Guest'", { user: { name: null } }), "Guest", "|| with nested: null → default");
assertEqual(az.evaluateExpression("user.name || 'Guest'", { user: { name: undefined } }), "Guest", "|| with nested: undefined → default");
assertEqual(az.evaluateExpression("user.name || 'Guest'", { user: { name: "" } }), "default", "|| with nested: empty string → default");
assertEqual(az.evaluateExpression("user.name || 'Guest'", { user: { name: false } }), false, "|| with nested: false → false (not defaulted)");
assertEqual(az.evaluateExpression("user.name || 'Guest'", { user: { name: 0 } }), 0, "|| with nested: 0 → 0 (not defaulted)");
assertEqual(az.evaluateExpression("user.name || 'Guest'", {}), "Guest", "|| with nested: missing user → default");
assertEqual(az.evaluateExpression("config.theme || 'light'", { config: { theme: "dark" } }), "dark", "|| with nested: config.theme");
assertEqual(az.evaluateExpression("config.theme || 'light'", { config: {} }), "light", "|| with nested: empty config → default");
assertEqual(az.evaluateExpression("a.b.c || 'default'", { a: { b: { c: "value" } } }), "value", "|| deep nested: a.b.c");
assertEqual(az.evaluateExpression("a.b.c || 'default'", { a: { b: {} } }), "default", "|| deep nested: missing c → default");
assertEqual(az.evaluateExpression("settings.preferences.locale || 'en'", { settings: { preferences: { locale: "fr" } } }), "fr", "|| very deep nested");

// ─── applyPrediction Tests ─────────────────────────────────────────────────

section("applyPrediction: simple flat set");

const state1 = { count: 0, name: "" };
az.applyPrediction(state1, "count = 5");
assertEqual(state1.count, 5, "applyPrediction: simple set number");

const state2 = { count: 0 };
az.applyPrediction(state2, "name = 'Alice'");
assertEqual(state2.name, "Alice", "applyPrediction: simple set string");

const state3 = { flag: false };
az.applyPrediction(state3, "flag = true");
assertEqual(state3.flag, true, "applyPrediction: set to true");

const state4 = { flag: true };
az.applyPrediction(state4, "flag = false");
assertEqual(state4.flag, false, "applyPrediction: set to false");

const state5 = { value: 0 };
az.applyPrediction(state5, "value = -42");
assertEqual(state5.value, -42, "applyPrediction: set negative number");

section("applyPrediction: nested path set");

const s1 = { user: { count: 0, name: "" } };
az.applyPrediction(s1, "user.count = 5");
assertEqual(s1.user.count, 5, "applyPrediction: nested set number");

const s2 = { user: { count: 0 } };
az.applyPrediction(s2, "user.name = 'Alice'");
assertEqual(s2.user.name, "Alice", "applyPrediction: nested set string");

const s3 = { user: { profile: { age: 0 } } };
az.applyPrediction(s3, "user.profile.age = 25");
assertEqual(s3.user.profile.age, 25, "applyPrediction: deep nested set");

const s4 = { config: { theme: "light" } };
az.applyPrediction(s4, "config.theme = 'dark'");
assertEqual(s4.config.theme, "dark", "applyPrediction: nested string overwrite");

const s5 = { a: { b: { c: 0 } } };
az.applyPrediction(s5, "a.b.c = 99");
assertEqual(s5.a.b.c, 99, "applyPrediction: deep set");

const s6 = { user: { active: false } };
az.applyPrediction(s6, "user.active = true");
assertEqual(s6.user.active, true, "applyPrediction: nested set boolean");

section("applyPrediction: toggle !field");

const t1 = { user: { active: true } };
az.applyPrediction(t1, "!user.active");
assertEqual(t1.user.active, false, "applyPrediction: toggle true → false");

const t2 = { user: { active: false } };
az.applyPrediction(t2, "!user.active");
assertEqual(t2.user.active, true, "applyPrediction: toggle false → true");

const t3 = { flag: true };
az.applyPrediction(t3, "!flag");
assertEqual(t3.flag, false, "applyPrediction: flat toggle true → false");

const t4 = { flag: false };
az.applyPrediction(t4, "!flag");
assertEqual(t4.flag, true, "applyPrediction: flat toggle false → true");

const t5 = { user: { settings: { notifications: true } } };
az.applyPrediction(t5, "!user.settings.notifications");
assertEqual(t5.user.settings.notifications, false, "applyPrediction: deep toggle");

section("applyPrediction: increment/decrement");

const i1 = { count: 5 };
az.applyPrediction(i1, "count = count + 1");
assertEqual(i1.count, 6, "applyPrediction: flat increment");

const i2 = { count: 10 };
az.applyPrediction(i2, "count = count - 3");
assertEqual(i2.count, 7, "applyPrediction: flat decrement");

const i3 = { user: { count: 0 } };
az.applyPrediction(i3, "user.count = user.count + 1");
assertEqual(i3.user.count, 1, "applyPrediction: nested increment");

const i4 = { user: { score: 50 } };
az.applyPrediction(i4, "user.score = user.score + 10");
assertEqual(i4.user.score, 60, "applyPrediction: nested increment by 10");

const i5 = { user: { score: 100 } };
az.applyPrediction(i5, "user.score = user.score - 25");
assertEqual(i5.user.score, 75, "applyPrediction: nested decrement");

const i6 = { stats: { total: 0 } };
az.applyPrediction(i6, "stats.total = stats.total + 1");
assertEqual(i6.stats.total, 1, "applyPrediction: deep increment");

section("applyPrediction: creates intermediate objects (pre-existing parent)");

// setNested only creates intermediate objects if the parent already exists
// It CANNOT create `a.b` from an empty `{}` — the parent `a` must exist first
// This is a security design: partial paths must exist for safety
const m1 = { a: { b: {} } };
az.applyPrediction(m1, "a.b.c = 5");
assertEqual(m1.a.b.c, 5, "applyPrediction: creates c under existing a.b");

const m2 = { user: { profile: {} } };
az.applyPrediction(m2, "user.profile.age = 30");
assertEqual(m2.user.profile.age, 30, "applyPrediction: deep set under existing chain");

const m3 = { existing: {} };
az.applyPrediction(m3, "existing.deep.value = 'set'");
assertEqual(m3.existing.deep.value, "set", "applyPrediction: extends existing object");

// Missing intermediate path does NOT create objects (security design)
const m4 = {};
az.applyPrediction(m4, "a.b.c = 5");
// setNested returns early because target (a) is undefined → m4 remains {}
assertEqual(m4.a, undefined, "applyPrediction: missing parent path → no creation");

section("applyPrediction: prototype pollution guard");

const warnCalls = [];
const originalWarn = az.warn.bind(az);
az.warn = (...args) => { warnCalls.push(args); };

const d1 = { __proto__: { admin: true } };
az.applyPrediction(d1, "__proto__ = 'blocked'");
assertEqual(d1.__proto__, undefined, "applyPrediction: __proto__ blocked, no change");

const d2 = { constructor: "test" };
az.applyPrediction(d2, "constructor = 'blocked'");
assertEqual(d2.constructor, "test", "applyPrediction: constructor blocked");

const d3 = {};
az.applyPrediction(d3, "prototype = 'blocked'");
assertEqual(d3.prototype, undefined, "applyPrediction: prototype blocked");

const d4 = {};
az.applyPrediction(d4, "__defineGetter__ = 'blocked'");
assertEqual(d4.__defineGetter__, undefined, "applyPrediction: __defineGetter__ blocked");

const d5 = {};
az.applyPrediction(d5, "__defineSetter__ = 'blocked'");
assertEqual(d5.__defineSetter__, undefined, "applyPrediction: __defineSetter__ blocked");

const d6 = {};
az.applyPrediction(d6, "hasOwnProperty = 'blocked'");
assertEqual(d6.hasOwnProperty, undefined, "applyPrediction: hasOwnProperty blocked");

const d7 = {};
az.applyPrediction(d7, "isPrototypeOf = 'blocked'");
assertEqual(d7.isPrototypeOf, undefined, "applyPrediction: isPrototypeOf blocked");

const d8 = {};
az.applyPrediction(d8, "propertyIsEnumerable = 'blocked'");
assertEqual(d8.propertyIsEnumerable, undefined, "applyPrediction: propertyIsEnumerable blocked");

const d9 = {};
az.applyPrediction(d9, "toLocaleString = 'blocked'");
assertEqual(d9.toLocaleString, undefined, "applyPrediction: toLocaleString blocked");

const d10 = {};
az.applyPrediction(d10, "toString = 'blocked'");
assertEqual(d10.toString, undefined, "applyPrediction: toString blocked");

const d11 = {};
az.applyPrediction(d11, "valueOf = 'blocked'");
assertEqual(d11.valueOf, undefined, "applyPrediction: valueOf blocked");

const d12 = {};
az.applyPrediction(d12, "__lookupGetter__ = 'blocked'");
assertEqual(d12.__lookupGetter__, undefined, "applyPrediction: __lookupGetter__ blocked");

const d13 = {};
az.applyPrediction(d13, "__lookupSetter__ = 'blocked'");
assertEqual(d13.__lookupSetter__, undefined, "applyPrediction: __lookupSetter__ blocked");

const d14 = {};
az.applyPrediction(d14, "prototype__ = 'blocked'");
assertEqual(d14.prototype__, undefined, "applyPrediction: prototype__ blocked");

az.warn = originalWarn;

// ─── Escaped String Tests ──────────────────────────────────────────────────

section("findTernaryIndex: escaped quotes inside strings");

assertEqual(az.findTernaryIndex("'it\\'s' ? 'yes' : 'no'"), 5, "findTernaryIndex: single quote in string, ? at index 5");
assertEqual(az.findTernaryIndex('"say \\"hello\\"" ? "a" : "b"'), 12, "findTernaryIndex: double quote in string");
assertEqual(az.findTernaryIndex("'escaped\\'' ? 'x' : 'y'"), 9, "findTernaryIndex: escaped quote at end of string");
assertEqual(az.findTernaryIndex("field == 'it\\'s' ? 'a' : 'b'"), 18, "findTernaryIndex: ternary after escaped quote equality");
assertEqual(az.findTernaryIndex("a 'b && c' d ? 'yes' : 'no'"), -1, "findTernaryIndex: && inside string not treated as operator");

section("findOperatorIndex: escaped quotes inside strings");

assertEqual(az.findOperatorIndex("'it\\'s' && 'ok'", "&&"), -1, "findOperatorIndex: && inside string skipped");
assertEqual(az.findOperatorIndex("field == 'a && b' || 'c'", "||"), 21, "findOperatorIndex: || after string with &&");
assertEqual(az.findOperatorIndex('"a\\"b" || "c"', "||"), 10, "findOperatorIndex: || with escaped double quotes");
assertEqual(az.findOperatorIndex("'test\\'s' || 'default'", "||"), 12, "findOperatorIndex: || with escaped single quote");
assertEqual(az.findOperatorIndex("'a\\\\b' || 'c'", "||"), 8, "findOperatorIndex: || with escaped backslash");

section("evaluateExpression: escaped string unescaping edge cases");

assertEqual(az.evaluateExpression("'\\\\n'"), "\\n", "evaluateExpression: backslash-n literal (not newline)");
assertEqual(az.evaluateExpression("'\\\\t'"), "\\t", "evaluateExpression: backslash-t literal (not tab)");
assertEqual(az.evaluateExpression("'\\\\\\\\'"), "\\\\", "evaluateExpression: double backslash");
assertEqual(az.evaluateExpression("'hello\\\\'"), "hello\\", "evaluateExpression: trailing backslash");
assertEqual(az.evaluateExpression("'a\\\\b\\\\c'"), "a\\b\\c", "evaluateExpression: multiple backslashes");

section("evaluatePredicate: equality with escaped quotes in value");

assertEqual(az.evaluatePredicate("field == 'it\\'s'", { field: "it's" }), true, "equality: field with escaped apostrophe");
assertEqual(az.evaluatePredicate("field == 'it\\'s'", { field: "it\\'s" }), false, "equality: backslash-apostrophe not same as apostrophe");
assertEqual(az.evaluatePredicate("field == \"say \\\"hi\\\"\"", { field: 'say "hi"' }), true, "equality: field with escaped double quotes");
assertEqual(az.evaluatePredicate("field == 'a\\\\b'", { field: "a\\b" }), true, "equality: field with escaped backslash");
assertEqual(az.evaluatePredicate("field != 'it\\'s'", { field: "different" }), true, "inequality: field differs from escaped-quote value");

section("evaluateExpression: string literals with escaped quotes");

assertEqual(az.evaluateExpression("'a\\'b\\'c'"), "a'b'c", "expression: multiple escaped apostrophes");
assertEqual(az.evaluateExpression('"a\\"b\\"c"'), "a\"b\"c", "expression: multiple escaped double quotes");

// ─── Deep Nested Ternary Tests ─────────────────────────────────────────────

section("Deep nested ternary: 3+ levels without parens");

assertEqual(az.evaluateExpression("a ? b ? c ? 'deep' : 'mid2' : 'mid1' : 'top'", { a: true, b: true, c: true }), "deep", "3-level ternary: all true");
assertEqual(az.evaluateExpression("a ? b ? c ? 'deep' : 'mid2' : 'mid1' : 'top'", { a: true, b: true, c: false }), "mid2", "3-level ternary: a,b true, c false");
assertEqual(az.evaluateExpression("a ? b ? c ? 'deep' : 'mid2' : 'mid1' : 'top'", { a: true, b: false }), "mid1", "3-level ternary: a true, b false");
assertEqual(az.evaluateExpression("a ? b ? c ? 'deep' : 'mid2' : 'mid1' : 'top'", { a: false }), "top", "3-level ternary: a false");
assertEqual(az.evaluateExpression("a ? b ? c ? 'deep' : 'mid2' : 'mid1' : 'top'", { a: true, b: true }), "mid2", "3-level ternary: a,b true, c missing → mid2");

// ─── Paren + Negation Edge Cases ──────────────────────────────────────────

section("Paren grouping with negation edge cases");

assertEqual(az.evaluatePredicate("!(a && b)", { a: true, b: true }), false, "!(a && b): both true → !true → false");
assertEqual(az.evaluatePredicate("!(a && b)", { a: true, b: false }), true, "!(a && b): second false → !(false) → true");
assertEqual(az.evaluatePredicate("!(a || b)", { a: false, b: false }), true, "!(a || b): both false → !false → true");
assertEqual(az.evaluatePredicate("!(a || b)", { a: true, b: false }), false, "!(a || b): one true → !true → false");
assertEqual(az.evaluatePredicate("!(user.age > 18)", { user: { age: 21 } }), false, "!(user.age > 18): true → !true → false");
assertEqual(az.evaluatePredicate("!(user.age > 18)", { user: { age: 15 } }), true, "!(user.age > 18): false → !false → true");
assertEqual(az.evaluatePredicate("!!(flag)", { flag: false }), false, "!!(flag): double negation of paren");
assertEqual(az.evaluatePredicate("!(!flag)", { flag: true }), false, "!(!flag): double negation cancelled");

section("Combined: paren + nested path + comparison");

assertEqual(az.evaluatePredicate("(user.age > 18)", { user: { age: 21 } }), true, "paren with nested comparison: truthy");
assertEqual(az.evaluatePredicate("(user.age > 18)", { user: { age: 15 } }), false, "paren with nested comparison: falsy");
assertEqual(az.evaluatePredicate("!(user.admin || user.moderator)", { user: { admin: false, moderator: false } }), true, "!(nested OR): both false → !false");
assertEqual(az.evaluatePredicate("!(user.admin || user.moderator)", { user: { admin: true, moderator: false } }), false, "!(nested OR): one true → !true");
assertEqual(az.evaluatePredicate("(user.count > 0) && user.active", { user: { count: 5, active: true } }), true, "paren comparison AND nested bool");

// ─── Null/Undefined Edge Cases ─────────────────────────────────────────────

section("Null/undefined state edge cases");

assertEqual(az.evaluatePredicate("field", { field: null }), false, "predicate: null field is falsy");
assertEqual(az.evaluatePredicate("field", { field: undefined }), false, "predicate: undefined field is falsy");
assertEqual(az.evaluatePredicate("field", {}), false, "predicate: missing field → false");
assertEqual(az.evaluatePredicate("user.name", { user: { name: null } }), false, "predicate: nested null → falsy");
assertEqual(az.evaluatePredicate("user.name", { user: {} }), false, "predicate: nested missing → false");

assertEqual(az.evaluateExpression("field", { field: null }), null, "expression: null field returns null");
assertEqual(az.evaluateExpression("field", { field: undefined }), undefined, "expression: undefined field returns undefined");
assertEqual(az.evaluateExpression("user.name", { user: { name: null } }), null, "expression: nested null");
assertEqual(az.evaluateExpression("user.name", { user: {} }), undefined, "expression: nested missing");

assertEqual(az.evaluatePredicate(null, {}), false, "predicate: null expression");
assertEqual(az.evaluatePredicate("", {}), false, "predicate: empty expression");
assertEqual(az.evaluateExpression("", {}), "", "expression: empty expression");

// ─── Whitespace Edge Cases ───────────────────────────────────────────────

section("Whitespace handling in nested paths");

assertEqual(az.evaluatePredicate("  user.age > 18  ", { user: { age: 21 } }), true, "predicate with surrounding spaces");
assertEqual(az.evaluateExpression("  user.name  ", { user: { name: "Alice" } }), "Alice", "expression with surrounding spaces");
assertEqual(az.evaluatePredicate("user.age>18", { user: { age: 21 } }), true, "comparison with no spaces");
assertEqual(az.evaluatePredicate("user.age  >  18", { user: { age: 21 } }), true, "comparison with extra spaces");
assertEqual(az.evaluateExpression("user.name  ", { user: { name: "Bob" } }), "Bob", "field with trailing spaces");

// ─── Security: Prototype Pollution ─────────────────────────────────────────

section("Security: prototype pollution blocking");

assertEqual(az.evaluatePredicate("__proto__", { "__proto__": true }), true, "__proto__ as truthy predicate");
assertEqual(az.evaluateExpression("constructor", { constructor: "poison" }), "poison", "constructor field returns value");
assertEqual(az.evaluateExpression("__proto__.foo", {}), "__proto__.foo", "__proto__ with property access returned as-is");
assertEqual(az.evaluateExpression("a.__proto__", { a: 1 }), "a.__proto__", "member __proto__ returned as-is");
assertEqual(az.evaluateExpression("a.constructor", { a: 1 }), "a.constructor", "member constructor returned as-is");
assertEqual(az.evaluateExpression("constructor", { constructor: null }), null, "constructor field can hold null");
assertEqual(az.evaluatePredicate("constructor", { constructor: false }), false, "constructor field as falsy predicate");

// ─── Evaluator Edge Cases ──────────────────────────────────────────────────

section("Evaluator edge cases: chained, type coercion");

// Nested ternaries with parens
assertEqual(az.evaluateExpression("a ? (b ? 'x' : 'y') : 'z'", { a: true, b: true }), "x", "nested ternary: outer truthy, inner truthy");
assertEqual(az.evaluateExpression("a ? (b ? 'x' : 'y') : 'z'", { a: true, b: false }), "y", "nested ternary: outer truthy, inner falsy");
assertEqual(az.evaluateExpression("a ? (b ? 'x' : 'y') : 'z'", { a: false }), "z", "nested ternary: outer falsy");

// Nested ternaries without parens
assertEqual(az.evaluateExpression("a ? b ? c : d : e", { a: true, b: true, c: "yes", d: "maybe", e: "no" }), "yes", "no-parens nested ternary: a&&b&&c → yes");
assertEqual(az.evaluateExpression("a ? b ? c : d : e", { a: true, b: false, c: "yes", d: "maybe", e: "no" }), "maybe", "no-parens nested ternary: a&&!b → maybe");
assertEqual(az.evaluateExpression("a ? b ? c : d : e", { a: false, b: true, c: "yes", d: "maybe", e: "no" }), "no", "no-parens nested ternary: !a → e");
assertEqual(az.evaluateExpression("a ? 'yes' : b ? 'maybe' : 'no'", { a: true, b: false }), "yes", "chained ternary-like expr (no parens)");
assertEqual(az.evaluateExpression("flag ? 'a' : flag2 ? 'b' : 'c'", { flag: true, flag2: false }), "a", "first ternary truthy");
assertEqual(az.evaluateExpression("flag ? 'a' : flag2 ? 'b' : 'c'", { flag: false, flag2: true }), "b", "second ternary when first falsy");
assertEqual(az.evaluateExpression("flag ? 'a' : flag2 ? 'b' : 'c'", { flag: false, flag2: false }), "c", "neither ternary truthy");

// Float comparisons
assertEqual(az.evaluatePredicate("score > 3", { score: 4.2 }), true, "float comparison: 4.2 > 3");
assertEqual(az.evaluatePredicate("score > 3", { score: 2.9 }), false, "float comparison: 2.9 !> 3");
assertEqual(az.evaluatePredicate("score >= 3", { score: 3 }), true, "float comparison: 3 >= 3");
assertEqual(az.evaluatePredicate("score > 3.5", { score: 4.2 }), true, "float comparison: 4.2 > 3.5");
assertEqual(az.evaluatePredicate("price < 9.99", { price: 8.5 }), true, "float comparison: 8.5 < 9.99");
assertEqual(az.evaluateExpression("price - 1", { price: 10.5 }), 9.5, "float subtraction (integer right operand)");
assertEqual(az.evaluateExpression("price - 1", { price: 10 }), 9, "float subtraction with whole number");

// Type coercion
assertEqual(az.evaluatePredicate("count == '5'", { count: 5 }), false, "number vs string equality: 5 != '5'");
assertEqual(az.evaluatePredicate("count == '5'", { count: "5" }), true, "string vs string equality: '5' == '5'");
assertEqual(az.evaluatePredicate("count != '5'", { count: 5 }), true, "number vs string inequality: 5 != '5'");
assertEqual(az.evaluatePredicate("name == 'Alice'", { name: "Alice" }), true, "string equality exact match");
assertEqual(az.evaluatePredicate("name == 'Alice'", { name: "alice" }), false, "string equality case sensitive");
assertEqual(az.evaluatePredicate("name == 'Alice'", { name: "Alice " }), false, "string equality with trailing space");

// Negation compounds
assertEqual(az.evaluatePredicate("!a && b", { a: false, b: true }), true, "!a && b → !false && true");
assertEqual(az.evaluatePredicate("a && !b", { a: true, b: false }), true, "a && !b → true && !false");
assertEqual(az.evaluatePredicate("!a && !b", { a: false, b: false }), true, "!a && !b → both false");
assertEqual(az.evaluatePredicate("!a || !b", { a: true, b: true }), false, "!a || !b → both truthy → false");
assertEqual(az.evaluatePredicate("!!flag", { flag: false }), false, "double negation cancels");
assertEqual(az.evaluatePredicate("!!!flag", { flag: true }), false, "triple negation");

// Deep compound chains
assertEqual(az.evaluatePredicate("a && b || c && d", { a: true, b: false, c: true, d: true }), true, "AND/OR chain: ((T&&F)||T)&&T");
assertEqual(az.evaluatePredicate("a && b || c && d", { a: true, b: true, c: true, d: true }), true, "AND/OR chain: ((T&&T)||T)&&T = T");
assertEqual(az.evaluatePredicate("a && b || c && d", { a: true, b: true, c: false, d: true }), true, "AND/OR chain: ((T&&T)||F)&&T = T");
assertEqual(az.evaluatePredicate("a && b || c && d", { a: true, b: false, c: false, d: true }), false, "AND/OR chain: ((T&&F)||F)&&T = F");
assertEqual(az.evaluatePredicate("a || b && c || d", { a: false, b: true, c: true, d: false }), true, "OR/AND chain: (F||(T&&T))||F");
assertEqual(az.evaluatePredicate("a || b && c || d", { a: false, b: false, c: false, d: false }), false, "OR/AND chain: all false");
assertEqual(az.evaluatePredicate("a && b && c && d", { a: true, b: true, c: true, d: true }), true, "AND quad chain all true");
assertEqual(az.evaluatePredicate("a && b && c && d", { a: true, b: true, c: true, d: false }), false, "AND quad chain last false");
assertEqual(az.evaluatePredicate("a || b || c || d", { a: false, b: false, c: false, d: true }), true, "OR quad chain last true");
assertEqual(az.evaluatePredicate("a || b || c || d", { a: false, b: false, c: false, d: false }), false, "OR quad chain all false");

// Empty string literals
assertEqual(az.evaluateExpression("''", {}), "", "empty single-quoted string");
assertEqual(az.evaluateExpression('""', {}), "", "empty double-quoted string");
assertEqual(az.evaluatePredicate("field == ''", { field: "" }), true, "empty string equality");
assertEqual(az.evaluatePredicate("field == ''", { field: "x" }), false, "non-empty vs empty string equality");

// ─── Summary ─────────────────────────────────────────────────────────────

console.log("\n" + "=".repeat(50));
console.log(`Results: ${passed} passed, ${failed} failed`);
if (failed === 0) {
    console.log("All tests passed!");
} else {
    process.exit(1);
}