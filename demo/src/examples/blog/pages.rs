use super::data::{get_post_by_slug, get_posts, Post};
use super::layout::layout;
use azumi::prelude::*;

// ─── Post List Page ──────────────────────────────────────────────────────────

#[azumi::component]
pub fn post_list_page_inner() -> impl Component {
    let posts = get_posts();

    html! {
        <section class="post-list">
            <h1 style="font-size: 2rem; margin-bottom: 2rem; color: #1a1a1a;">"Latest Posts"</h1>
            @for post in &posts {
                <article class="post-card">
                    <h2 class="post-title">
                        <a href={format!("/blog/posts/{}", post.slug)}>{&post.title}</a>
                    </h2>
                    <div class="post-meta">
                        {&post.author} " · " {&post.date} " · " {post.likes} " likes"
                    </div>
                    <div class="tag-row">
                        @for tag in &post.tags {
                            <span class="tag">{tag}</span>
                        }
                    </div>
                    <p class="post-excerpt">{&post.excerpt}</p>
                    <a href={format!("/blog/posts/{}", post.slug)}
                       style="color: #0070f3; font-size: 0.875rem; font-weight: 500;">
                        "Read more →"
                    </a>
                </article>
            }
        </section>
    }
}

pub async fn post_list_page() -> impl axum::response::IntoResponse {
    let content = post_list_page_inner();
    let html = azumi::render_to_string(&layout("Blog — Azumi", content));
    axum::response::Html(html)
}

// ─── Single Post Page ───────────────────────────────────────────────────────

#[azumi::component]
pub fn post_page_inner(slug: &str) -> impl Component {
    let post = get_post_by_slug(slug);

    let not_found = html! {
        <div style="text-align: center; padding: 4rem;">
            <h1 style="font-size: 3rem; margin-bottom: 1rem;">"404"</h1>
            <p style="color: #888; margin-bottom: 1.5rem;">"This post doesn't exist yet."</p>
            <a href="/blog" style="color: #0070f3; font-size: 0.875rem; font-weight: 500;">"Browse all posts"</a>
        </div>
    };

    match post {
        Some(p) => html! {
            <>
                <a class="back-link" href="/blog">"← Back to Blog"</a>
                <article class="post-body">
                    <h1 style="font-size: 2rem; margin-bottom: 0.5rem;">{&p.title}</h1>
                    <div class="post-meta" style="margin-bottom: 1.5rem;">
                        {&p.author} " · " {&p.date} " · " {p.likes} " likes"
                    </div>
                    <div class="tag-row" style="margin-bottom: 1.5rem;">
                        @for tag in &p.tags {
                            <span class="tag">{tag}</span>
                        }
                    </div>
                    <div class="post-content">
                        {PostContent(&p.content)}
                    </div>
                </article>
            </>
        },
        None => not_found,
    }
}

pub async fn post_page(slug: axum::extract::Path<String>) -> impl axum::response::IntoResponse {
    let slug = slug.into_inner();
    let content = post_page_inner(&slug);
    let html = azumi::render_to_string(&layout("Post — Azumi Blog", content));
    axum::response::Html(html)
}

/// Renders post content — simple HTML passthrough for demo
struct PostContent(&'static str);

impl std::fmt::Display for PostContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.lines() {
            if line.is_empty() {
                writeln!(f, "<br>")?;
            } else if line.starts_with("## ") {
                writeln!(f, "<h2>{}</h2>", &line[3..])?;
            } else if line.starts_with("### ") {
                writeln!(f, "<h3>{}</h3>", &line[4..])?;
            } else if line.starts_with("- ") {
                writeln!(f, "<li>{}</li>", &line[2..])?;
            } else {
                writeln!(f, "<p>{}</p>", line)?;
            }
        }
        Ok(())
    }
}

// ─── Contact Page ────────────────────────────────────────────────────────────

#[azumi::component]
pub fn contact_page_inner() -> impl Component {
    html! {
        <div class="contact-card">
            <h1 style="margin-bottom: 1.5rem; font-size: 1.75rem;">"Get in Touch"</h1>
            <p style="color: #555; margin-bottom: 1.5rem;">
                "Have a question or want to contribute? We'd love to hear from you."
            </p>

            <form action="/blog/contact" method="POST" az-on:submit="submit">
                <div style="margin-bottom: 1rem;">
                    <label style="display: block; margin-bottom: 0.375rem; font-weight: 500; font-size: 0.875rem;" for="name">"Your Name"</label>
                    <input type="text" name="name" id="name"
                           style="width: 100%; padding: 0.5rem 0.75rem; border: 1px solid #ddd; border-radius: 4px; font-size: 1rem; font-family: inherit;" />
                </div>

                <div style="margin-bottom: 1rem;">
                    <label style="display: block; margin-bottom: 0.375rem; font-weight: 500; font-size: 0.875rem;" for="email">"Email Address"</label>
                    <input type="email" name="email" id="email"
                           style="width: 100%; padding: 0.5rem 0.75rem; border: 1px solid #ddd; border-radius: 4px; font-size: 1rem; font-family: inherit;" />
                </div>

                <div style="margin-bottom: 1rem;">
                    <label style="display: block; margin-bottom: 0.375rem; font-weight: 500; font-size: 0.875rem;" for="message">"Message"</label>
                    <textarea name="message" id="message" rows="5"
                              style="width: 100%; padding: 0.5rem 0.75rem; border: 1px solid #ddd; border-radius: 4px; font-size: 1rem; font-family: inherit; resize: vertical;"></textarea>
                </div>

                <button type="submit"
                        style="display: inline-block; padding: 0.5rem 1rem; background: #0070f3; color: #fff; border: none; border-radius: 4px; font-size: 0.875rem; cursor: pointer;">
                    "Send Message"
                </button>
            </form>
        </div>
    }
}

pub async fn contact_page() -> impl axum::response::IntoResponse {
    let content = contact_page_inner();
    let html = azumi::render_to_string(&layout("Contact — Azumi Blog", content));
    axum::response::Html(html)
}

// ─── About Page ──────────────────────────────────────────────────────────────

#[azumi::component]
pub fn about_page_inner() -> impl Component {
    html! {
        <div class="about-card">
            <h1 style="margin-bottom: 1rem;">"About This Blog"</h1>
            <p style="color: #555; margin-bottom: 1rem;">
                "This blog is built with Azumi, a Rust web framework with compile-time HTML/CSS/JS validation."
            </p>
            <p style="color: #555; margin-bottom: 1rem;">
                "Azumi catches XSS vectors, CSS typos, and invalid HTML patterns at compile time — before they reach production."
            </p>
            <p style="color: #555;">
                "This demo shows: routing, component composition, forms with action handlers, SEO metadata, and CSS scoping — all in type-safe Rust."
            </p>
        </div>
    }
}

pub async fn about_page() -> impl axum::response::IntoResponse {
    let content = about_page_inner();
    let html = azumi::render_to_string(&layout("About — Azumi Blog", content));
    axum::response::Html(html)
}