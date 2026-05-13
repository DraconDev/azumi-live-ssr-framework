# Framework Comparison

> Azumi vs Next.js, SvelteKit, Astro, Leptos, HTMX, Maud, Rails

## The Five Camps

| Camp | Strategy | Examples |
|------|----------|----------|
| **Hydrators** | Send HTML, then JS to replay it | Next.js, SvelteKit, Remix |
| **App-Builders** | Download WASM, treat browser as OS | Leptos, Dioxus |
| **Purists** | Server does everything, wait for network | HTMX, Maud, Rails |
| **Islanders** | Zero JS by default, hydrate islands | Astro |
| **Optimists** | Server truth + compiler predictions | **Azumi** |

## Quick Comparison

| Dimension | Azumi | Astro | Next.js | Leptos | Svelte 5 | HTMX | Maud | Rails |
|-----------|-------|-------|---------|--------|----------|------|------|-------|
| **Language** | Rust | TypeScript | TypeScript | Rust | TypeScript | HTML | Rust | Ruby |
| **Initial JS** | **<3KB** | 0KB | ~80KB | ~150KB | ~15KB | ~14KB | 0KB | ~30KB |
| **Hydration** | **Zero** | Zero | High | High | Low | Zero | Zero | Zero |
| **Latency** | **~0ms** | Varies | Varies | ~0ms | Fast | Network RTT | Full refresh | Full refresh |
| **Type Safety** | **100% E2E** | ~80% | ~80% | 100% | ~90% | 0% | 100% server | 0% |
| **Mem Safety** | **Guaranteed** | GC | GC | **Guaranteed** | GC | N/A | N/A | N/A |
| **Security** | **Signed state** | Trust client | Trust client | Trust client | Trust client | Signed (opt) | N/A | Cookie |
| **CSS** | **Co-validated** | Scoped | Modules/Tailwind | Scoped | Scoped | Global | None | Global |
| **Ecosystem** | Small | Massive | Massive | Medium | Large | Medium | Niche | Massive |
| **AI Suitability** | **Excellent** | Good | Poor | Good | Medium | Good | Good | Medium |

## Why Each Loses (For Interactive Apps)

**Next.js:** Hydration tax — renders HTML then runs JS to rebuild it. Sends data twice.

**Leptos:** WASM download tax (~150KB), DOM bridge overhead.

**HTMX:** Network latency on every interaction. No type safety.

**Maud:** Dead end — the moment you need a toggle, you're rewriting everything.

**Astro:** Islands are isolated. No cross-island state without external stores.

**Rails:** Full page refreshes. No compile-time validation.

## Azumi's Thesis

> The server has the truth (DB). The client has the user (events). The compiler is the bridge.

By compiling strict server logic into cheap client predictions, Azumi achieves:
- **0ms latency** (optimistic)
- **0ms hydration** (HTML-native)
- **Zero type erasure** (Rust end-to-end)
- **Signed state** (security by default)

## Use Case Matrix

| Use Case | Winner |
|----------|--------|
| Documentation site | Astro |
| Marketing landing page | Tie (both excellent) |
| Dashboard / Admin panel | **Azumi** |
| E-commerce checkout | **Azumi** (signed state) |
| Real-time collaborative | Neither (WebSocket-native) |

## The AI-Native Perspective

| Old Question | New Question |
|-------------|-------------|
| "Can I `npm install` this?" | "Can the AI write this correctly?" |
| "How many GitHub stars?" | "How strict is the compiler?" |
| "Is there a tutorial?" | "Does the AI need less context?" |

Strict types = AI self-corrects via compiler errors.
One language = No context-switching overhead.
Rigid rules = Smaller search space for generation.
