use super::data::{get_post_by_slug, get_posts};
use super::layout::layout;
use azumi::prelude::*;

// ─── Post List Page ──────────────────────────────────────────────────────────

#[azumi::component]
pub fn post_list_page_inner() -> impl Component {
    let posts = get_posts();

    html! {
        <section class:external="post-list">
            <h1 class={heading}>"Latest Posts"</h1>
            @for post in &posts {
                <article class={post_card}>
                    <h2 class={post_title}>
                        <a href={format!("/blog/posts/{}", post.slug)}>{&post.title}</a>
                    </h2>
                    <div class={post_meta}>
                        {&post.author} " · " {&post.date} " · " {post.likes} " likes"
                    </div>
                    <div class={tag_row}>
                        @for tag in &post.tags {
                            <span class={tag}>{tag}</span>
                        }
                    </div>
                    <p class={post_excerpt}>{&post.excerpt}</p>
                    <a href={format!("/blog/posts/{}", post.slug)} class={read_more}>
                        "Read more →"
                    </a>
                </article>
            }
        </section>

        <style>
            .heading { font-size: "2rem"; margin-bottom: "2rem"; color: "#1a1a1a"; }
            .post_card {
                background: "#fff";
                border: "1px solid #eaeaea";
                border-radius: "8px";
                padding: "1.5rem";
                margin-bottom: "1rem";
                transition: "box-shadow 0.2s";
            }
            .post_card:hover { box-shadow: "0 4px 12px rgba(0,0,0,0.08)"; }
            .post_title { font-size: "1.5rem"; margin-bottom: "0.5rem"; }
            .post_title a { color: "#333"; }
            .post_title a:hover { color: "#0070f3"; }
            .post_meta { color: "#888"; font-size: "0.875rem"; margin-bottom: "0.75rem"; }
            .post_excerpt { color: "#555"; margin-bottom: "1rem"; }
            .tag {
                display: "inline-block";
                padding: "0.125rem 0.5rem";
                border-radius: "999px";
                font-size: "0.75rem";
                background: "#e8f0fe";
                color: "#1a73e8";
                margin-right: "0.25rem";
            }
            .tag_row { margin-bottom: "1rem"; }
            .read_more { color: "#0070f3"; font-size: "0.875rem"; font-weight: "500"; }
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
        <div class={not_found}>
            <h1>"404"</h1>
            <p>"This post doesn't exist yet."</p>
            <a href="/blog">"Browse all posts"</a>
        </div>

        <style>
            .not_found { text-align: "center"; padding: "4rem"; }
            .not_found h1 { font-size: "3rem"; margin-bottom: "1rem"; }
            .not_found p { color: "#888"; margin-bottom: "1.5rem"; }
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
                    <a class={back_link} href="/blog">"← Back to Blog"</a>
                    <article class={post_body}>
                        <h1>{&title}</h1>
                        <div class={post_meta}>
                            {&author} " · " {&date} " · " {likes} " likes"
                        </div>
                        <div class={tag_row_inner}>
                            @for tag in &tags {
                                <span class={tag}>{tag}</span>
                            }
                        </div>
                        <div class:external="post-content">
                            {TrustedHtml::from_string(content_html)}
                        </div>
                    </article>
                </div>
                <style>
                    .back_link { display: "inline-block"; margin-bottom: "1.5rem"; color: "#555"; font-size: "0.875rem"; }
                    .back_link:hover { color: "#0070f3"; }
                    .post_body {
                        background: "#fff";
                        border: "1px solid #eaeaea";
                        border-radius: "8px";
                        padding: "2rem";
                    }
                    .post_body h1 { margin-bottom: "0.5rem"; }
                    .post_body h2 { margin: "1.5rem 0 0.75rem"; font-size: "1.25rem"; }
                    .post_body p { margin-bottom: "1rem"; }
                    .post_meta { color: "#888"; font-size: "0.875rem"; margin-bottom: "0.75rem"; }
                    .tag_row_inner { margin-bottom: "1rem"; }
                    .tag {
                        display: "inline-block";
                        padding: "0.125rem 0.5rem";
                        border-radius: "999px";
                        font-size: "0.75rem";
                        background: "#e8f0fe";
                        color: "#1a73e8";
                        margin-right: "0.25rem";
                    }
                </style>
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
            } else if let Some(rest) = line.strip_prefix("## ") {
                writeln!(f, "<h2>{}</h2>", rest)?;
            } else if let Some(rest) = line.strip_prefix("### ") {
                writeln!(f, "<h3>{}</h3>", rest)?;
            } else if let Some(rest) = line.strip_prefix("- ") {
                writeln!(f, "<li>{}</li>", rest)?;
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
        <div class={contact_card}>
            <h1>"Get in Touch"</h1>
            <p>"Have a question or want to contribute? We'd love to hear from you."</p>

            <form action="/blog/contact" method="POST" az-on:submit="submit">
                <div class={form_group}>
                    <label for="name">"Your Name"</label>
                    <input type="text" name="name" id:external="name" class={form_input} />
                </div>

                <div class={form_group}>
                    <label for="email">"Email Address"</label>
                    <input type="email" name="email" id:external="email" class={form_input} />
                </div>

                <div class={form_group}>
                    <label for="message">"Message"</label>
                    <textarea name="message" id:external="message" rows="5" class={form_textarea}></textarea>
                </div>

                <button type="submit" class={submit_btn}>
                    "Send Message"
                </button>
            </form>
        </div>

        <style>
            .contact_card { background: "#fff"; border: "1px solid #eaeaea"; border-radius: "8px"; padding: "2rem"; max-width: "500px"; margin: "0 auto"; }
            .contact_card h1 { margin-bottom: "1rem"; }
            .contact_card p { color: "#555"; margin-bottom: "1.5rem"; }
            .form_group { margin-bottom: "1rem"; }
            .form_group label { display: "block"; margin-bottom: "0.375rem"; font-weight: "500"; font-size: "0.875rem"; }
            .form_input { width: "100%"; padding: "0.5rem 0.75rem"; border: "1px solid #ddd"; border-radius: "4px"; font-size: "1rem"; font-family: "inherit"; }
            .form_textarea { width: "100%"; padding: "0.5rem 0.75rem"; border: "1px solid #ddd"; border-radius: "4px"; font-size: "1rem"; font-family: "inherit"; resize: "vertical"; }
            .submit_btn { display: "inline-block"; padding: "0.5rem 1rem"; background: "#0070f3"; color: "#fff"; border: "none"; border-radius: "4px"; font-size: "0.875rem"; cursor: "pointer"; }
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
        <div class={about_card}>
            <h1>"About This Blog"</h1>
            <p>"This blog is built with Azumi, a Rust web framework with compile-time HTML/CSS/JS validation."</p>
            <p>"Azumi catches XSS vectors, CSS typos, and invalid HTML patterns at compile time — before they reach production."</p>
            <p>"This demo shows: routing, component composition, forms with action handlers, SEO metadata, and CSS scoping — all in type-safe Rust."</p>
        </div>
        <style>
            .about_card { background: "#fff"; border: "1px solid #eaeaea"; border-radius: "8px"; padding: "2rem"; max-width: "600px"; margin: "0 auto"; text-align: "center"; }
            .about_card h1 { margin-bottom: "1rem"; }
            .about_card p { color: "#555"; margin-bottom: "1rem"; }
        </style>
    }
}

pub async fn about_page() -> impl axum::response::IntoResponse {
    let content = about_page_inner();
    let html = azumi::render_to_string(&layout("About — Azumi Blog", content));
    axum::response::Html(html)
}
