# Dracon Platform Analysis: Real-World Azumi Usage

## Executive Summary

The dracon-platform is the production system that consumes azumi. It consists of 5 apps (public-site, products, dashboard, ai-api-gateway, ai-hub) plus a shared `chrome` UI library. This document analyzes actual production patterns to identify improvements for azumi.

## Architecture

```
Caddy :8080
├── public-site-app  →  /, /contact, /terms, /privacy
├── products-app     →  /products*, /licensing, /pricing
├── dashboard-app    →  /dashboard/*, /api/v1/auth*
├── ai-api-gateway  →  /v1/ai/*
└── ai-hub          →  /ai-hub/*

External daemons (dracon-demons repo):
├── auth-daemon     →  tarpc on /run/dracon/auth.sock
├── billing-daemon  →  tarpc on /run/dracon/billing.sock
├── email-daemon    →  tarpc on /run/dracon/email.sock
├── ai-daemon       →  tarpc on /run/dracon/ai.sock
└── bucket-daemon   →  tarpc on /run/dracon/bucket.sock
```

## Chrome Crate Structure

Shared UI primitives in `libs/chrome/`:

| Module | Purpose | Lines |
|--------|---------|-------|
| `base_css.rs` | `BaseStyles` component with `<style global>` | 224 |
| `components/base.rs` | Base UI components (ProductCard, PricingTierCard, etc.) | ~800 |
| `components/pages.rs` | Page components (HomePage, ProductsPage, etc.) | 906 |
| `render.rs` | All `render_*_document()` functions | 929 |
| `nav.rs` | `NavBar`, `NavLink` | ~200 |
| `footer.rs` | `Footer`, `FooterColumns` | ~150 |
| `site_content.rs` | Site content as Rust constants | ~400 |
| `products.rs` | Product catalog loader + manifest builders | ~200 |
| `ai_providers.rs` | AI provider types + catalog | ~600 |

## Production Patterns Observed

### 1. Component Prop Explosion

```rust
#[component]
pub fn page_app_layout(
    title: String,
    description: Option<String>,
    canonical_path: Option<String>,
    _page_route: Option<String>,
    ga_id: Option<String>,
    noindex: bool,
    logo_label: String,
    logo_href: String,
    logo_gradient: bool,
    links: Vec<NavLink>,
    user: Option<UserInfo>,
    dashboard_url: String,
    public_url: String,
    footer_app_name: String,
    current_path: String,
    extra_head_html: Option<String>,
    include_scroll_reveal: bool,
    include_back_to_top: bool,
    children: impl azumi::Component,
) -> impl azumi::Component {
```

**Finding**: `page_app_layout` has 19 parameters. This is a pattern across the codebase.

**Impact**: Clutters component signatures, makes call sites verbose.

**Recommendation for azumi**: Consider a builder pattern or context-based props to reduce parameter count.

### 2. Raw HTML Escape Hatch in Production

```rust
let extra_head_raw = extra_head_html.as_ref().map(|extra| azumi::Raw(extra.clone()));
html! {
    @if let Some(extra) = &extra_head_raw {
        {extra}
    }
}
```

**Finding**: `azumi::Raw` is actively used in production for injecting arbitrary HTML (Google Analytics scripts, custom head content).

**Impact**: Bypasses all of azumi's safety guarantees. XSS risk if user-controlled data reaches this path.

**Recommendation for azumi**: 
- Add `TrustedHtml` type with explicit sanitization requirements
- Document that `Raw` should only be used with trusted/static content
- Consider a `ScriptSrc`/`StyleSrc` helper for common injection patterns

### 3. Global CSS Pattern

```rust
#[component]
pub fn BaseStyles() -> impl azumi::Component {
    html! {
        <style global>
            :root {
                --bg_primary: "#000000";
                --bg_secondary: "#050505";
                // ... 40+ variables
            }
            * { margin: "0"; padding: "0"; box-sizing: "border-box"; }
            html { scroll-behavior: "smooth"; }
            // ... 150+ lines of global CSS
        </style>
    }
}
```

**Finding**: `BaseStyles` component injects ~224 lines of global CSS via `<style global>`.

**Impact**: Every page includes this CSS inline in the HTML response. No CSS file caching.

**Recommendation for azumi**:
- Consider CSS extraction to static files in production builds
- Add CSS minification for `<style global>` blocks
- Document the trade-off between inline CSS (no HTTP request) vs cached CSS file

