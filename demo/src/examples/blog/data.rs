use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub author: String,
    pub date: String,
    pub likes: u32,
    pub tags: Vec<String>,
}

/// In-memory blog posts (simulating a database)
pub fn get_posts() -> Vec<Post> {
    vec![
        Post {
            id: 1,
            slug: "introducing-azumi".to_string(),
            title: "Introducing Azumi: A Compile-Time Safe Web Framework".to_string(),
            excerpt: "Azumi brings Rust's compile-time guarantees to web development, catching XSS, CSS typos, and invalid HTML before your code ever runs.".to_string(),
            content: r#"<p>Web development has long suffered from a fundamental asymmetry: bugs like XSS, CSS typos, and invalid HTML structure are caught at <em>runtime</em> — often in production — rather than at compile time like in statically typed languages.</p>
<p>Azumi changes this by applying Rust's compile-time validation philosophy to HTML, CSS, and JavaScript generation. The <code>html!</code> macro validates your markup at compile time, blocking dangerous patterns before they can ship.</p>
<h2>Key Features</h2>
<ul>
<li><strong>Compile-time XSS prevention</strong> — The compiler catches <code>Raw()</code> in html!, <code>format!</code> building HTML, and unescaped content</li>
<li><strong>CSS validation</strong> — Class names must be defined in <code>&lt;style&gt;</code> blocks, catching typos like <code>class="btn"</code> vs <code>class="button"</code></li>
<li><strong>HTML structure rules</strong> — Blocks like <code>&lt;p&gt;</code> inside <code>&lt;p&gt;</code> are caught at compile time</li>
<li><strong>Zero-hydration rendering</strong> — Components render directly to HTML, no virtual DOM, no diffing</li>
</ul>
<p>Built on Axum with HMAC-signed state and Content Security Policy support, Azumi is production-ready today.</p>"#.to_string(),
            author: "Azumi Team".to_string(),
            date: "2026-05-10".to_string(),
            likes: 42,
            tags: vec!["rust".to_string(), "web".to_string(), "framework".to_string()],
        },
        Post {
            id: 2,
            slug: "why-rust-for-web".to_string(),
            title: "Why Rust for Web Development?".to_string(),
            excerpt: "Memory safety without garbage collection, fearless concurrency, and a type system that makes invalid states unrepresentable. Here's why Rust and web dev are a natural fit.".to_string(),
            content: r#"<p>Rust has earned its reputation as a systems language, but its principles apply equally well to web development. Let's explore why more developers are choosing Rust for their web backends and frontends.</p>
<h2>Memory Safety Without GC</h2>
<p>Unlike languages with garbage collectors, Rust guarantees memory safety at compile time through its ownership system. For web servers handling thousands of concurrent requests, this means predictable performance without pause-the-world GC delays.</p>
<h2>Fearless Concurrency</h2>
<p>The same type rules that prevent use-after-free bugs also prevent data races. WebSocket handlers, background workers, and async request handling become dramatically simpler when the compiler enforces thread safety.</p>
<h2>Invalid States Are Unrepresentable</h2>
<p>Rust's type system lets you encode business rules directly into types. A field that's either a string or absent becomes <code>Option&lt;String&gt;</code> — the compiler forces you to handle both cases.</p>
<p>Azumi extends this philosophy to the frontend, making invalid HTML structures unrepresentable at compile time.</p>"#.to_string(),
            author: "Dr. Anna Kim".to_string(),
            date: "2026-05-12".to_string(),
            likes: 28,
            tags: vec!["rust".to_string(), "philosophy".to_string()],
        },
        Post {
            id: 3,
            slug: "live-state-explained".to_string(),
            title: "Live State Explained: Reactive UI Without JavaScript Frameworks".to_string(),
            excerpt: "Azumi's live state system lets you build interactive UIs using only Rust, with the client runtime handling DOM updates automatically.".to_string(),
            content: r#"<p>React, Vue, and Svelte popularized reactive UI — where state changes automatically update the DOM. Azumi brings this to Rust, with a crucial difference: the reactive logic runs on the server, not the client.</p>
<h2>How It Works</h2>
<p>You define live state as a struct with <code>#[azumi::live]</code>:</p>
<pre><code>#[azumi::live]
struct Counter { count: i32 }</code></pre>
<p>Then use <code>state.count</code> directly in your template. When an action modifies the state, Azumi sends only the changed HTML fragment to the browser — no client-side JavaScript framework required.</p>
<h2>Optimistic Updates</h2>
<p>Before the server confirms a state change, Azumi applies a predicted update to the DOM. If the server rejects the change, it rolls back automatically. This makes UIs feel instant while maintaining server authority.</p>
<h2>The 3KB Runtime</h2>
<p>The client-side JavaScript is minimal: event delegation, DOM morphing via Idiomorph, and optimistic update rollback. No virtual DOM, no reactivity system, no framework weight.</p>"#.to_string(),
            author: "Marcus Chen".to_string(),
            date: "2026-05-14".to_string(),
            likes: 35,
            tags: vec!["rust".to_string(), "reactive".to_string(), "azumi".to_string()],
        },
    ]
}

pub fn get_post_by_slug(slug: &str) -> Option<Post> {
    get_posts().into_iter().find(|p| p.slug == slug)
}

/// Increments the like count for a post by slug and returns the new count
pub fn increment_likes(slug: &str) -> u32 {
    // In production this would be a database call.
    // Here we just return an incremented value for demo purposes.
    let posts = get_posts();
    let base = posts.iter().find(|p| p.slug == slug).map(|p| p.likes).unwrap_or(0);
    base + 1
}