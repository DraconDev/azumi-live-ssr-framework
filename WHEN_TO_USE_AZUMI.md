# When to Use Azumi vs Simpler Approaches

> **DEPRECATED**: This file has been consolidated into [docs/guide.md](docs/guide.md#when-to-use-azumi). The content below is preserved for reference.

> **TL;DR**: Use azumi for production apps with reusable components. Use `format!()` strings for quick pages.

---

## The Honest Trade-off Matrix

| Criteria | Azumi | `format!()` Strings |
|----------|-------|---------------------|
| **Reusability** | ✅ Component library | ❌ Copy-paste |
| **CSS drift prevention** | ✅ Co-located | ❌ Manual sync |
| **Compile-time checks** | ✅ Classes, a11y, HTML | ❌ None |
| **XSS prevention** | ✅ Automatic | ❌ Manual |
| **HMAC state signing** | ✅ Built-in | ❌ None |
| **Learning curve** | ❌ Steep | ✅ Flat |
| **Boilerplate** | ❌ High | ✅ Low |
| **DX for simple pages** | ❌ Verbose | ✅ Simple |

---

## When Simpler Approaches Win

Use `format!()` strings or libraries like **Maud** when:

### 1. One-off Static Pages

```rust
// Perfect for simple pages - no component library needed
pub fn about_page() -> String {
    format!(r#"
        <html>
        <head><title>About Us</title></head>
        <body>
            <h1>About Us</h1>
            <p>We are awesome.</p>
        </body>
        </html>
    "#)
}
```

**Verdict**: Azumi would be overkill here.

### 2. No Shared Components

If your site has:
- No reusable card patterns
- No repeated UI elements
- No component library aspirations

**Verdict**: The component model provides no benefit.

### 3. Team Unfamiliar with Rust Macros

Azumi's strict DSL (double-quoted values, snake_case, style ordering) has a learning curve.

**Verdict**: Use simpler approaches if your team is new.

---

## When Azumi Wins

Use Azumi when you need:

### 1. Reusable Component Library

```rust
// Components you use everywhere
ProductCard, SiteRuleRow, PricingTierCard, NavBar, Footer
```

**Without Azumi**: You copy-paste HTML/CSS, then update 10 files when designs change.

**With Azumi**: One change propagates everywhere.

### 2. Live State with Optimistic UI

```rust
#[azumi::live]
pub struct ShoppingCart {
    items: Vec<CartItem>,
    total: f64,
}

#[azumi::live_impl(component = "cart_view")]
impl ShoppingCart {
    pub fn add_item(&mut self, item: CartItem) {
        self.items.push(item);
        self.total += item.price;
    }
}
```

**Without Azumi**: Manual fetch, state management, re-render logic.

**With Azumi**: Predictions compile into HTML. Zero JS for state.

### 3. Production Security Requirements

| Threat | Azumi | Manual |
|--------|-------|--------|
| **XSS** | Automatic escaping | `html!` escapes, `Raw<T>` bypasses |
| **CSS Injection** | Escaping in `style={}` | Manual sanitization |
| **State Tampering** | HMAC verification | ❌ None |
| **Shell Injection** | 21-char filter in devtools | Manual validation |

**Verdict**: Azumi's security features prevent real attacks.

### 4. Complex Forms with Validation

```rust
#[azumi::component]
pub fn ContactForm<'a>(state: &'a FormState) -> impl Component + 'a {
    html! {
        <form on:submit={state.submit}>
            <input type="email" required={true} value={state.email} />
            // Validation errors shown at compile time
            // No runtime "undefined is not a function"
        </form>
    }
}
```

**Without Azumi**: Runtime validation errors, undefined properties.

**With Azumi**: Compiler catches missing fields, wrong types.

---

## The Real Decision Tree

```
Does your project have:
├── Reusable UI components (cards, buttons, layouts)?
│   └── YES → Azumi wins
├── Live state (user interactions, form handling)?
│   └── YES → Azumi wins
├── Multiple developers (needs compile-time enforcement)?
│   └── YES → Azumi wins
└── Simple, one-off pages with no shared components?
    └── YES → Consider simpler approach
```

---

## Hybrid Approach (Recommended)

Many projects benefit from **both**:

```rust
// SIMPLE PAGE: Use format!() strings
pub fn landing_page() -> String {
    format!(r#"..."#)
}

// SHARED COMPONENTS: Use Azumi
#[azumi::component]
pub fn ProductCard(product: &Product) -> impl Component {
    html! {
        <div class={card}>
            <h3>{product.name}</h3>
            <p>{product.description}</p>
        </div>
    }
}
```

**This is valid and recommended.** Not every page needs azumi's full feature set.

---

## What Azumi Is NOT

| Misconception | Reality |
|---------------|---------|
| "Azumi replaces all HTML writing" | ❌ Simple static pages may not benefit |
| "Azumi is like React" | ❌ Server-first, no Virtual DOM |
| "Azumi is always better than Maud" | ❌ For static strings, Maud is simpler |
| "Azumi requires everything" | ❌ Opt-in per-component |

---

## Bottom Line

| Use Case | Recommendation |
|----------|----------------|
| Blog, docs, marketing site | Maud or `format!()` |
| SaaS app with shared components | **Azumi** |
| E-commerce with cart/checkout | **Azumi** |
| Admin dashboard | **Azumi** |
| One-off landing page | `format!()` |
| Component library | **Azumi** |

**Azumi's real win is the component library** - the ability to build `ProductCard` once and use it on 20 pages, with compile-time CSS class validation, automatic escaping, and HMAC state protection.

If you don't need a component library, you don't need azumi's complexity.

---

## Summary

**Azumi costs you:**
- Verbose syntax (double-quoted CSS values)
- Strict rules (snake_case, style ordering)
- Learning curve (macro errors can be cryptic)

**Azumi gives you:**
- Reusable component library
- Compile-time CSS/HTML/a11y validation
- Automatic XSS/CSS injection prevention
- HMAC state integrity
- Zero-JS optimistic UI

**Choose azumi if the benefits matter for your project. Choose simplicity if they don't.**
