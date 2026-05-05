use azumi::html;

fn main() {
    // Verify safe macros still compile
    let data = serde_json::json!({"key": "value"});
    let _component = html! {
        <div>
            {azumi::json_data!("DATA" = &data)}
            {azumi::inline_css!(THEME_CSS)}
            {azumi::inline_script!(AI_HUB_COPY_JS)}
            <p>"Hello"</p>
        </div>
    };
}

// Dummy variables referenced by macros
static THEME_CSS: &str = ".card { color: red; }";
static AI_HUB_COPY_JS: &str = "console.log('hello');";
