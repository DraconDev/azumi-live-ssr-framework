use azumi::html;

fn main() {
    // Verify safe patterns still compile:
    // - json_data! macro for JSON injection
    // - <style>{var}</style> for CSS injection (auto-escaped)
    // - <script>{var}</script> for JS injection (auto-escaped)
    let data = serde_json::json!({"key": "value"});
    let _component = html! {
        <div>
            {azumi::json_data!("DATA" = &data)}
            <style>{THEME_CSS}</style>
            <script>{AI_HUB_COPY_JS}</script>
            <p>"Hello"</p>
        </div>
    };
}

// Dummy variables referenced by templates
static THEME_CSS: &str = ".card { color: red; }";
static AI_HUB_COPY_JS: &str = "console.log('hello');";
