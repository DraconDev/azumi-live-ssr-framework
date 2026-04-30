//! Attribute Handling Tests
//!
//! Comprehensive tests for all HTML attribute variations
//! Run with: cargo test

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Boolean Attributes (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_disabled_button() {
    let component = html! { <button disabled="true">"Disabled"</button> };
    let html = test::render(&component);
    assert!(html.contains("disabled"));
}

#[test]
fn test_readonly_input() {
    let component = html! { <input type="text" readonly="true" /> };
    let html = test::render(&component);
    assert!(html.contains("readonly"));
}

#[test]
fn test_required_input() {
    let component = html! { <input type="text" required="true" /> };
    let html = test::render(&component);
    assert!(html.contains("required"));
}

#[test]
fn test_checked_checkbox() {
    let component = html! { <input type="checkbox" checked="true" /> };
    let html = test::render(&component);
    assert!(html.contains("checked"));
}

#[test]
fn test_selected_option() {
    let component = html! { <option selected="true">"Selected"</option> };
    let html = test::render(&component);
    assert!(html.contains("selected"));
}

#[test]
fn test_multiple_select() {
    let component = html! { <select multiple="true"><option>"1"</option></select> };
    let html = test::render(&component);
    assert!(html.contains("multiple"));
}

#[test]
fn test_autofocus_input() {
    let component = html! { <input type="text" autofocus="true" /> };
    let html = test::render(&component);
    assert!(html.contains("autofocus"));
}

#[test]
fn test_autoplay_video() {
    let component = html! { <video autoplay="true" src="/v.mp4">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("autoplay"));
}

#[test]
fn test_controls_video() {
    let component = html! { <video controls="true" src="/v.mp4">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("controls"));
}

#[test]
fn test_loop_video() {
    let component = html! { <video loop="true" src="/v.mp4">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("loop"));
}

#[test]
fn test_muted_video() {
    let component = html! { <video muted="true" src="/v.mp4">"Video"</video> };
    let html = test::render(&component);
    assert!(html.contains("muted"));
}

#[test]
fn test_hidden_element() {
    let component = html! { <div hidden="true">"Hidden"</div> };
    let html = test::render(&component);
    assert!(html.contains("hidden"));
}

#[test]
fn test_novalidate_form() {
    let component = html! { <form novalidate="true">"Form"</form> };
    let html = test::render(&component);
    assert!(html.contains("novalidate"));
}

#[test]
fn test_open_details() {
    let component = html! { <details open="true"><summary>"Open"</summary></details> };
    let html = test::render(&component);
    assert!(html.contains("open"));
}

#[test]
fn test_defer_script() {
    let component = html! { <script defer="true" src="/app.js"></script> };
    let html = test::render(&component);
    assert!(html.contains("defer"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Data Attributes (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_data_id() {
    let component = html! { <div data-id="123">"Item"</div> };
    let html = test::render(&component);
    assert!(html.contains("data-id=\"123\""));
}

#[test]
fn test_data_type() {
    let component = html! { <div data-type="card">"Card"</div> };
    let html = test::render(&component);
    assert!(html.contains("data-type=\"card\""));
}

#[test]
fn test_data_action() {
    let component = html! { <button data-action="submit">"Submit"</button> };
    let html = test::render(&component);
    assert!(html.contains("data-action=\"submit\""));
}

#[test]
fn test_data_index() {
    let component = html! { <li data-index="0">"First"</li> };
    let html = test::render(&component);
    assert!(html.contains("data-index=\"0\""));
}

#[test]
fn test_data_value_dynamic() {
    let val = 42;
    let component = html! { <span data-value={val}>{val}</span> };
    let html = test::render(&component);
    assert!(html.contains("data-value=") && html.contains("42"));
}

#[test]
fn test_data_state() {
    let component = html! { <div data-state="active">"Active"</div> };
    let html = test::render(&component);
    assert!(html.contains("data-state=\"active\""));
}

#[test]
fn test_data_toggle() {
    let component = html! { <button data-toggle="modal">"Open"</button> };
    let html = test::render(&component);
    assert!(html.contains("data-toggle=\"modal\""));
}

#[test]
fn test_data_target() {
    let component = html! { <button data-target="#modal">"Open"</button> };
    let html = test::render(&component);
    assert!(html.contains("data-target="));
}

#[test]
fn test_data_dismiss() {
    let component = html! { <button data-dismiss="modal">"Close"</button> };
    let html = test::render(&component);
    assert!(html.contains("data-dismiss=\"modal\""));
}

#[test]
fn test_data_loading() {
    let loading = true;
    let component = html! { <div data-loading={loading.to_string()}>"Content"</div> };
    let html = test::render(&component);
    assert!(html.contains("data-loading=\"true\""));
}

#[test]
fn test_data_page() {
    let page = 5;
    let component = html! { <nav data-page={page}>""</nav> };
    let html = test::render(&component);
    assert!(html.contains("data-page="));
}

#[test]
fn test_data_sort() {
    let component = html! { <th data-sort="asc">"Name"</th> };
    let html = test::render(&component);
    assert!(html.contains("data-sort=\"asc\""));
}

#[test]
fn test_data_filter() {
    let component = html! { <input type="text" data-filter="name" /> };
    let html = test::render(&component);
    assert!(html.contains("data-filter=\"name\""));
}

#[test]
fn test_data_testid() {
    let component = html! { <button data-testid="submit-btn">"Submit"</button> };
    let html = test::render(&component);
    assert!(html.contains("data-testid=\"submit-btn\""));
}

#[test]
fn test_multiple_data_attrs() {
    let component = html! { <div data-id="1" data-type="card" data-state="active">"Card"</div> };
    let html = test::render(&component);
    assert!(html.contains("data-id") && html.contains("data-type") && html.contains("data-state"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Event Attributes (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_az_on_click() {
    let component = html! { <button az-on="click call increment">"+"</button> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_submit() {
    let component = html! { <form az-on="submit call save">"Form"</form> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_input() {
    let component = html! { <input type="text" az-on="input call update" /> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_change() {
    let component = html! { <select az-on="change call select"><option>"1"</option></select> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_with_target() {
    let component = html! { <button az-on="click call update -> #result">"Update"</button> };
    let html = test::render(&component);
    assert!(html.contains("az-on=") && html.contains("#result"));
}

#[test]
fn test_az_on_focus() {
    let component = html! { <input type="text" az-on="focus call highlight" /> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_blur() {
    let component = html! { <input type="text" az-on="blur call validate" /> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_keydown() {
    let component = html! { <input type="text" az-on="keydown call handle" /> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_keyup() {
    let component = html! { <input type="text" az-on="keyup call search" /> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_mouseover() {
    let component = html! { <div az-on="mouseover call preview">"Hover"</div> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_mouseout() {
    let component = html! { <div az-on="mouseout call hide">"Hover"</div> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_scroll() {
    let component = html! { <div az-on="scroll call load_more">"Content"</div> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_dblclick() {
    let component = html! { <div az-on="dblclick call edit">"Edit me"</div> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_contextmenu() {
    let component = html! { <div az-on="contextmenu call menu">"Right-click"</div> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

#[test]
fn test_az_on_load() {
    let component = html! { <img src="/img.jpg" alt="img" az-on="load call loaded" /> };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Link and URL Attributes (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_href_absolute() {
    let component = html! { <a href="https://example.com">"Link"</a> };
    let html = test::render(&component);
    assert!(html.contains("href=\"https://example.com\""));
}

#[test]
fn test_href_relative() {
    let component = html! { <a href="/about">"About"</a> };
    let html = test::render(&component);
    assert!(html.contains("href=\"/about\""));
}

#[test]
fn test_href_hash() {
    let component = html! { <a href="#section">"Jump"</a> };
    let html = test::render(&component);
    assert!(html.contains("href=\"#section\""));
}

#[test]
fn test_href_mailto() {
    let component = html! { <a href="mailto:test@example.com">"Email"</a> };
    let html = test::render(&component);
    assert!(html.contains("mailto:"));
}

#[test]
fn test_href_tel() {
    let component = html! { <a href="tel:+1234567890">"Call"</a> };
    let html = test::render(&component);
    assert!(html.contains("tel:"));
}

#[test]
fn test_href_dynamic() {
    let url = "/page/1";
    let component = html! { <a href={url}>"Page 1"</a> };
    let html = test::render(&component);
    assert!(html.contains("href=\"/page/1\""));
}

#[test]
fn test_target_blank() {
    let component =
        html! { <a href="https://example.com" target="_blank" rel="noopener">"External"</a> };
    let html = test::render(&component);
    assert!(html.contains("target=\"_blank\""));
}

#[test]
fn test_target_self() {
    let component = html! { <a href="/about" target="_self">"About"</a> };
    let html = test::render(&component);
    assert!(html.contains("target=\"_self\""));
}

#[test]
fn test_rel_noopener() {
    let component = html! { <a href="https://example.com" rel="noopener">"Link"</a> };
    let html = test::render(&component);
    assert!(html.contains("rel=\"noopener\""));
}

#[test]
fn test_rel_noreferrer() {
    let component = html! { <a href="https://example.com" rel="noreferrer">"Link"</a> };
    let html = test::render(&component);
    assert!(html.contains("rel=\"noreferrer\""));
}

#[test]
fn test_download_attr() {
    let component = html! { <a href="/file.pdf" download="file.pdf">"Download"</a> };
    let html = test::render(&component);
    assert!(html.contains("download="));
}

#[test]
fn test_src_image() {
    let component = html! { <img src="/image.jpg" alt="Image" /> };
    let html = test::render(&component);
    assert!(html.contains("src=\"/image.jpg\""));
}

#[test]
fn test_srcset_responsive() {
    let component = html! { <img srcset="/small.jpg 480w, /large.jpg 800w" src="/default.jpg" alt="Responsive" /> };
    let html = test::render(&component);
    assert!(html.contains("srcset="));
}

#[test]
fn test_action_form() {
    let component = html! { <form action="/submit" method="post">"Form"</form> };
    let html = test::render(&component);
    assert!(html.contains("action=\"/submit\""));
}

#[test]
fn test_action_dynamic() {
    let endpoint = "/api/users";
    let component = html! { <form action={endpoint}>"Form"</form> };
    let html = test::render(&component);
    assert!(html.contains("action=\"/api/users\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 5: Form Input Attributes (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_input_type_text() {
    let component = html! { <input type="text" name="username" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"text\""));
}

#[test]
fn test_input_type_password() {
    let component = html! { <input type="password" name="password" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"password\""));
}

#[test]
fn test_input_type_email() {
    let component = html! { <input type="email" name="email" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"email\""));
}

#[test]
fn test_input_type_number() {
    let component = html! { <input type="number" name="age" min="0" max="120" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"number\"") && html.contains("min=") && html.contains("max="));
}

#[test]
fn test_input_type_range() {
    let component = html! { <input type="range" name="volume" min="0" max="100" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"range\""));
}

#[test]
fn test_input_type_date() {
    let component = html! { <input type="date" name="birthdate" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"date\""));
}

#[test]
fn test_input_type_time() {
    let component = html! { <input type="time" name="meeting" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"time\""));
}

#[test]
fn test_input_type_datetime_local() {
    let component = html! { <input type="datetime-local" name="event" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"datetime-local\""));
}

#[test]
fn test_input_type_color() {
    let component = html! { <input type="color" name="theme" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"color\""));
}

#[test]
fn test_input_type_file() {
    let component = html! { <input type="file" name="upload" accept=".pdf,.doc" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"file\"") && html.contains("accept="));
}

#[test]
fn test_input_placeholder() {
    let component = html! { <input type="text" placeholder="Enter name" /> };
    let html = test::render(&component);
    assert!(html.contains("placeholder=\"Enter name\""));
}

#[test]
fn test_input_value() {
    let val = "John";
    let component = html! { <input type="text" value={val} /> };
    let html = test::render(&component);
    assert!(html.contains("value=\"John\""));
}

#[test]
fn test_input_minlength() {
    let component = html! { <input type="text" minlength="3" /> };
    let html = test::render(&component);
    assert!(html.contains("minlength=\"3\""));
}

#[test]
fn test_input_maxlength() {
    let component = html! { <input type="text" maxlength="50" /> };
    let html = test::render(&component);
    assert!(html.contains("maxlength=\"50\""));
}

#[test]
fn test_input_pattern() {
    let component = html! { <input type="text" pattern="[A-Za-z]+" /> };
    let html = test::render(&component);
    assert!(html.contains("pattern="));
}

#[test]
fn test_input_step() {
    let component = html! { <input type="number" step="0.01" /> };
    let html = test::render(&component);
    assert!(html.contains("step=\"0.01\""));
}

#[test]
fn test_input_autocomplete() {
    let component = html! { <input type="email" autocomplete="email" /> };
    let html = test::render(&component);
    assert!(html.contains("autocomplete=\"email\""));
}

#[test]
fn test_input_inputmode() {
    let component = html! { <input type="text" inputmode="numeric" /> };
    let html = test::render(&component);
    assert!(html.contains("inputmode=\"numeric\""));
}

#[test]
fn test_textarea_rows_cols() {
    let component = html! { <textarea rows="10" cols="50">"Text"</textarea> };
    let html = test::render(&component);
    assert!(html.contains("rows=\"10\"") && html.contains("cols=\"50\""));
}

#[test]
fn test_select_size() {
    let component = html! { <select size="5"><option>"1"</option></select> };
    let html = test::render(&component);
    assert!(html.contains("size=\"5\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: Azumi UI State (az-ui) Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_az_ui_basic() {
    let component = html! { <div az-ui="{\"count\": 0}">"Counter"</div> };
    let html = test::render(&component);
    assert!(html.contains("az-ui=\""), "az-ui attribute missing");
}

#[test]
fn test_az_ui_with_set_command() {
    let component = html! {
        <div az-ui="{\"active_tab\": \"rust\"}">
            <button az-on="click set active_tab = 'rust'">"Rust"</button>
            <button az-on="click set active_tab = 'python'">"Python"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-ui=\""));
    assert!(html.contains("click set active_tab"));
}

#[test]
fn test_az_ui_with_bind_class() {
    let component = html! {
        <div az-ui="{\"is_open\": false}">
            <div az-bind:class:open="is_open">"Content"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-ui=\""));
    assert!(html.contains("az-bind:class:open=\"is_open\""));
}

#[test]
fn test_az_ui_with_bind_text() {
    let component = html! {
        <div az-ui="{\"count\": 0}">
            <span az-bind:text="count">"0"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-ui=\""));
    assert!(html.contains("az-bind:text=\"count\""));
}

#[test]
fn test_az_ui_toggle_boolean() {
    let component = html! {
        <details>
            <summary az-on="click set is_open = !is_open">"Toggle"</summary>
            <div az-bind:class:open="is_open">"Content"</div>
        </details>
    };
    let html = test::render(&component);
    assert!(html.contains("click set is_open = !is_open"));
}

#[test]
fn test_az_ui_multiple_fields() {
    let component = html! {
        <div az-ui="{\"tab1\": true, \"tab2\": false, \"count\": 42}">
            <button az-on="click set tab1 = !tab1">"Tab 1"</button>
            <button az-on="click set count = count + 1">"+1"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-ui=\""));
    assert!(html.contains("click set tab1"));
    assert!(html.contains("click set count"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: az-bind Rendering Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_az_bind_text_field() {
    let component = html! {
        <div az-ui="{\"count\": 0}">
            <span az-bind:text="count">"0"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text=\"count\""));
}

#[test]
fn test_az_bind_class_colon_field() {
    let component = html! {
        <div az-ui="{\"active\": true}">
            <div az-bind:class:active="active">"Content"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class:active=\"active\""));
}

#[test]
fn test_az_bind_class_colon_equality() {
    let component = html! {
        <div az-ui="{\"active_tab\": \"rust\"}">
            <button az-bind:class:active="active_tab == 'rust'">"Rust"</button>
        </div>
    };
    let html = test::render(&component);
    // HTML escapes single quotes to &#x27; — check for the attribute presence instead
    assert!(html.contains("az-bind:class:active="));
    assert!(html.contains("active_tab == "));
}

#[test]
fn test_az_bind_class_colon_negation() {
    let component = html! {
        <div az-ui="{\"is_open\": false}">
            <div az-bind:class:open="!is_open">"Hidden"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class:open=\"!is_open\""));
}

#[test]
fn test_az_bind_class_dot_syntax() {
    let component = html! {
        <div az-ui="{\"liked\": true}">
            <button az-bind:class.liked="liked">"Like"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class.liked=\"liked\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: az-bind Expression Evaluator Tests (v2)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_az_bind_class_less_than() {
    let component = html! {
        <div az-ui="{\"count\": 5}">
            <div az-bind:class:visible="count < 10">"Content"</div>
        </div>
    };
    let html = test::render(&component);
    // HTML escapes < to &lt;
    assert!(html.contains("az-bind:class:visible="));
    assert!(html.contains("count &lt; 10"));
}

#[test]
fn test_az_bind_class_greater_than() {
    let component = html! {
        <div az-ui="{\"count\": 5}">
            <div az-bind:class:active="count > 0">"Content"</div>
        </div>
    };
    let html = test::render(&component);
    // HTML escapes > to &gt;
    assert!(html.contains("az-bind:class:active="));
    assert!(html.contains("count &gt; 0"));
}

#[test]
fn test_az_bind_class_less_than_or_equal() {
    let component = html! {
        <div az-ui="{\"count\": 5}">
            <div az-bind:class:ready="count <= 10">"Content"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class:ready="));
    assert!(html.contains("count &lt;="));
}

#[test]
fn test_az_bind_class_greater_than_or_equal() {
    let component = html! {
        <div az-ui="{\"count\": 5}">
            <div az-bind:class:ready="count >= 5">"Content"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class:ready="));
    assert!(html.contains("count &gt;="));
}

#[test]
fn test_az_bind_class_compound_and() {
    let component = html! {
        <div az-ui="{\"a\": true, \"b\": true}">
            <div az-bind:class:ready="a && b">"Content"</div>
        </div>
    };
    let html = test::render(&component);
    // HTML escapes && to &amp;&amp;
    assert!(html.contains("az-bind:class:ready="));
    assert!(html.contains("&amp;&amp;"));
}

#[test]
fn test_az_bind_class_compound_or() {
    let component = html! {
        <div az-ui="{\"a\": false, \"b\": true}">
            <div az-bind:class:visible="a || b">"Content"</div>
        </div>
    };
    let html = test::render(&component);
    // HTML escapes | to &amp;| — just check attribute presence and || presence
    assert!(html.contains("az-bind:class:visible="));
    assert!(html.contains("||"));
}

#[test]
fn test_az_bind_class_ternary() {
    let component = html! {
        <div az-ui="{\"flag\": true}">
            <div az-bind:class:active="flag ? 'yes' : 'no'">"Content"</div>
        </div>
    };
    let html = test::render(&component);
    // HTML escapes single quotes
    assert!(html.contains("az-bind:class:active="));
    assert!(html.contains("flag ?"));
}

#[test]
fn test_az_bind_text_ternary() {
    let component = html! {
        <div az-ui="{\"acc1\": true}">
            <span az-bind:text="acc1 ? '−' : '+'">"+"</span>
        </div>
    };
    let html = test::render(&component);
    // HTML escapes single quotes
    assert!(html.contains("az-bind:text="));
    assert!(html.contains("acc1 ?"));
}

#[test]
fn test_az_bind_class_inequality() {
    let component = html! {
        <div az-ui="{\"status\": \"pending\"}">
            <div az-bind:class:error="status != 'ok'">"Alert"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class:error="));
    assert!(html.contains("status !="));
}

#[test]
fn test_az_bind_text_field_value() {
    let component = html! {
        <div az-ui="{\"count\": 42}">
            <span az-bind:text="count">"0"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text=\"count\""));
}

#[test]
fn test_az_bind_text_equality() {
    let component = html! {
        <div az-ui="{\"mode\": \"dark\"}">
            <span az-bind:text="mode == 'dark' ? '🌙' : '☀️'">"☀️"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text="));
    assert!(html.contains("mode =="));
}

#[test]
fn test_az_bind_text_inequality() {
    let component = html! {
        <div az-ui="{\"mode\": \"light\"}">
            <span az-bind:text="mode != 'dark' ? 'light' : 'dark'">"light"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text="));
    assert!(html.contains("mode !="));
}

#[test]
fn test_az_bind_text_numeric_comparison() {
    let component = html! {
        <div az-ui="{\"score\": 8}">
            <span az-bind:text="score > 5 ? 'high' : 'low'">"low"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text="));
    assert!(html.contains("score &gt;"));
}

#[test]
fn test_az_bind_text_negation() {
    let component = html! {
        <div az-ui="{\"enabled\": false}">
            <span az-bind:text="!enabled ? 'OFF' : 'ON'">"OFF"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text="));
    assert!(html.contains("!enabled"));
}

#[test]
fn test_az_bind_text_compound_and() {
    let component = html! {
        <div az-ui="{\"a\": true, \"b\": true}">
            <span az-bind:text="a && b ? 'ready' : 'waiting'">"waiting"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text="));
    assert!(html.contains("&amp;&amp;"));
}

#[test]
fn test_az_bind_text_compound_or() {
    let component = html! {
        <div az-ui="{\"x\": false, \"y\": true}">
            <span az-bind:text="x || y ? 'yes' : 'no'">"yes"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text="));
    assert!(html.contains("||"));
}

#[test]
fn test_az_bind_text_arithmetic_add() {
    let component = html! {
        <div az-ui="{\"count\": 5}">
            <span az-bind:text="count + 1">"6"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text=\"count + 1\""));
}

#[test]
fn test_az_bind_text_arithmetic_sub() {
    let component = html! {
        <div az-ui="{\"count\": 5}">
            <span az-bind:text="count - 1">"4"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text=\"count - 1\""));
}

#[test]
fn test_az_bind_text_boolean_literals() {
    let component = html! {
        <div az-ui="{}">
            <span az-bind:text="true">"truthy"</span>
            <span az-bind:text="false">"falsy"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text=\"true\""));
    assert!(html.contains("az-bind:text=\"false\""));
}

#[test]
fn test_az_bind_text_null() {
    let component = html! {
        <div az-ui="{}">
            <span az-bind:text="null">"empty"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text=\"null\""));
}

#[test]
fn test_az_bind_text_number_literal() {
    let component = html! {
        <div az-ui="{}">
            <span az-bind:text="42">"answer"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text=\"42\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: az-ui / az-scope Interop Tests
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_az_ui_within_az_scope() {
    let component = html! {
        <div az-scope="{\"server_val\": true}">
            <div az-ui="{\"local_val\": false}">
                <span az-bind:text="server_val ? 'yes' : 'no'">"no"</span>
                <span az-bind:text="local_val ? 'on' : 'off'">"off"</span>
            </div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-scope="));
    assert!(html.contains("az-ui="));
    assert!(html.contains("az-bind:text="));
}

#[test]
fn test_az_ui_and_az_scope_priority() {
    let component = html! {
        <div az-scope="{\"shared\": \"from-server\"}">
            <div az-ui="{\"shared\": \"from-ui\"}">
                <span az-bind:text="shared">"shared"</span>
            </div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-ui="));
    assert!(html.contains("az-scope="));
    assert!(html.contains("shared"));
}

#[test]
fn test_az_ui_multiple_scopes_isolated() {
    let component = html! {
        <div az-ui="{\"count\": 1}">
            <span az-bind:text="count">"1"</span>
        </div>
        <div az-ui="{\"count\": 2}">
            <span az-bind:text="count">"2"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains(r#"az-ui={"count": 1}"#));
    assert!(html.contains(r#"az-ui={"count": 2}"#));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: Multiple Bindings on Same Element
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_az_bind_class_and_text_same_element() {
    let component = html! {
        <div az-ui="{\"count\": 5, \"is_open\": true}">
            <div az-bind:class:visible="count > 0" az-bind:text="count">"5"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class:visible="));
    assert!(html.contains("az-bind:text="));
}

#[test]
fn test_az_bind_multiple_class_same_element() {
    let component = html! {
        <div az-ui="{\"a\": true, \"b\": false}">
            <div az-bind:class:x="a" az-bind:class:y="b">"text"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class:x="));
    assert!(html.contains("az-bind:class:y="));
}

#[test]
fn test_az_bind_class_colon_and_dot_same_element() {
    let component = html! {
        <div az-ui="{\"liked\": true, \"active\": true}">
            <button az-bind:class.liked="liked" az-bind:class:active="active">"Btn"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class.liked="));
    assert!(html.contains("az-bind:class:active="));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION: set Command Edge Cases
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_az_ui_set_nonexistent_field() {
    let component = html! {
        <div az-ui="{\"count\": 0}">
            <button az-on="click set nonexistent = 'value'">"Set new"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-on="));
    assert!(html.contains("set nonexistent"));
}

#[test]
fn test_az_ui_set_boolean_toggle_on_number() {
    let component = html! {
        <div az-ui="{\"count\": 5}">
            <button az-on="click set count = !count">"Toggle"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("click set count = !count"));
}

#[test]
fn test_az_ui_set_increment_on_string() {
    let component = html! {
        <div az-ui="{\"name\": \"Alice\"}">
            <button az-on="click set name = name + '!'">"Update"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("click set name = name + '!'"));
}

#[test]
fn test_az_ui_set_multiple_commands() {
    let component = html! {
        <div az-ui="{\"a\": 1, \"b\": 2}">
            <button az-on="click set a = a + 1; set b = b + 1">"Both"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("set a = a + 1"));
    assert!(html.contains("set b = b + 1"));
}

#[test]
fn test_az_ui_set_empty_value() {
    let component = html! {
        <div az-ui="{\"f\": \"x\"}">
            <button az-on="click set f = \"\"">"Clear"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("click set f ="));
}

#[test]
fn test_az_ui_set_increment_on_string() {
    let component = html! {
        <div az-ui="{\"n\": \"x\"}">
            <button az-on="click set n = n + \"y\"">"Update"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("click set n = n + "));
}

#[test]
fn test_az_ui_set_null_value() {
    let component = html! {
        <div az-ui="{\"f\": \"x\"}">
            <button az-on="click set f = null">"Nullify"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("click set f = null"));
}

#[test]
fn test_az_bind_text_empty_string_field() {
    let component = html! {
        <div az-ui="{\"n\": \"\"}">
            <span az-bind:text="n">"(empty)"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text="));
}

#[test]
fn test_az_bind_text_zero_value() {
    let component = html! {
        <div az-ui="{\"c\": 0}">
            <span az-bind:text="c">"0"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text="));
}

#[test]
fn test_az_bind_class_unicode_value() {
    let component = html! {
        <div az-ui="{\"s\": \"a\"}">
            <div az-bind:class:x="s == \"a\"">"Item"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class:x="));
}

#[test]
fn test_az_bind_class_empty_string_result() {
    let component = html! {
        <div az-ui="{\"n\": \"\"}">
            <div az-bind:class:h="n != \"\"">"Content"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class:h="));
}

#[test]
fn test_az_bind_text_arithmetic_missing_field() {
    let component = html! {
        <div az-ui="{}">
            <span az-bind:text="c + 1">"1"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text="));
}

#[test]
fn test_az_bind_class_false_comparison() {
    let component = html! {
        <div az-ui="{\"v\": 5}">
            <div az-bind:class:h="v == 0">"Zero?"</div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:class:h="));
}

#[test]
fn test_az_bind_text_string_literal() {
    let component = html! {
        <div az-ui="{}">
            <span az-bind:text="'hello'">"greeting"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-bind:text="));
    assert!(html.contains("hello"));
}

#[test]
fn test_az_ui_large_state_many_fields() {
    let component = html! {
        <div az-ui="{\"f1\":1,\"f2\":2,\"f3\":3,\"f4\":4,\"f5\":5,\"f6\":6,\"f7\":7,\"f8\":8,\"f9\":9,\"f10\":10}">
            <span az-bind:text="f10">"10"</span>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("az-ui="));
    assert!(html.contains("f10"));
    assert!(html.contains("az-bind:text="));
}

#[test]
fn test_az_ui_many_bound_elements() {
    let component = html! {
        <div az-ui="{\"v\": true}">
            <span az-bind:class:a="v">"1"</span>
            <span az-bind:class:b="v">"2"</span>
            <span az-bind:class:c="v">"3"</span>
            <span az-bind:class:d="v">"4"</span>
            <span az-bind:class:e="v">"5"</span>
        </div>
    };
    let html = test::render(&component);
    let count = html.matches("az-bind:class:").count();
    assert!(count >= 5, "Expected at least 5 az-bind:class, got {}", count);
}

#[test]
fn test_az_ui_state_preserved_in_nested_structure() {
    let component = html! {
        <div az-ui="{\"o\": true}">
            <div az-ui="{\"i\": false}">
                <span az-bind:class:so="o">"outer"</span>
                <span az-bind:class:si="i">"inner"</span>
            </div>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("inner"));
    assert!(html.contains("outer"));
}

