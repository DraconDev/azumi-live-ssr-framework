use azumi::prelude::*;

pub fn layout(title: &str, children: impl Component) -> impl Component {
    html! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>{title}</title>
            {azumi_script()}
        </head>
        <body>
            <header class={header}>
                <nav class={nav}>
                    <a class={nav_link} href="/blog">"Blog"</a>
                    <a class={nav_link} href="/blog/about">"About"</a>
                    <a class={nav_link} href="/blog/contact">"Contact"</a>
                </nav>
            </header>

            <main class={main}>
                {children}
            </main>

            <footer class={footer}>
                <p>"Built with Azumi — compile-time safe web development"</p>
            </footer>

            <style>
            {GLOBAL_CSS}
            </style>
        </body>
        </html>
    }
}

// In-memory post state for the like counter
#[azumi::live]
pub struct PostLikes {
    counts: Vec<u32>,
}

impl Default for PostLikes {
    fn default() -> Self {
        let posts = super::data::get_posts();
        Self {
            counts: posts.iter().map(|p| p.likes).collect(),
        }
    }
}

// Shared CSS across all blog pages
const GLOBAL_CSS: &str = r#"
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; background: #fafafa; }
a { color: #0070f3; text-decoration: none; }
a:hover { text-decoration: underline; }
.header { background: #fff; border-bottom: 1px solid #eaeaea; padding: 1rem 2rem; }
.nav { display: flex; gap: 1.5rem; max-width: 800px; margin: 0 auto; }
.nav_link { color: #333; font-weight: 500; }
.nav_link:hover { color: #0070f3; text-decoration: none; }
.main { max-width: 800px; margin: 2rem auto; padding: 0 1rem; min-height: 60vh; }
.footer { text-align: center; padding: 2rem; color: #888; font-size: 0.875rem; border-top: 1px solid #eaeaea; }
.post-card { background: #fff; border: 1px solid #eaeaea; border-radius: 8px; padding: 1.5rem; margin-bottom: 1rem; transition: box-shadow 0.2s; }
.post-card:hover { box-shadow: 0 4px 12px rgba(0,0,0,0.08); }
.post-title { font-size: 1.5rem; margin-bottom: 0.5rem; }
.post-title a { color: #333; }
.post-title a:hover { color: #0070f3; }
.post-meta { color: #888; font-size: 0.875rem; margin-bottom: 0.75rem; }
.post-excerpt { color: #555; margin-bottom: 1rem; }
.btn { display: inline-block; padding: 0.5rem 1rem; border-radius: 4px; font-size: 0.875rem; cursor: pointer; border: none; transition: background 0.2s; }
.btn-primary { background: #0070f3; color: #fff; }
.btn-primary:hover { background: #0051a8; }
.tag { display: inline-block; padding: 0.125rem 0.5rem; border-radius: 999px; font-size: 0.75rem; background: #e8f0fe; color: #1a73e8; margin-right: 0.25rem; }
.tag-row { margin-bottom: 1rem; }
.post-body { background: #fff; border: 1px solid #eaeaea; border-radius: 8px; padding: 2rem; }
.post-body h2 { margin: 1.5rem 0 0.75rem; font-size: 1.25rem; }
.post-body p { margin-bottom: 1rem; }
.post-body ul, .post-body ol { margin: 1rem 0 1rem 1.5rem; }
.post-body li { margin-bottom: 0.5rem; }
.post-body code { background: #f5f5f5; padding: 0.125rem 0.375rem; border-radius: 3px; font-size: 0.875em; }
.post-body pre { background: #1e1e1e; color: #d4d4d4; padding: 1rem; border-radius: 6px; overflow-x: auto; margin: 1rem 0; }
.post-body pre code { background: none; padding: 0; color: inherit; }
.post-body em { font-style: italic; }
.post-body strong { font-weight: 600; }
.like-btn { background: #fafafa; border: 1px solid #eaeaea; color: #555; }
.like-btn:hover { background: #fff0f0; border-color: #ff4081; color: #ff4081; }
.like-btn.liked { background: #ff4081; border-color: #ff4081; color: #fff; }
.like-row { display: flex; align-items: center; gap: 0.5rem; margin-top: 1.5rem; padding-top: 1rem; border-top: 1px solid #eaeaea; }
.like-count { color: #888; font-size: 0.875rem; }
.back-link { display: inline-block; margin-bottom: 1.5rem; color: #555; font-size: 0.875rem; }
.back-link:hover { color: #0070f3; }
.form-group { margin-bottom: 1rem; }
.form-label { display: block; margin-bottom: 0.375rem; font-weight: 500; font-size: 0.875rem; color: #333; }
.form-input, .form-textarea { width: 100%; padding: 0.5rem 0.75rem; border: 1px solid #ddd; border-radius: 4px; font-size: 1rem; font-family: inherit; }
.form-input:focus, .form-textarea:focus { outline: none; border-color: #0070f3; box-shadow: 0 0 0 3px rgba(0,112,243,0.1); }
.form-textarea { min-height: 120px; resize: vertical; }
.form-error { color: #d32f2f; font-size: 0.875rem; margin-top: 0.25rem; }
.form-success { background: #e8f5e9; color: #2e7d32; padding: 0.75rem 1rem; border-radius: 4px; margin-bottom: 1rem; font-size: 0.875rem; }
.contact-card { background: #fff; border: 1px solid #eaeaea; border-radius: 8px; padding: 2rem; max-width: 500px; margin: 0 auto; }
.about-card { background: #fff; border: 1px solid #eaeaea; border-radius: 8px; padding: 2rem; max-width: 600px; margin: 0 auto; text-align: center; }
.about-card h1 { margin-bottom: 1rem; }
.about-card p { color: #555; }
"#;