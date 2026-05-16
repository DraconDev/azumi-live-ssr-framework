use crate::examples::blog::layout::layout;
use crate::examples::blog::data::get_posts;
use azumi::prelude::*;

pub fn post_list() -> impl Component {
    let posts = get_posts();

    layout("Azumi Blog", html! {
        <h1 style="margin-bottom: 1.5rem; font-size: 2rem; color: #1a1a1a;">"Latest Posts"</h1>

        @for post in &posts {
            <article class={post_card}>
                <h2 class={post_title}>
                    <a href={format!("/blog/{}", post.slug)}>{&post.title}</a>
                </h2>
                <p class={post_meta}>
                    {&post.author}" — "{&post.date}
                </p>
                <p class={post_excerpt}>{&post.excerpt}</p>
                <div class={tag_row}>
                    @for tag in &post.tags {
                        <span class={tag}>{tag}</span>
                    }
                </div>
            </article>
        }
    })
}

pub fn post_detail(slug: &str) -> impl Component {
    let posts = get_posts();
    let post = posts.iter().find(|p| p.slug == slug);

    layout(
        post.map(|p| p.title.as_str()).unwrap_or("Post Not Found"),
        html! {
            @match post {
                Some(post) => {
                    <a class={back_link} href="/blog">"← Back to all posts"</a>
                    <article class={post_body}>
                        <h1 style="font-size: 2rem; margin-bottom: 0.5rem;">{&post.title}</h1>
                        <p class={post_meta} style="margin-bottom: 1rem;">{&post.author}" — "{&post.date}</p>
                        <div class={tag_row}>
                            @for tag in &post.tags {
                                <span class={tag}>{tag}</span>
                            }
                        </div>
                        <div style="margin-top: 1.5rem; line-height: 1.8;">
                            {raw_html(&post.content)}
                        </div>
                    </article>
                },
                None => {
                    <div style="text-align: center; padding: 4rem;">
                        <h1 style="font-size: 3rem; margin-bottom: 1rem;">"404"</h1>
                        <p style="color: #888; margin-bottom: 1.5rem;">"This post doesn't exist yet."</p>
                        <a class={btn} class:external={format!("btn {}", btn_primary)} href="/blog">"Browse all posts"</a>
                    </div>
                },
            }
        },
    )
}

pub fn about_page() -> impl Component {
    layout("About — Azumi Blog", html! {
        <div class={about_card}>
            <h1>"About This Blog"</h1>
            <p style="margin-bottom: 1rem;">"This blog is built with Azumi, a Rust web framework with compile-time validation for HTML, CSS, and JavaScript."</p>
            <p>"No JavaScript frameworks. No virtual DOM. Just Rust."</p>
        </div>
    })
}

pub fn contact_page() -> impl Component {
    layout("Contact — Azumi Blog", html! {
        <div class={contact_card}>
            <h1 style="margin-bottom: 1.5rem; text-align: center;">"Get in Touch"</h1>
            <p style="color: #888; text-align: center; margin-bottom: 1.5rem;">"Contact form coming soon."</p>
            <p style="text-align: center; color: #555;">
                "Email us at "<a href="mailto:hello@azumi.dev">"hello@azumi.dev"</a>
            </p>
        </div>
    })
}

// Axum handlers

pub async fn blog_handler() -> axum::response::Html<String> {
    axum::response::Html(azumi::render_to_string(&post_list()))
}

pub async fn post_handler(
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> axum::response::Html<String> {
    axum::response::Html(azumi::render_to_string(&post_detail(&slug)))
}

pub async fn about_handler() -> axum::response::Html<String> {
    axum::response::Html(azumi::render_to_string(&about_page()))
}

pub async fn contact_handler() -> axum::response::Html<String> {
    axum::response::Html(azumi::render_to_string(&contact_page()))
}