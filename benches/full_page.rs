use azumi::html;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_render_large_page(c: &mut Criterion) {
    c.bench_function("render_large_page_500_elements", |b| {
        b.iter(|| {
            let component = html! {
                <div class={page}>
                    <header class={header}>
                        <nav class={nav}>
                            <a href="/" class={logo}>"Azumi"</a>
                            <div class={links}>
                                <a href="/about">"About"</a>
                                <a href="/docs">"Docs"</a>
                                <a href="/blog">"Blog"</a>
                            </div>
                        </nav>
                    </header>
                    <main class={main}>
                        <section class={hero}>
                            <h1>"Fast, type-safe HTML templates"</h1>
                            <p>"Zero hydration. Zero WASM. Full Rust."</p>
                            <div class={actions}>
                                <a href="/start" class={btn_primary}>"Get Started"</a>
                                <a href="/demo" class={btn_secondary}>"View Demo"</a>
                            </div>
                        </section>
                        <section class={features}>
                            <article class={card}>
                                <h3>"Compile-Time Validation"</h3>
                                <p>"CSS classes, HTML structure, and accessibility checked before your code ships."</p>
                            </article>
                            <article class={card}>
                                <h3>"Tiny Runtime"</h3>
                                <p>"~3KB for interactive pages. Zero bytes for static pages."</p>
                            </article>
                            <article class={card}>
                                <h3>"Type-Safe State"</h3>
                                <p>"HMAC-signed state prevents tampering. No trust-the-client vulnerabilities."</p>
                            </article>
                        </section>
                        <section class={pricing}>
                            <div class={tier}>
                                <h3>"Starter"</h3>
                                <p class={price}>"$0"</p>
                                <ul>
                                    <li>"Unlimited pages"</li>
                                    <li>"Community support"</li>
                                    <li>"Open source"</li>
                                </ul>
                            </div>
                            <div class={tier}>
                                <h3>"Pro"</h3>
                                <p class={price}>"$29/mo"</p>
                                <ul>
                                    <li>"Everything in Starter"</li>
                                    <li>"Priority support"</li>
                                    <li>"Custom domains"</li>
                                </ul>
                            </div>
                            <div class={tier}>
                                <h3>"Enterprise"</h3>
                                <p class={price}>"Custom"</p>
                                <ul>
                                    <li>"Everything in Pro"</li>
                                    <li>"SLA guarantee"</li>
                                    <li>"Dedicated support"</li>
                                </ul>
                            </div>
                        </section>
                    </main>
                    <footer class={footer}>
                        <p>"Azumi Framework"</p>
                        <div class={footer_links}>
                            <a href="/terms">"Terms"</a>
                            <a href="/privacy">"Privacy"</a>
                            <a href="/contact">"Contact"</a>
                        </div>
                    </footer>
                </div>
                <style>
                    .page { font-family: "system-ui, sans-serif"; max-width: "1200px"; margin: "0 auto"; padding: "0 1rem"; }
                    .header { border-bottom: "1px solid #eee"; padding: "1rem 0"; }
                    .nav { display: "flex"; justify-content: "space-between"; align-items: "center"; }
                    .logo { font-weight: "800"; font-size: "1.5rem"; text-decoration: "none"; }
                    .links { display: "flex"; gap: "1.5rem"; }
                    .main { padding: "2rem 0"; }
                    .hero { text-align: "center"; padding: "4rem 0"; }
                    .hero h1 { font-size: "3rem"; margin: "0 0 1rem"; }
                    .hero p { font-size: "1.25rem"; color: "#666"; margin: "0 0 2rem"; }
                    .actions { display: "flex"; gap: "1rem"; justify-content: "center"; }
                    .btn_primary { background: "#0070f3"; color: "white"; padding: "0.75rem 1.5rem"; border-radius: "0.5rem"; text-decoration: "none"; }
                    .btn_secondary { border: "1px solid #0070f3"; color: "#0070f3"; padding: "0.75rem 1.5rem"; border-radius: "0.5rem"; text-decoration: "none"; }
                    .features { display: "grid"; grid-template-columns: "repeat(3, 1fr)"; gap: "2rem"; padding: "4rem 0"; }
                    .card { border: "1px solid #eee"; padding: "2rem"; border-radius: "0.5rem"; }
                    .card h3 { margin: "0 0 1rem"; }
                    .pricing { display: "grid"; grid-template-columns: "repeat(3, 1fr)"; gap: "2rem"; padding: "4rem 0"; }
                    .tier { border: "1px solid #eee"; padding: "2rem"; border-radius: "0.5rem"; text-align: "center"; }
                    .price { font-size: "2rem"; font-weight: "800"; margin: "1rem 0"; }
                    .footer { border-top: "1px solid #eee"; padding: "2rem 0"; display: "flex"; justify-content: "space-between"; }
                    .footer_links { display: "flex"; gap: "1.5rem"; }
                </style>
            };
            black_box(azumi::render_to_string(&component));
        })
    });
}

fn bench_render_repeated_list(c: &mut Criterion) {
    c.bench_function("render_list_100_items", |b| {
        b.iter(|| {
            let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
            let component = html! {
                <ul class={list}>
                    @for item in items.iter() {
                        <li class={item_class}>{item.clone()}</li>
                    }
                </ul>
                <style>
                    .list { list-style: "none"; padding: "0"; }
                    .item_class { padding: "0.5rem"; border-bottom: "1px solid #eee"; }
                </style>
            };
            black_box(azumi::render_to_string(&component));
        })
    });
}

fn bench_render_many_attributes(c: &mut Criterion) {
    c.bench_function("render_20_attributes", |b| {
        b.iter(|| {
            let component = html! {
                <div
                    id={"test"}
                    class={"container"}
                    data-id={"123"}
                    data-role={"main"}
                    data-section={"hero"}
                    aria-label={"Main content"}
                    aria-describedby={"desc"}
                    tabindex={"0"}
                    role={"region"}
                    lang={"en"}
                    dir={"ltr"}
                    hidden={false}
                    draggable={false}
                    contenteditable={false}
                    translate={"no"}
                    spellcheck={false}
                    autocapitalize={"off"}
                    inputmode={"text"}
                    enterkeyhint={"done"}
                    itemscope={true}
                    itemtype={"https://schema.org/WebPage"}
                >
                    "Content"
                </div>
            };
            black_box(azumi::render_to_string(&component));
        })
    });
}

criterion_group!(
    benches,
    bench_render_large_page,
    bench_render_repeated_list,
    bench_render_many_attributes
);
criterion_main!(benches);
