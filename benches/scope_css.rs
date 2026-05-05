use azumi::scope_css;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_scope_css_small(c: &mut Criterion) {
    let css = ".btn { color: red; } .card { padding: 1rem; }";
    c.bench_function("scope_css_small", |b| {
        b.iter(|| scope_css(black_box(css), black_box("sabc123")))
    });
}

fn bench_scope_css_medium(c: &mut Criterion) {
    let css = r#"
        .btn { color: red; }
        .card { padding: 1rem; }
        .header { font-size: "2rem"; }
        @media (min-width: 768px) {
            .sidebar { width: "250px"; }
        }
        .footer { margin-top: "2rem"; }
    "#;
    c.bench_function("scope_css_medium", |b| {
        b.iter(|| scope_css(black_box(css), black_box("sabc123")))
    });
}

fn bench_scope_css_large(c: &mut Criterion) {
    let mut css = String::new();
    for i in 0..1000 {
        css.push_str(&format!(".class_{i} {{ color: red; padding: 1rem; margin: 2rem; }}\n"));
    }
    c.bench_function("scope_css_large_1000_rules", |b| {
        b.iter(|| scope_css(black_box(&css), black_box("sabc123")))
    });
}

fn bench_scope_css_with_media_queries(c: &mut Criterion) {
    let css = r#"
        @media (min-width: 768px) {
            .sidebar { width: "250px"; }
            .main { margin-left: "250px"; }
        }
        @supports (display: grid) {
            .grid { display: grid; }
        }
        @keyframes slide {
            from { transform: translateX(0); }
            to { transform: translateX(100%); }
        }
    "#;
    c.bench_function("scope_css_media_queries", |b| {
        b.iter(|| scope_css(black_box(css), black_box("sabc123")))
    });
}

criterion_group!(
    benches,
    bench_scope_css_small,
    bench_scope_css_medium,
    bench_scope_css_large,
    bench_scope_css_with_media_queries
);
criterion_main!(benches);
