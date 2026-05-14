use azumi::html;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_render_simple_div(c: &mut Criterion) {
    c.bench_function("render_simple_div", |b| {
        b.iter(|| {
            let component = html! { <div>"Hello"</div> };
            black_box(azumi::render_to_string(&component));
        })
    });
}

fn bench_render_with_style(c: &mut Criterion) {
    c.bench_function("render_with_style", |b| {
        b.iter(|| {
            let component = html! {
                <div class={my_class}>
                    "Hello"
                </div>
                <style>
                    .my_class { color: "red"; }
                </style>
            };
            black_box(azumi::render_to_string(&component));
        })
    });
}

fn bench_render_with_json_data(c: &mut Criterion) {
    let data = serde_json::json!({"count": 42, "name": "test"});
    c.bench_function("render_with_json_data", |b| {
        b.iter(|| {
            let data = data.clone();
            let component = html! {
                <div>"Hello"</div>
                {azumi::json_data!("APP_DATA" = &data)}
            };
            black_box(azumi::render_to_string(&component));
        })
    });
}

fn bench_render_nested_components(c: &mut Criterion) {
    c.bench_function("render_nested_10_levels", |b| {
        b.iter(|| {
            let component = html! {
                <div>
                    <div>
                        <div>
                            <div>
                                <div>
                                    <div>
                                        <div>
                                            <div>
                                                <div>
                                                    <div>"Deep"</div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            };
            black_box(azumi::render_to_string(&component));
        })
    });
}

criterion_group!(
    benches,
    bench_render_simple_div,
    bench_render_with_style,
    bench_render_with_json_data,
    bench_render_nested_components,
    bench_render_1000_components,
    bench_render_concurrent,
    bench_render_to_writer_vs_string
);

fn bench_render_1000_components(c: &mut Criterion) {
    c.bench_function("render_1000_simple_divs", |b| {
        b.iter(|| {
            let mut output = String::with_capacity(20_000);
            for _ in 0..1000 {
                let component = html! { <div>"Hello"</div> };
                output.push_str(&azumi::render_to_string(&component));
            }
            black_box(&output);
        })
    });
}

fn bench_render_concurrent(c: &mut Criterion) {
    use std::thread;
    c.bench_function("render_concurrent_8_threads", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..8)
                .map(|_| {
                    thread::spawn(|| {
                        let mut output = String::with_capacity(10_000);
                        for _ in 0..125 {
                            let component = html! { <div>"Hello"</div> };
                            output.push_str(&azumi::render_to_string(&component));
                        }
                        output
                    })
                })
                .collect();
            let results: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();
            black_box(&results);
        })
    });
}
