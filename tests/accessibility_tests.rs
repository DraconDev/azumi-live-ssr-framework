//! Accessibility Tests
//!
//! Tests for ARIA attributes and accessibility features
//! Run with: cargo test --features test-utils

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: ARIA Attributes (25 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_aria_label() {
    let component = html! { <button aria-label="Close dialog">"X"</button> };
    let html = test::render(&component);
    assert!(html.contains("aria-label="));
}

#[test]
fn test_aria_labelledby() {
    let component = html! { <div aria-labelledby="title">"Content"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-labelledby="));
}

#[test]
fn test_aria_describedby() {
    let component = html! { <input type="text" aria-describedby="hint" /> };
    let html = test::render(&component);
    assert!(html.contains("aria-describedby="));
}

#[test]
fn test_aria_hidden() {
    let component = html! { <span aria-hidden="true">"Decorative"</span> };
    let html = test::render(&component);
    assert!(html.contains("aria-hidden="));
}

#[test]
fn test_aria_expanded() {
    let component = html! { <button aria-expanded="false">"Toggle"</button> };
    let html = test::render(&component);
    assert!(html.contains("aria-expanded="));
}

#[test]
fn test_aria_pressed() {
    let component = html! { <button aria-pressed="true">"Active"</button> };
    let html = test::render(&component);
    assert!(html.contains("aria-pressed="));
}

#[test]
fn test_aria_checked() {
    let component = html! { <div role="checkbox" aria-checked="true">"✓"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-checked="));
}

#[test]
fn test_aria_selected() {
    let component = html! { <div role="option" aria-selected="true">"Option"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-selected="));
}

#[test]
fn test_aria_disabled() {
    let component = html! { <button aria-disabled="true">"Disabled"</button> };
    let html = test::render(&component);
    assert!(html.contains("aria-disabled="));
}

#[test]
fn test_aria_readonly() {
    let component = html! { <input type="text" aria-readonly="true" /> };
    let html = test::render(&component);
    assert!(html.contains("aria-readonly="));
}

#[test]
fn test_aria_required() {
    let component = html! { <input type="text" aria-required="true" /> };
    let html = test::render(&component);
    assert!(html.contains("aria-required="));
}

#[test]
fn test_aria_invalid() {
    let component = html! { <input type="email" aria-invalid="true" /> };
    let html = test::render(&component);
    assert!(html.contains("aria-invalid="));
}

#[test]
fn test_aria_live() {
    let component = html! { <div aria-live="polite">"Status"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-live="));
}

#[test]
fn test_aria_atomic() {
    let component = html! { <div aria-atomic="true">"Atomic"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-atomic="));
}

#[test]
fn test_aria_busy() {
    let component = html! { <div aria-busy="true">"Loading..."</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-busy="));
}

#[test]
fn test_aria_controls() {
    let component = html! { <button aria-controls="menu">"Toggle Menu"</button> };
    let html = test::render(&component);
    assert!(html.contains("aria-controls="));
}

#[test]
fn test_aria_owns() {
    let component = html! { <div aria-owns="child">"Parent"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-owns="));
}

#[test]
fn test_aria_haspopup() {
    let component = html! { <button aria-haspopup="menu">"Menu"</button> };
    let html = test::render(&component);
    assert!(html.contains("aria-haspopup="));
}

#[test]
fn test_aria_level() {
    let component = html! { <div role="heading" aria-level="2">"Heading"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-level="));
}

#[test]
fn test_aria_valuemin() {
    let component = html! { <div role="slider" aria-valuemin="0">"Slider"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-valuemin="));
}

#[test]
fn test_aria_valuemax() {
    let component = html! { <div role="slider" aria-valuemax="100">"Slider"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-valuemax="));
}

#[test]
fn test_aria_valuenow() {
    let component = html! { <div role="slider" aria-valuenow="50">"Slider"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-valuenow="));
}

#[test]
fn test_aria_valuetext() {
    let component = html! { <div role="slider" aria-valuetext="50%">"Slider"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-valuetext="));
}

#[test]
fn test_aria_current() {
    let component = html! { <a href="/" aria-current="page">"Home"</a> };
    let html = test::render(&component);
    assert!(html.contains("aria-current="));
}

#[test]
fn test_aria_modal() {
    let component = html! { <div role="dialog" aria-modal="true">"Modal"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-modal="));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Additional ARIA Roles (banner, main, search, dialog, tooltip)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_role_tooltip() {
    let component = html! { <div role="tooltip">"Hint text"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"tooltip\""));
}

#[test]
fn test_role_banner_is_document_region() {
    let component = html! { <header role="banner">"Site Header"</header> };
    let html = test::render(&component);
    assert!(html.contains("role=\"banner\""));
}

#[test]
fn test_role_main_is_unique_per_page() {
    let component = html! { <main role="main">"Main Content"</main> };
    let html = test::render(&component);
    assert!(html.contains("role=\"main\""));
}

#[test]
fn test_role_search_with_form() {
    let component = html! {
        <form role="search">
            <input type="search" aria-label="Search" />
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"search\""));
}

#[test]
fn test_role_dialog_with_aria_modal() {
    let title_id = "title";
    let component = html! {
        <div role="dialog" aria-modal="true" aria-labelledby={title_id}>
            <h2 id={title_id}>"Confirm"</h2>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("role=\"dialog\"") && html.contains("aria-modal="));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Remaining Input Types
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_input_type_password() {
    let component = html! { <input type="password" name="pwd" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"password\""));
}

#[test]
fn test_input_type_number_with_attributes() {
    let component = html! {
        <input type="number" name="qty" min="0" max="100" step="5" />
    };
    let html = test::render(&component);
    assert!(html.contains("type=\"number\""));
    assert!(html.contains("min="));
    assert!(html.contains("max="));
    assert!(html.contains("step="));
}

#[test]
fn test_input_type_color() {
    let component = html! { <input type="color" name="theme" value="#ff0000" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"color\""));
}

#[test]
fn test_input_type_file_accept() {
    let component = html! {
        <input type="file" name="avatar" accept="image/png, image/jpeg" />
    };
    let html = test::render(&component);
    assert!(html.contains("type=\"file\""));
    assert!(html.contains("accept="));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 5: Button Type Validation
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_button_type_submit() {
    let component = html! { <button type="submit">"Submit"</button> };
    let html = test::render(&component);
    assert!(html.contains("type=\"submit\""));
}

#[test]
fn test_button_type_reset() {
    let component = html! { <button type="reset">"Reset"</button> };
    let html = test::render(&component);
    assert!(html.contains("type=\"reset\""));
}

#[test]
fn test_button_type_button() {
    let component = html! { <button type="button">"Click me"</button> };
    let html = test::render(&component);
    assert!(html.contains("type=\"button\""));
}

#[test]
fn test_button_without_type_defaults() {
    let component = html! { <button>"Default"</button> };
    let html = test::render(&component);
    assert!(html.contains("<button>"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 6: Accessibility Suggestion (suggested_type_correction)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_input_type_tel_suggested() {
    let component = html! { <input type="tel" name="phone" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"tel\""));
}

#[test]
fn test_input_type_url_suggested() {
    let component = html! { <input type="url" name="website" placeholder="https://" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"url\""));
}

#[test]
fn test_role_link() {
    let component = html! { <span role="link">"Link"</span> };
    let html = test::render(&component);
    assert!(html.contains("role=\"link\""));
}

#[test]
fn test_role_navigation() {
    let component = html! { <div role="navigation">"Nav"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"navigation\""));
}

#[test]
fn test_role_main() {
    let component = html! { <div role="main">"Main"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"main\""));
}

#[test]
fn test_role_banner() {
    let component = html! { <div role="banner">"Header"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"banner\""));
}

#[test]
fn test_role_contentinfo() {
    let component = html! { <div role="contentinfo">"Footer"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"contentinfo\""));
}

#[test]
fn test_role_complementary() {
    let component = html! { <div role="complementary">"Sidebar"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"complementary\""));
}

#[test]
fn test_role_search() {
    let component = html! { <div role="search">"Search"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"search\""));
}

#[test]
fn test_role_alert() {
    let component = html! { <div role="alert">"Error!"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"alert\""));
}

#[test]
fn test_role_alertdialog() {
    let component = html! { <div role="alertdialog">"Confirm?"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"alertdialog\""));
}

#[test]
fn test_role_dialog() {
    let component = html! { <div role="dialog">"Modal"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"dialog\""));
}

#[test]
fn test_role_menu() {
    let component = html! { <div role="menu">"Menu"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"menu\""));
}

#[test]
fn test_role_menuitem() {
    let component = html! { <div role="menuitem">"Item"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"menuitem\""));
}

#[test]
fn test_role_listbox() {
    let component = html! { <div role="listbox">"List"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"listbox\""));
}

#[test]
fn test_role_option() {
    let component = html! { <div role="option">"Option"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"option\""));
}

