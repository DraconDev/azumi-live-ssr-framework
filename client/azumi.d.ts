/**
 * Type definitions for azumi.js client runtime.
 *
 * This file provides TypeScript types for hybrid teams that use
 * Azumi's client-side API alongside the Rust framework.
 *
 * @example
 * ```ts
 * import { Azumi } from 'azumi';
 * // or just use the global: window.Azumi
 * ```
 */

declare global {
    interface Window {
        Azumi: typeof Azumi;
    }
}

/**
 * Azumi client runtime — event delegation, DOM morphing, optimistic UI.
 *
 * This class is instantiated automatically as `new Azumi()` at the end
 * of the script. You typically don't create instances yourself.
 */
declare class Azumi {
    /** Enable debug logging to console */
    debug: boolean;

    /** WeakMap of scope elements to their state cache */
    scopes: WeakMap<Element, Record<string, unknown>>;

    constructor();

    /**
     * Set up IntersectionObserver for `az-reveal` elements.
     * Called automatically on init; call again if you add new
     * `az-reveal` elements dynamically.
     */
    setupReveal(): void;

    /**
     * Re-observe any new `az-reveal` elements after DOM morphing.
     */
    observeRevealElements(): void;

    /**
     * Immediately reveal all `az-reveal` elements (e.g. for
     * users who prefer reduced motion).
     */
    revealAll(): void;

    /** Debug log — only prints when `this.debug` is true */
    log(...args: unknown[]): void;

    /** Debug warn — only prints when `this.debug` is true */
    warn(...args: unknown[]): void;

    /** Debug error — only prints when `this.debug` is true */
    error(...args: unknown[]): void;

    /**
     * Connect to the hot reload WebSocket endpoint.
     * Only active in development mode.
     */
    connectHotReload(): void;

    /**
     * Poll the server to check if it's back up after a disconnect.
     * Auto-refreshes the page when the server responds.
     */
    pollForReload(): void;

    /**
     * Handle a style update pushed from the hot reload server.
     * @param scopeId - The CSS scope ID to update
     * @param css - The new scoped CSS content
     */
    handleStyleUpdate(scopeId: string, css: string): void;

    /**
     * Apply an optimistic UI prediction to the DOM.
     * @param scopeEl - The scope element containing the state
     * @param dsl - The prediction DSL string (e.g. "count = count + 1")
     * @returns The prediction result with original state for rollback
     */
    applyPrediction(
        scopeEl: Element,
        dsl: string
    ): { newState: Record<string, unknown>; originalState: Record<string, unknown> };

    /**
     * Roll back a failed optimistic update, restoring the original state.
     * @param scopeEl - The scope element
     * @param originalState - The state snapshot before the prediction
     */
    rollbackPrediction(
        scopeEl: Element,
        originalState: Record<string, unknown>
    ): void;

    /**
     * Read the signed state from a scope element's `az-scope` attribute.
     * @param scopeEl - The element with `az-scope`
     * @returns The parsed state object
     */
    readState(scopeEl: Element): Record<string, unknown>;

    /**
     * Evaluate a prediction DSL expression against a state object.
     * @param state - The current state
     * @param dsl - The prediction DSL (e.g. "count = count + 1", "open = !open")
     * @returns The new state after applying the prediction
     */
    evaluateExpression(
        state: Record<string, unknown>,
        dsl: string
    ): Record<string, unknown>;

    /**
     * Evaluate a prediction predicate (e.g. "count > 0").
     * @param state - The current state
     * @param predicate - The predicate DSL
     * @returns Whether the predicate is true
     */
    evaluatePredicate(
        state: Record<string, unknown>,
        predicate: string
    ): boolean;

    /**
     * Set a nested value on a state object using dot notation.
     * @param state - The state object
     * @param path - Dot-separated path (e.g. "user.name")
     * @param value - The value to set
     */
    setNested(state: Record<string, unknown>, path: string, value: unknown): void;

    /**
     * Execute local state changes from `az-ui` attribute DSL.
     * @param el - The element with the `az-ui` attribute
     * @param cmd - The local state command
     */
    executeLocalState(el: Element, cmd: string): void;

    /**
     * Delegate an event from an element to its action.
     * @param action - The action DSL string
     * @param el - The source element
     */
    delegate(action: string, el: Element): void;

    /**
     * Execute a server action via fetch + DOM morph.
     * @param action - The action DSL (method name on the state struct)
     * @param scopeEl - The scope element containing signed state
     * @param formEl - The form element (if any)
     */
    execute(action: string, scopeEl: Element, formEl?: HTMLFormElement | null): void;

    /**
     * Handle a delegated DOM event (click, submit, change, input).
     * @param e - The DOM event
     */
    handleEvent(e: Event): void;

    /**
     * Handle a form submission with `az-action`.
     * @param e - The submit event
     */
    handleFormSubmit(e: SubmitEvent): void;

    /**
     * Validate a form field against its `data-validate` rules.
     * @param input - The input element
     * @returns Whether the field is valid
     */
    validateFormField(input: HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement): boolean;

    /**
     * Parse an `az-on` action string into its components.
     * @param actionStr - The raw action string (e.g. "click call toggle -> #target")
     * @returns Parsed action object
     */
    parseAction(actionStr: string): {
        event: string;
        method: string;
        target?: string;
        swap?: string;
    };

    /**
     * Check if a URL string is valid.
     * @param url - The URL to validate
     */
    isValidUrl(url: string): boolean;

    /**
     * Check if an email string is valid.
     * @param email - The email to validate
     */
    isValidEmail(email: string): boolean;
}

export { Azumi };
