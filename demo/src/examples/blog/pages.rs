use super::data::{get_post_by_slug, get_posts};
use super::layout::layout;
use azumi::prelude::*;

// ─── Post List Page ──────────────────────────────────────────────────────────

#[azumi::component]
pub fn post_list_page_inner() -> impl Component {
    let posts = get_posts();

    html! {
        <section class:external="post-list">
            <h1>"Latest Posts"</h1>
            @for post in &posts {
                <article class:external="post-card">
                    <h2 class:external="post-title">
                        <a href={format!("/blog/posts/{}", post.slug)}>{&post.title}</a>
                    </h2>
                    <div class:external="post-meta">
                        {&post.author} " · " {&post.date} " · " {post.likes} " likes"
                    </div>
                    <div class:external="tag-row">
                        @for tag in &post.tags {
                            <span class:external="tag">{tag}</span>
                        }
                    </div>
                    <p class:external="post-excerpt">{&post.excerpt}</p>
                    <a href={format!("/blog/posts/{}", post.slug)} class:external="read-more">
                        "Read more →"
                    </a>
                </article>
            }
        </section>

        <style>
            .post-list h1 {
                font-size: 2rem;
                margin-bottom: 2rem;
                color: #1a1a1a;
            }
            .read-more {
                color: #0070f3;
                font-size: 0.875rem;
                font-weight: 500;
            }
        </style>
    }
}

pub async fn post_list_page() -> impl axum::response::IntoResponse {
    let content = post_list_page_inner();
    let html = azumi::render_to_string(&layout("Blog — Azumi", content));
    axum::response::Html(html)
}

// ─── Single Post Page ────────────────────────────────────────────────────────

#[azumi::component]
pub fn post_page_inner(slug: String) -> Box<dyn Component> {
    let post = get_post_by_slug(&slug);

    let not_found = html! {
        <div class:external="not-found">
            <h1>"404"</h1>
            <p>"This post doesn't exist yet."</p>
            <a href="/blog">"Browse all posts"</a>
        </div>

        <style>
            .not-found {
                text-align: center;
                padding: 4rem;
            }
            .not-found h1 {
                font-size: 3rem;
                margin-bottom: 1rem;
            }
            .not-found p {
                color: #888;
                margin-bottom: 1.5rem;
            }
        </style>
    };

    match post {
        Some(p) => {
            let title = p.title.clone();
            let author = p.author.clone();
            let date = p.date.clone();
            let likes = p.likes;
            let tags = p.tags.clone();
            // Pre-render post content to owned String to avoid lifetime issues with Box<dyn Component>
            let content_html = PostContent(&p.content).to_string();
            Box::new(html! {
                <div>
                    <a class:external="back-link" href="/blog">"← Back to Blog"</a>
                    <article class:external="post-body">
                        <h1>{&title}</h1>
                        <div class:external="post-meta">
                            {&author} " · " {&date} " · " {likes} " likes"
                        </div>
                        <div class:external="tag-row">
                            @for tag in &tags {
                                <span class:external="tag">{tag}</span>
                            }
                        </div>
                        <div class:external="post-content">
                            {content_html}
                        </div>
                    </article>
                </div>
            })
        }
        None => Box::new(not_found),
    }
}

pub async fn post_page(axum::extract::Path(slug): axum::extract::Path<String>) -> impl axum::response::IntoResponse {
    let content = post_page_inner::render(
        post_page_inner::Props::builder()
            .slug(slug)
            .build()
            .expect("missing slug prop"),
    );
    let html = azumi::render_to_string(&layout("Post — Azumi Blog", content));
    axum::response::Html(html)
}

/// Renders post content — simple HTML passthrough for demo
struct PostContent<'a>(&'a str);

impl<'a> std::fmt::Display for PostContent<'a> {
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
        <div class:external="contact-card">
            <h1>"Get in Touch"</h1>
            <p>"Have a question or want to contribute? We'd love to hear from you."</p>

            <form action="/blog/contact" method="POST" az-on:submit="submit">
                <div class:external="form-group">
                    <label for="name">"Your Name"</label>
                    <input type="text" name="name" id:external="name" class:external="form-input" />
                </div>

                <div class:external="form-group">
                    <label for="email">"Email Address"</label>
                    <input type="email" name="email" id:external="email" class:external="form-input" />
                </div>

                <div class:external="form-group">
                    <label for="message">"Message"</label>
                    <textarea name="message" id:external="message" rows="5" class:external="form-textarea"></textarea>
                </div>

                <button type="submit" class:external="submit-btn">
                    "Send Message"
                </button>
            </form>
        </div>

        <style>
            .form-group {
                margin-bottom: 1rem;
            }
            .form-group label {
                display: block;
                margin-bottom: 0.375rem;
                font-weight: 500;
                font-size: 0.875rem;
            }
            .form-input, .form-textarea {
                width: 100%;
                padding: 0.5rem 0.75rem;
                border: 1px solid #ddd;
                border-radius: 4px;
                font-size: 1rem;
                font-family: inherit;
            }
            .form-textarea {
                resize: vertical;
            }
            .submit-btn {
                display: inline-block;
                padding: 0.5rem 1rem;
                background: #0070f3;
                color: #fff;
                border: none;
                border-radius: 4px;
                font-size: 0.875rem;
                cursor: pointer;
            }
        </style>
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
        <div class:external="about-card">
            <h1>"About This Blog"</h1>
            <p>"This blog is built with Azumi, a Rust web framework with compile-time HTML/CSS/JS validation."</p>
            <p>"Azumi catches XSS vectors, CSS typos, and invalid HTML patterns at compile time — before they reach production."</p>
            <p>"This demo shows: routing, component composition, forms with action handlers, SEO metadata, and CSS scoping — all in type-safe Rust."</p>
        </div>
    }
}

pub async fn about_page() -> impl axum::response::IntoResponse {
    let content = about_page_inner();
    let html = azumi::render_to_string(&layout("About — Azumi Blog", content));
    axum::response::Html(html)
}