#[test]
fn test_role_tablist() {
    let component = html! { <div role="tablist">"Tabs"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"tablist\""));
}

#[test]
fn test_role_tab() {
    let component = html! { <div role="tab">"Tab"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"tab\""));
}

#[test]
fn test_role_tabpanel() {
    let component = html! { <div role="tabpanel">"Panel"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"tabpanel\""));
}

#[test]
fn test_role_progressbar() {
    let component = html! { <div role="progressbar" aria-valuenow="50">"50%"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"progressbar\""));
}

#[test]
fn test_role_spinbutton() {
    let component = html! { <div role="spinbutton">"Spinner"</div> };
    let html = test::render(&component);
    assert!(html.contains("role=\"spinbutton\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Keyboard Accessibility (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_tabindex_zero() {
    let component = html! { <div tabindex="0">"Focusable"</div> };
    let html = test::render(&component);
    assert!(html.contains("tabindex=\"0\""));
}

#[test]
fn test_tabindex_negative() {
    let component = html! { <div tabindex="-1">"Programmatic focus"</div> };
    let html = test::render(&component);
    assert!(html.contains("tabindex=\"-1\""));
}

#[test]
fn test_tabindex_positive() {
    let component = html! { <div tabindex="1">"First"</div> };
    let html = test::render(&component);
    assert!(html.contains("tabindex=\"1\""));
}

#[test]
fn test_accesskey() {
    let component = html! { <button accesskey="s">"Save"</button> };
    let html = test::render(&component);
    assert!(html.contains("accesskey="));
}

#[test]
fn test_autofocus() {
    let component = html! { <input type="text" autofocus="true" /> };
    let html = test::render(&component);
    assert!(html.contains("autofocus"));
}

#[test]
fn test_enterkeyhint() {
    let component = html! { <input type="text" enterkeyhint="search" /> };
    let html = test::render(&component);
    assert!(html.contains("enterkeyhint="));
}

#[test]
fn test_inputmode() {
    let component = html! { <input type="text" inputmode="numeric" /> };
    let html = test::render(&component);
    assert!(html.contains("inputmode="));
}

#[test]
fn test_inert() {
    let component = html! { <div data-inert="true">"Inert content"</div> };
    let html = test::render(&component);
    assert!(html.contains("data-inert"));
}

#[test]
fn test_focusable_div_with_role() {
    let component = html! { <div role="button" tabindex="0">"Button"</div> };
    let html = test::render(&component);
    assert!(html.contains("tabindex=") && html.contains("role="));
}

#[test]
fn test_skip_link() {
    let component = html! { <a href="#main" tabindex="0">"Skip to main"</a> };
    let html = test::render(&component);
    assert!(html.contains("Skip to main") && html.contains("href="));
}

#[test]
fn test_focus_trap_pattern() {
    let component = html! {
        <div role="dialog" aria-modal="true">
            <button tabindex="0">"Close"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("aria-modal=") && html.contains("tabindex="));
}

#[test]
fn test_multiple_focusable() {
    let component = html! {
        <div>
            <button>"One"</button>
            <button>"Two"</button>
            <button>"Three"</button>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("One") && html.contains("Two") && html.contains("Three"));
}

#[test]
fn test_contenteditable() {
    let component = html! { <div contenteditable="true">"Edit me"</div> };
    let html = test::render(&component);
    assert!(html.contains("contenteditable="));
}

#[test]
fn test_draggable() {
    let component = html! { <div draggable="true">"Drag me"</div> };
    let html = test::render(&component);
    assert!(html.contains("draggable="));
}

#[test]
fn test_spellcheck() {
    let component = html! { <textarea spellcheck="true">"Text"</textarea> };
    let html = test::render(&component);
    assert!(html.contains("spellcheck="));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Screen Reader Accessibility (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_sr_only_class() {
    let component = html! { 
        <span class={sr_only}>"Screen reader only"</span>
        <style>
            .sr_only { position: "absolute"; width: "1px"; height: "1px"; overflow: "hidden"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("class=\"sr_only\""));
}

#[test]
fn test_visually_hidden() {
    let component = html! { 
        <span class={visually_hidden}>"Hidden text"</span>
        <style>
            .visually_hidden { position: "absolute"; left: "-9999px"; }
        </style>
    };
    let html = test::render(&component);
    assert!(html.contains("class=\"visually_hidden\""));
}

#[test]
fn test_alt_text_img() {
    let component = html! { <img src="/logo.png" alt="Company Logo" /> };
    let html = test::render(&component);
    assert!(html.contains("alt=\"Company Logo\""));
}

#[test]
fn test_empty_alt_decorative() {
    let component = html! { <img src="/decoration.png" alt="" /> };
    let html = test::render(&component);
    assert!(html.contains("alt=\"\""));
}

#[test]
fn test_figure_with_figcaption() {
    let component = html! {
        <figure>
            <img src="/chart.png" alt="Sales chart" />
            <figcaption>"Quarterly sales data"</figcaption>
        </figure>
    };
    let html = test::render(&component);
    assert!(html.contains("<figure>") && html.contains("<figcaption>"));
}

#[test]
fn test_abbr_with_title() {
    let component = html! { <abbr title="HyperText Markup Language">"HTML"</abbr> };
    let html = test::render(&component);
    assert!(html.contains("title=") && html.contains("HTML"));
}

#[test]
fn test_lang_attribute() {
    let component = html! { <span lang="es">"Hola"</span> };
    let html = test::render(&component);
    assert!(html.contains("lang=\"es\""));
}

#[test]
fn test_dir_attribute() {
    let component = html! { <span dir="rtl">"مرحبا"</span> };
    let html = test::render(&component);
    assert!(html.contains("dir=\"rtl\""));
}

#[test]
fn test_translate_attribute() {
    let component = html! { <span translate="no">"Brand Name"</span> };
    let html = test::render(&component);
    assert!(html.contains("translate="));
}

#[test]
fn test_live_region_assertive() {
    let component = html! { <div aria-live="assertive">"Important!"</div> };
    let html = test::render(&component);
    assert!(html.contains("aria-live=\"assertive\""));
}