### 4. SVG Icons Inline

```rust
@if i == 0 {
    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2"/><path d="M8 21h8M12 17v4"/></svg>
}
```

**Finding**: SVG icons are inlined directly in component HTML. No icon library or sprite sheet.

**Impact**: HTML bloat, repeated SVG paths across pages.

**Recommendation for azumi**:
- Add SVG sprite sheet support or `<use href>` pattern
- Consider an `<Icon>` component that references a sprite

### 5. Mobile-First Responsive Design

The platform follows strict mobile-first principles:
- Base CSS = mobile layout
- Desktop enhancements via `@media (min-width: 640px)` etc.
- Visual-first on mobile (images above text)
- Never use `max-width` as primary strategy

**Breakpoints**:
| Width | Purpose |
|-------|---------|
| < 640px | Mobile (single column, compact) |
| 640px+ | 2-col cards, pricing grid |
| 900px+ | Hero 2-col, How It Works 3-col |
| 1100px+ | Wider gaps, relaxed fonts |

### 6. Data-Driven Pages

Products and AI providers are data-driven from JSON:
- `libs/chrome/data/products-catalog.json`
- `libs/chrome/data/ai-providers-catalog.json`

Pages render from these catalogs rather than hardcoded HTML.

## Pain Points Identified

### High Priority

1. **Component prop explosion** — `page_app_layout` has 19 params, `HomePage` has 3+ params. Builder pattern needed.

2. **No CSS caching** — All CSS is inline. No extracted CSS files for caching.

3. **Raw HTML usage** — `azumi::Raw` bypasses safety. Needs safer alternatives.

### Medium Priority

4. **SVG duplication** — Icons inlined repeatedly. Needs sprite sheet.

5. **No component-level CSS scoping evidence** — All CSS appears global or page-scoped. No evidence of component-level scoped CSS usage.

6. **Cluttered component signatures** — Many components take `content: SomeContent` structs instead of named params.

## Opportunities for Azumi Improvement

### 1. Builder Pattern for Components

```rust
// Current (verbose)
page_app_layout(
    title, description, canonical_path, _page_route, ga_id,
    noindex, logo_label, logo_href, logo_gradient, links, user,
    dashboard_url, public_url, footer_app_name, current_path,
    extra_head_html, include_scroll_reveal, include_back_to_top,
    children
)

// Proposed
PageLayout::builder()
    .title(title)
    .description(description)
    .canonical(canonical_path)
    .ga_id(ga_id)
    .noindex(noindex)
    .logo(logo_label, logo_href, logo_gradient)
    .nav(links)
    .user(user)
    .footer(footer_app_name)
    .extra_head(extra_head_html)
    .scroll_reveal(include_scroll_reveal)
    .back_to_top(include_back_to_top)
    .render(children)
```

### 2. Safe HTML Injection Helpers

```rust
// Instead of Raw
html! {
    <script async="async" src={url.clone()}></script>
}

// Proposed: type-safe script injection
html! {
    <script src={azumi::ScriptSrc::from_url(url)}></script>
}
```

### 3. CSS Extraction for Production

```rust
// Build-time extraction
#[component]
pub fn BaseStyles() -> impl azumi::Component {
    html! {
        <style global extract="base.css">
            // ... CSS
        </style>
    }
}
// Renders as: <link rel="stylesheet" href="/static/base.css">
```

### 4. Component-Level Scoping

```rust
#[component]
pub fn PricingCard() -> impl azumi::Component {
    html! {
        <style scoped>
            .card { border: "1px solid"; }
            .price { font-size: "2rem"; }
        </style>
        <div class="card">
            <div class="price">{price}</div>
        </div>
    }
}
// Scoped to data-azumi-scope automatically
```

## Conclusion

The dracon-platform validates azumi's core value proposition: type-safe HTML templating with compile-time validation works at scale. However, production usage reveals opportunities for:

1. **Builder patterns** to manage component complexity
2. **Safe injection helpers** to reduce `Raw` usage
3. **CSS extraction** for production caching
4. **SVG/icon helpers** to reduce duplication
5. **Better documentation** of mobile-first responsive patterns

The platform serves 5 apps with ~3000 lines of component code across `chrome`, demonstrating azumi's viability for real-world production use.
