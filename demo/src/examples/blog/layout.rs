use azumi::prelude::*;

pub fn layout<T: Component>(title: &str, children: T) -> impl Component + use<'_, T> {
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
            <header class:external="header">
                <nav class:external="nav">
                    <a class:external="nav_link" href="/blog">"Blog"</a>
                    <a class:external="nav_link" href="/blog/about">"About"</a>
                    <a class:external="nav_link" href="/blog/contact">"Contact"</a>
                </nav>
            </header>

            <main class:external="main">
                {children}
            </main>

            <footer class:external="footer">
                <p>"Built with Azumi — compile-time safe web development"</p>
            </footer>

            <style>
            {GLOBAL_CSS}
            </style>
        </body>
        </html>
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
.back-link { display: inline-block; margin-bottom: 1.5rem; color: #555; font-size: 0.875rem; }
.back-link:hover { color: #0070f3; }
.contact-card { background: #fff; border: 1px solid #eaeaea; border-radius: 8px; padding: 2rem; max-width: 500px; margin: 0 auto; }
.about-card { background: #fff; border: 1px solid #eaeaea; border-radius: 8px; padding: 2rem; max-width: 600px; margin: 0 auto; text-align: center; }
.about-card h1 { margin-bottom: 1rem; }
.about-card p { color: #555; }
"#;