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
                * { box-sizing: "border-box"; margin: "0"; padding: "0"; }
                body { font-family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif"; line-height: "1.6"; color: "#333"; background: "#fafafa"; }
                a { color: "#0070f3"; text-decoration: "none"; }
                a:hover { text-decoration: "underline"; }
            </style>
            <style>
                .header { background: "#fff"; border-bottom: "1px solid #eaeaea"; padding: "1rem 2rem"; }
                .nav { display: "flex"; gap: "1.5rem"; max-width: "800px"; margin: "0 auto"; }
                .nav_link { color: "#333"; font-weight: "500"; }
                .nav_link:hover { color: "#0070f3"; text-decoration: "none"; }
                .main { max-width: "800px"; margin: "2rem auto"; padding: "0 1rem"; min-height: "60vh"; }
                .footer { text-align: "center"; padding: "2rem"; color: "#888"; font-size: "0.875rem"; border-top: "1px solid #eaeaea"; }
            </style>
        </body>
        </html>
    }
}
