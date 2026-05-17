# Client Interactivity Reference

> Built-in client features that replace custom JavaScript.

## Decision Tree

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

## 1. Form Actions (`az-action` + `az-target`)

Submit a form to a server action and swap the result HTML.

```html
<form az-action="save_settings" az-target="#result" az-swap="morph">
    <input name="theme" type="text">
    <button type="submit">Save</button>
</form>
<div id="result"></div>
```

**Server handler:**
```rust
async fn save_settings() -> impl IntoResponse {
    azumi::action::success_fragment("<p>Settings saved!</p>")
}
```

## 2. Client State (`az-ui` + `set`)

Manage state entirely on the client without server round-trips.

```html
<div az-ui='{"count":0}'>
    <p>Count: <span az-bind:text="count">0</span></p>
    <button az-on="click set count = count + 1">+</button>
</div>
```

**State priority:**
1. Optimistic predictions (ephemeral)
2. `az-ui` attribute (client-set)
3. `az-scope` attribute (server-signed)

## 3. Event Handlers (`az-on`)

Trigger actions on DOM events.

```html
<button az-on="click call toggle_like -> #like-area">Toggle</button>
<select az-on="change call filter -> #results">...</select>
```

Syntax: `{event} call {action} -> {target} {swap}`

## 4. Text Bindings (`az-bind:text`)

```html
<div az-ui='{"name":"World"}'>
    <h1 az-bind:text="name">Placeholder</h1>
    <p az-bind:text="'Hello, ' + name + '!'">...</p>
</div>
```

## 5. Conditional Classes (`az-bind:class`)

```html
<div az-ui='{"liked":false}'>
    <button az-on="click set liked = !liked"
            az-bind:class:active="liked">
        Like
    </button>
</div>
```

## 6. Confirmation (`az-confirm`)

```html
<form az-action="delete_account" az-confirm="Delete your account permanently?">
    <button type="submit">Delete</button>
</form>
```

## 7. Auto-Init (`az-init`)

```html
<div az-init="call load_data -> #feed">
    <div id="feed">Loading...</div>
</div>
```

## 8. Scroll Reveal (`az-reveal`)

```html
<style>
    [az-reveal] { opacity: 0; transform: translateY(20px); transition: 0.6s; }
    [az-reveal][data-revealed] { opacity: 1; transform: translateY(0); }
</style>
<div az-reveal><h2>Fades in when visible</h2></div>
```

## 9. Scroll Top (`scroll-top`)

```html
<button az-on="click scroll-top">Back to Top</button>
```

## What Still Needs Custom JS

| Feature | Why |
|---------|-----|
| Canvas/WebGL | Browser API |
| WebSocket chat | Persistent connections |
| External APIs | Stripe, Paddle, Maps |
| Complex drag-and-drop | HTML5 DnD API |
| File handling | FileReader |
| Audio/video | Media APIs |

---

## Pattern Catalog: JS → Azumi

### Pattern 1: Tabs

**Before (JS):**
```javascript
// tabs.js — 45 lines
document.querySelectorAll('.tab-btn').forEach(btn => {
    btn.addEventListener('click', () => {
        document.querySelectorAll('.tab-panel').forEach(p => p.classList.remove('active'));
        document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
        document.getElementById(btn.dataset.target).classList.add('active');
        btn.classList.add('active');
    });
});
```

**After (Azumi):**
```html
<div az-ui='{"tab":"general"}'>
    <div class="tabs">
        <button az-on="click set tab = 'general'" az-bind:class:active="tab == 'general'">General</button>
        <button az-on="click set tab = 'security'" az-bind:class:active="tab == 'security'">Security</button>
    </div>
    <div az-bind:class:hidden="tab != 'general'">General settings...</div>
    <div az-bind:class:hidden="tab != 'security'">Security settings...</div>
</div>
```

**Result:** 45 lines of JS → 0 lines. State is in the HTML. No event listeners to manage.

---

### Pattern 2: Modal / Dialog

**Before (JS):**
```javascript
// modal.js — 38 lines
const modal = document.getElementById('modal');
const overlay = document.getElementById('overlay');

document.getElementById('open').addEventListener('click', () => {
    modal.style.display = 'block';
    overlay.style.display = 'block';
});

document.getElementById('close').addEventListener('click', () => {
    modal.style.display = 'none';
    overlay.style.display = 'none';
});

overlay.addEventListener('click', () => {
    modal.style.display = 'none';
    overlay.style.display = 'none';
});
```

**After (Azumi):**
```html
<div az-ui='{"open":false}'>
    <button az-on="click set open = true">Open Modal</button>
    <div az-bind:class:hidden="!open" class="overlay" az-on="click set open = false">
        <div class="modal" az-on="click stop">
            <h2>Modal Title</h2>
            <button az-on="click set open = false">Close</button>
        </div>
    </div>
</div>
```

**Result:** 38 lines of JS → 0 lines. No DOM queries, no event listener cleanup.

---

### Pattern 3: Accordion

**Before (JS):**
```javascript
// accordion.js — 52 lines
document.querySelectorAll('.accordion-header').forEach(header => {
    header.addEventListener('click', () => {
        const item = header.parentElement;
        const isOpen = item.classList.contains('open');
        
        // Close all
        document.querySelectorAll('.accordion-item').forEach(i => {
            i.classList.remove('open');
            i.querySelector('.accordion-body').style.maxHeight = '0';
        });
        
        // Open clicked (if wasn't open)
        if (!isOpen) {
            item.classList.add('open');
            const body = item.querySelector('.accordion-body');
            body.style.maxHeight = body.scrollHeight + 'px';
        }
    });
});
```

