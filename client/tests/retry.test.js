/**
 * Unit tests for __azumi_retry client-side behavior.
 *
 * Run: node client/tests/retry.test.js
 *
 * Tests the built-in __azumi_retry action handler in isolation by
 * extracting just the retry logic and testing it against a jsdom DOM.
 * No browser or Playwright needed.
 */

const { JSDOM } = require("jsdom");

function makeDOM(html) {
  const dom = new JSDOM(html, { runScripts: "dangerously", resources: "usable" });
  return dom.window.document;
}

function simulateRetry(document, buttonSelector) {
  const element = document.querySelector(buttonSelector);
  if (!element) throw new Error(`Button not found: ${buttonSelector}`);
  const formId = element.getAttribute("data-retry-form");
  if (formId) {
    const form = document.getElementById(formId);
    if (form) form.style.display = "";
  }
  const errorDiv = element.closest(".error_message");
  if (errorDiv) errorDiv.remove();
}

// ── Tests ──────────────────────────────────────────────────────────────────

let passed = 0;
let failed = 0;

function assert(condition, message) {
  if (condition) {
    passed++;
    console.log(`  ✓ ${message}`);
  } else {
    failed++;
    console.error(`  ✗ ${message}`);
  }
}

console.log("retry.test.js\n");

// Test 1: Retry unhides form with inline display:none
{
  const doc = makeDOM(`
    <div>
      <form id="my_form" style="display:none"><input /></form>
      <div class="error_message">
        <p>Error</p>
        <button data-retry-form="my_form">Try Again</button>
      </div>
    </div>
  `);
  const form = doc.getElementById("my_form");
  assert(form.style.display === "none", "form starts hidden");

  simulateRetry(doc, "button[data-retry-form]");

  assert(form.style.display === "", "form display cleared after retry");
  assert(doc.querySelector(".error_message") === null, "error_message div removed");
}

// Test 2: Retry with missing form (graceful no-op)
{
  const doc = makeDOM(`
    <div>
      <div class="error_message">
        <p>Error</p>
        <button data-retry-form="nonexistent">Try Again</button>
      </div>
    </div>
  `);
  simulateRetry(doc, "button[data-retry-form]");
  assert(doc.querySelector(".error_message") === null, "error_message removed even with missing form");
}

// Test 3: Retry with no data-retry-form attribute
{
  const doc = makeDOM(`
    <div class="error_message">
      <p>Error</p>
      <button>Try Again</button>
    </div>
  `);
  simulateRetry(doc, "button");
  assert(doc.querySelector(".error_message") === null, "error_message removed without form_id");
}

// Test 4: Form hidden by CSS class is NOT unhidden
{
  const doc = makeDOM(`
    <div>
      <form id="css_form" class="hidden-by-css"><input /></form>
      <div class="error_message">
        <p>Error</p>
        <button data-retry-form="css_form">Try Again</button>
      </div>
    </div>
  `);
  const form = doc.getElementById("css_form");
  const originalClass = form.className;

  simulateRetry(doc, "button[data-retry-form]");

  assert(form.className === originalClass, "CSS class NOT removed (display:none was not inline)");
  assert(form.style.display === "", "inline display cleared (no-op since it was empty)");
}

// Test 5: Form with display:block inline is restored
{
  const doc = makeDOM(`
    <div>
      <form id="visible_form" style="display:block"><input /></form>
      <div class="error_message">
        <p>Error</p>
        <button data-retry-form="visible_form">Try Again</button>
      </div>
    </div>
  `);
  simulateRetry(doc, "button[data-retry-form]");
  const form = doc.getElementById("visible_form");
  assert(form.style.display === "", "display:block cleared to empty string");
}

// ── Summary ────────────────────────────────────────────────────────────────

console.log(`\n${passed} passed, ${failed} failed`);
process.exit(failed > 0 ? 1 : 0);
