# Benchmark Baselines — v47.0.0

## escape (benches/escape.rs)

| Case | Time |
|------|------|
| `escape_script_small` | 345.99 ns – 354.73 ns |
| `escape_script_large_1.7mb` | 2.2516 ms – 2.2855 ms |
| `escape_style_small` | 366.98 ns – 372.27 ns |
| `escape_style_large_1.2mb` | 1.6548 ms – 1.8056 ms |
| `escape_script_no_match` | 383.80 ns – 389.09 ns |

## render (benches/render.rs)

| Case | Time |
|------|------|
| `render_simple_div` | 76.052 ns – 84.281 ns |
| `render_with_style` | 202.86 ns – 224.00 ns |
| `render_with_json_data` | 702.67 ns – 776.10 ns |
| `render_nested_10_levels` | 222.42 ns – 248.04 ns |

## scope_css (benches/scope_css.rs)

| Case | Time |
|------|------|
| `scope_css_small` | 402.14 ns – 441.29 ns |
| `scope_css_medium` | 1.4350 µs – 1.5822 µs |
| `scope_css_large_1000_rules` | 286.52 µs – 306.98 µs |
| `scope_css_media_queries` | 1.6441 µs – 1.6714 µs |

*Recorded: 2026-05-06, cargo bench --save-baseline v47.0.0*
