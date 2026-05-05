use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_escape_script_small(c: &mut Criterion) {
    let input = "console.log('hello'); </script>";
    c.bench_function("escape_script_small", |b| {
        b.iter(|| azumi::escape_script_content(black_box(input)))
    });
}

fn bench_escape_script_large(c: &mut Criterion) {
    let base = "console.log('x');";
    let input = format!("{}{}", base.repeat(100_000), "</script>");
    c.bench_function("escape_script_large_1_7mb", |b| {
        b.iter(|| azumi::escape_script_content(black_box(&input)))
    });
}

fn bench_escape_style_small(c: &mut Criterion) {
    let input = ".btn { color: red; } </style>";
    c.bench_function("escape_style_small", |b| {
        b.iter(|| azumi::escape_style_content(black_box(input)))
    });
}

fn bench_escape_style_large(c: &mut Criterion) {
    let base = ".my_class { color: red; }";
    let input = format!("{}{}", base.repeat(50_000), "</style>");
    c.bench_function("escape_style_large_1_2mb", |b| {
        b.iter(|| azumi::escape_style_content(black_box(&input)))
    });
}

fn bench_escape_no_match(c: &mut Criterion) {
    let input = "hello world no closing tag here just normal text";
    c.bench_function("escape_script_no_match", |b| {
        b.iter(|| azumi::escape_script_content(black_box(input)))
    });
}

criterion_group!(
    benches,
    bench_escape_script_small,
    bench_escape_script_large,
    bench_escape_style_small,
    bench_escape_style_large,
    bench_escape_no_match
);
criterion_main!(benches);