**After (Azumi):**
```html
<div az-ui='{"open":null}'>
    <div class="accordion-item" az-bind:class:open="open == 1">
        <button az-on="click set open = open == 1 ? null : 1" class="accordion-header">Section 1</button>
        <div az-bind:class:hidden="open != 1" class="accordion-body">Content 1</div>
    </div>
    <div class="accordion-item" az-bind:class:open="open == 2">
        <button az-on="click set open = open == 2 ? null : 2" class="accordion-header">Section 2</button>
        <div az-bind:class:hidden="open != 2" class="accordion-body">Content 2</div>
    </div>
</div>
```

**Result:** 52 lines of JS → 0 lines. No scrollHeight calculations, no maxHeight manipulation.

---

### Pattern 4: Form Submit with Feedback

**Before (JS):**
```javascript
// form.js — 48 lines
const form = document.getElementById('contact');
const result = document.getElementById('result');

form.addEventListener('submit', async (e) => {
    e.preventDefault();
    const btn = form.querySelector('button');
    btn.disabled = true;
    btn.textContent = 'Sending...';
    
    try {
        const res = await fetch('/api/contact', {
            method: 'POST',
            body: new FormData(form)
        });
        const html = await res.text();
        result.innerHTML = html;
        form.reset();
    } catch (err) {
        result.innerHTML = '<p class="error">Failed to send</p>';
    } finally {
        btn.disabled = false;
        btn.textContent = 'Send';
    }
});
```

**After (Azumi):**
```html
<form az-action="contact_submit" az-target="#result" az-swap="morph">
    <input name="email" type="email" required>
    <textarea name="message" required></textarea>
    <button type="submit">Send</button>
</form>
<div id="result"></div>
```

**Server:**
```rust
#[azumi::action]
pub async fn contact_submit(form: Form<ContactForm>) -> ActionResult {
    // send email...
    Ok(html! { <p class="success">"Message sent!"</p> })
}
```

**Result:** 48 lines of JS → 0 lines. FormData, fetch, error handling, loading state — all handled by the runtime.

---

### Pattern 5: Confirm Dialog

**Before (JS):**
```javascript
// confirm.js — 22 lines
document.getElementById('delete').addEventListener('click', (e) => {
    if (!confirm('Delete this item permanently?')) {
        e.preventDefault();
        return false;
    }
});
```

**After (Azumi):**
```html
<form az-action="delete_item" az-confirm="Delete this item permanently?">
    <button type="submit">Delete</button>
</form>
```

**Result:** 22 lines of JS → 1 attribute. No event listener, no preventDefault.

---

### Pattern 6: Scroll-Triggered Animation

**Before (JS):**
```javascript
// reveal.js — 35 lines
const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.classList.add('revealed');
            observer.unobserve(entry.target);
        }
    });
}, { threshold: 0.1 });

document.querySelectorAll('[data-reveal]').forEach(el => observer.observe(el));
```

**After (Azumi):**
```html
<div az-reveal><h2>Fades in when scrolled into view</h2></div>
```

With CSS:
```css
[az-reveal] { opacity: 0; transform: translateY(20px); transition: 0.6s; }
[az-reveal][data-revealed] { opacity: 1; transform: translateY(0); }
```

**Result:** 35 lines of JS → 1 attribute. IntersectionObserver handled by the runtime.

---

### Pattern 7: Live Search

**Before (JS):**
```javascript
// search.js — 67 lines
let timeout;
document.getElementById('search').addEventListener('input', (e) => {
    clearTimeout(timeout);
    timeout = setTimeout(async () => {
        const res = await fetch(`/api/search?q=${encodeURIComponent(e.target.value)}`);
        const html = await res.text();
        document.getElementById('results').innerHTML = html;
    }, 300);
});
```

**After (Azumi):**
```html
<form az-action="search" az-target="#results" az-trigger="input delay 300">
    <input name="q" type="search" placeholder="Search...">
</form>
<div id="results"></div>
```

**Result:** 67 lines of JS → 1 attribute. Debouncing, encoding, fetch — all handled.

---

### Pattern 8: Back to Top

**Before (JS):**
```javascript
// back-to-top.js — 19 lines
document.getElementById('top').addEventListener('click', () => {
    window.scrollTo({ top: 0, behavior: 'smooth' });
});
```

**After (Azumi):**
```html
<button az-on="click scroll-top">Back to Top</button>
```

**Result:** 19 lines of JS → 1 attribute.

---

## Lines Saved Summary

| Pattern | Before (JS lines) | After (Azumi) | Saved |
|---------|-------------------|---------------|-------|
| Tabs | 45 | 0 | 45 |
| Modal | 38 | 0 | 38 |
| Accordion | 52 | 0 | 52 |
| Form submit | 48 | 0 | 48 |
| Confirm | 22 | 0 | 22 |
| Scroll reveal | 35 | 0 | 35 |
| Live search | 67 | 0 | 67 |
| Back to top | 19 | 0 | 19 |
| **Total** | **326** | **0** | **326** |

These patterns represent the most common interactivity needs in web apps. Azumi's built-in features eliminate **all** of the corresponding JavaScript.
