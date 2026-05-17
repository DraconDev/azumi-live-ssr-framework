//! Form Pattern Tests
//!
//! Tests for form elements and validation patterns
//! Run with: cargo test

use azumi::{html, test};

// ════════════════════════════════════════════════════════════════════════════
// SECTION 1: Form Structure (20 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_basic_form() {
    let component = html! {
        <form action="/submit" method="post">
            <button type="submit">"Submit"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("<form") && html.contains("action="));
}

#[test]
fn test_form_get_method() {
    let component = html! {
        <form action="/search" method="get">"Search"</form>
    };
    let html = test::render(&component);
    assert!(html.contains("method=\"get\""));
}

#[test]
fn test_form_multipart() {
    let component = html! {
        <form enctype="multipart/form-data">"Upload"</form>
    };
    let html = test::render(&component);
    assert!(html.contains("multipart/form-data"));
}

#[test]
fn test_fieldset() {
    let component = html! {
        <fieldset>
            <legend>"Personal Info"</legend>
            <input type="text" name="name" />
        </fieldset>
    };
    let html = test::render(&component);
    assert!(html.contains("<fieldset>") && html.contains("<legend>"));
}

#[test]
fn test_label_for_input() {
    let component = html! {
        <div>
            <label for="email">"Email"</label>
            <input type="email" name="email" />
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("<label") && html.contains("for="));
}

#[test]
fn test_input_with_label() {
    let component = html! {
        <label>
            "Username"
            <input type="text" name="username" />
        </label>
    };
    let html = test::render(&component);
    assert!(html.contains("Username") && html.contains("input"));
}

#[test]
fn test_text_input() {
    let component = html! { <input type="text" name="fullname" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"text\""));
}

#[test]
fn test_email_input() {
    let component = html! { <input type="email" name="email" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"email\""));
}

#[test]
fn test_password_input() {
    let component = html! { <input type="password" name="pass" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"password\""));
}

#[test]
fn test_number_input() {
    let component = html! { <input type="number" name="qty" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"number\""));
}

#[test]
fn test_tel_input() {
    let component = html! { <input type="tel" name="phone" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"tel\""));
}

#[test]
fn test_url_input() {
    let component = html! { <input type="url" name="website" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"url\""));
}

#[test]
fn test_search_input() {
    let component = html! { <input type="search" name="q" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"search\""));
}

#[test]
fn test_checkbox_input() {
    let component = html! { <input type="checkbox" name="agree" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"checkbox\""));
}

#[test]
fn test_radio_input() {
    let component = html! { <input type="radio" name="choice" value="a" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"radio\""));
}

#[test]
fn test_hidden_input() {
    let component = html! { <input type="hidden" name="token" value="abc123" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"hidden\""));
}

#[test]
fn test_button_submit() {
    let component = html! { <button type="submit">"Submit"</button> };
    let html = test::render(&component);
    assert!(html.contains("type=\"submit\""));
}

#[test]
fn test_button_reset() {
    let component = html! { <button type="reset">"Reset"</button> };
    let html = test::render(&component);
    assert!(html.contains("type=\"reset\""));
}

#[test]
fn test_button_button() {
    let component = html! { <button type="button">"Click"</button> };
    let html = test::render(&component);
    assert!(html.contains("type=\"button\""));
}

#[test]
fn test_input_submit() {
    let component = html! { <input type="submit" value="Submit" /> };
    let html = test::render(&component);
    assert!(html.contains("type=\"submit\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 2: Select and Options (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_basic_select() {
    let component = html! {
        <select name="country">
            <option value="us">"USA"</option>
        </select>
    };
    let html = test::render(&component);
    assert!(html.contains("<select") && html.contains("<option"));
}

#[test]
fn test_select_multiple_options() {
    let options = vec![("us", "USA"), ("uk", "UK"), ("ca", "Canada")];
    let component = html! {
        <select name="country">
            @for (val, label) in &options {
                <option value={*val}>{label}</option>
            }
        </select>
    };
    let html = test::render(&component);
    assert!(html.contains("USA") && html.contains("UK") && html.contains("Canada"));
}

#[test]
fn test_select_optgroup() {
    let component = html! {
        <select>
            <optgroup label="Europe">
                <option>"UK"</option>
                <option>"France"</option>
            </optgroup>
        </select>
    };
    let html = test::render(&component);
    assert!(html.contains("<optgroup") && html.contains("Europe"));
}

#[test]
fn test_select_disabled_option() {
    let component = html! {
        <select>
            <option disabled="true">"Select..."</option>
            <option>"Option 1"</option>
        </select>
    };
    let html = test::render(&component);
    assert!(html.contains("disabled"));
}

#[test]
fn test_select_selected() {
    let component = html! {
        <select>
            <option>"One"</option>
            <option selected="true">"Two"</option>
        </select>
    };
    let html = test::render(&component);
    assert!(html.contains("selected"));
}

#[test]
fn test_datalist() {
    let component = html! {
        <div>
            <input list="browsers" />
            <datalist>
                <option value="Chrome" />
                <option value="Firefox" />
            </datalist>
        </div>
    };
    let html = test::render(&component);
    assert!(html.contains("<datalist") && html.contains("Chrome"));
}

#[test]
fn test_textarea() {
    let component = html! { <textarea name="message">"Default text"</textarea> };
    let html = test::render(&component);
    assert!(html.contains("<textarea") && html.contains("Default text"));
}

#[test]
fn test_textarea_rows() {
    let component = html! { <textarea rows="10" name="bio">"Bio"</textarea> };
    let html = test::render(&component);
    assert!(html.contains("rows=\"10\""));
}

#[test]
fn test_textarea_cols() {
    let component = html! { <textarea cols="50" name="msg">"Msg"</textarea> };
    let html = test::render(&component);
    assert!(html.contains("cols=\"50\""));
}

#[test]
fn test_textarea_placeholder() {
    let component = html! { <textarea placeholder="Enter message...">"text"</textarea> };
    let html = test::render(&component);
    assert!(html.contains("placeholder="));
}

#[test]
fn test_output_element() {
    let component = html! { <output name="result">"0"</output> };
    let html = test::render(&component);
    assert!(html.contains("<output"));
}

#[test]
fn test_meter_element() {
    let component = html! { <meter value="0.6" min="0" max="1">"60%"</meter> };
    let html = test::render(&component);
    assert!(html.contains("<meter") && html.contains("value="));
}

#[test]
fn test_progress_element() {
    let component = html! { <progress value="70" max="100">"70%"</progress> };
    let html = test::render(&component);
    assert!(html.contains("<progress") && html.contains("70"));
}

#[test]
fn test_keygen_deprecated() {
    // Just ensure form element exists
    let component = html! { <form>"Form"</form> };
    let html = test::render(&component);
    assert!(html.contains("<form"));
}

#[test]
fn test_select_size() {
    let component = html! {
        <select size="5">
            <option>"1"</option>
        </select>
    };
    let html = test::render(&component);
    assert!(html.contains("size=\"5\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 3: Validation Attributes (15 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_required_attribute() {
    let component = html! { <input type="text" required="true" /> };
    let html = test::render(&component);
    assert!(html.contains("required"));
}

#[test]
fn test_minlength_validation() {
    let component = html! { <input type="text" minlength="5" /> };
    let html = test::render(&component);
    assert!(html.contains("minlength=\"5\""));
}

#[test]
fn test_maxlength_validation() {
    let component = html! { <input type="text" maxlength="100" /> };
    let html = test::render(&component);
    assert!(html.contains("maxlength=\"100\""));
}

#[test]
fn test_min_validation() {
    let component = html! { <input type="number" min="0" /> };
    let html = test::render(&component);
    assert!(html.contains("min=\"0\""));
}

#[test]
fn test_max_validation() {
    let component = html! { <input type="number" max="100" /> };
    let html = test::render(&component);
    assert!(html.contains("max=\"100\""));
}

#[test]
fn test_step_validation() {
    let component = html! { <input type="number" step="5" /> };
    let html = test::render(&component);
    assert!(html.contains("step=\"5\""));
}

#[test]
fn test_pattern_validation() {
    let component = html! { <input type="text" pattern="[A-Z]{3}" /> };
    let html = test::render(&component);
    assert!(html.contains("pattern="));
}

#[test]
fn test_novalidate_form() {
    let component = html! { <form novalidate="true">"Form"</form> };
    let html = test::render(&component);
    assert!(html.contains("novalidate"));
}

#[test]
fn test_formnovalidate_button() {
    let component = html! { <button formnovalidate="true" type="submit">"Skip"</button> };
    let html = test::render(&component);
    assert!(html.contains("formnovalidate"));
}

#[test]
fn test_aria_invalid() {
    let component = html! { <input type="email" aria-invalid="true" /> };
    let html = test::render(&component);
    assert!(html.contains("aria-invalid"));
}

#[test]
fn test_aria_errormessage() {
    let component = html! { <input type="text" aria-errormessage="error-msg" /> };
    let html = test::render(&component);
    assert!(html.contains("aria-errormessage"));
}

#[test]
fn test_title_validation_message() {
    let component = html! { <input type="text" pattern="[0-9]+" title="Numbers only" /> };
    let html = test::render(&component);
    assert!(html.contains("title=\"Numbers only\""));
}

#[test]
fn test_autocomplete_off() {
    let component = html! { <input type="password" autocomplete="off" /> };
    let html = test::render(&component);
    assert!(html.contains("autocomplete=\"off\""));
}

#[test]
fn test_autocomplete_new_password() {
    let component = html! { <input type="password" autocomplete="new-password" /> };
    let html = test::render(&component);
    assert!(html.contains("new-password"));
}

#[test]
fn test_inputmode_numeric() {
    let component = html! { <input type="text" inputmode="numeric" /> };
    let html = test::render(&component);
    assert!(html.contains("inputmode=\"numeric\""));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 4: Form Examples (10 tests)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_login_form_structure() {
    let component = html! {
        <form action="/login" method="post">
            <label for="username">"Username"</label>
            <input type="text" name="username" required="true" />
            <label for="password">"Password"</label>
            <input type="password" name="password" required="true" />
            <button type="submit">"Login"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("Username") && html.contains("Password") && html.contains("Login"));
}

#[test]
fn test_registration_form() {
    let component = html! {
        <form action="/register" method="post">
            <input type="text" name="name" placeholder="Name" />
            <input type="email" name="email" placeholder="Email" />
            <input type="password" name="password" placeholder="Password" />
            <input type="password" name="confirm" placeholder="Confirm" />
            <button type="submit">"Register"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("register") && html.contains("Register"));
}

#[test]
fn test_contact_form_structure() {
    let component = html! {
        <form action="/contact" method="post">
            <input type="text" name="name" />
            <input type="email" name="email" />
            <input type="tel" name="phone" />
            <textarea name="message">"msg"</textarea>
            <button type="submit">"Send"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("contact") && html.contains("<textarea"));
}

#[test]
fn test_search_form_structure() {
    let component = html! {
        <form action="/search" method="get" role="search">
            <input type="search" name="q" placeholder="Search..." />
            <button type="submit">"Go"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("search") && html.contains("role=\"search\""));
}

#[test]
fn test_newsletter_form() {
    let component = html! {
        <form action="/subscribe" method="post">
            <label for="email">"Newsletter"</label>
            <input type="email" name="email" placeholder="your@email.com" />
            <button type="submit">"Subscribe"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("Newsletter") && html.contains("Subscribe"));
}

#[test]
fn test_checkout_form() {
    let component = html! {
        <form action="/checkout" method="post">
            <fieldset>
                <legend>"Payment"</legend>
                <input type="text" name="card" placeholder="Card Number" />
                <input type="text" name="expiry" placeholder="MM/YY" />
                <input type="text" name="cvv" placeholder="CVV" />
            </fieldset>
            <button type="submit">"Pay Now"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("Payment") && html.contains("Pay Now"));
}

#[test]
fn test_settings_form() {
    let component = html! {
        <form action="/settings" method="post">
            <label>"Notifications"</label>
            <input type="checkbox" name="notify_email" /> "Email"
            <input type="checkbox" name="notify_sms" /> "SMS"
            <button type="submit">"Save"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("Notifications") && html.contains("checkbox"));
}

#[test]
fn test_profile_form() {
    let component = html! {
        <form action="/profile" method="post" enctype="multipart/form-data">
            <input type="file" name="avatar" accept="image/*" />
            <input type="text" name="bio" />
            <button type="submit">"Update"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("multipart") && html.contains("file"));
}

#[test]
fn test_feedback_form() {
    let component = html! {
        <form action="/feedback" method="post">
            <label>"Rating"</label>
            <select name="rating">
                <option value="5">"Excellent"</option>
                <option value="4">"Good"</option>
                <option value="3">"Average"</option>
            </select>
            <textarea name="comments">"comments"</textarea>
            <button type="submit">"Submit"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("Rating") && html.contains("Excellent"));
}

#[test]
fn test_survey_form() {
    let questions = ["How did you hear about us?", "Would you recommend us?"];
    let component = html! {
        <form action="/survey" method="post">
            @for (i, q) in questions.iter().enumerate() {
                <div>
                    <p>{q}</p>
                    <input type="text" name={format!("q{}", i)} />
                </div>
            }
            <button type="submit">"Complete"</button>
        </form>
    };
    let html = test::render(&component);
    assert!(html.contains("How did you hear") && html.contains("recommend"));
}

// ════════════════════════════════════════════════════════════════════════════
// SECTION 5: ValidatedForm Render Output Tests
// ════════════════════════════════════════════════════════════════════════════

use azumi::form::{ValidatedForm, ValidationErrors};

#[test]
fn test_validated_input_renders_text_input() {
    let component = ValidatedForm::input("username", "alice", None);
    let html = test::render(&component);
    assert!(html.contains(r#"type="text""#), "Should render text input type");
    assert!(html.contains(r#"name="username""#), "Should render name attribute");
    assert!(html.contains(r#"value="alice""#), "Should render value attribute");
    assert!(html.contains("<div>"), "Should wrap in div");
    assert!(html.contains("</div>"), "Should close div");
}

#[test]
fn test_validated_input_renders_no_error_when_none() {
    let component = ValidatedForm::input("email", "test@test.com", None);
    let html = test::render(&component);
    assert!(!html.contains("aria-invalid"), "Should not have aria-invalid when no error");
    assert!(!html.contains("field_error"), "Should not have error div when no error");
}

#[test]
fn test_validated_input_renders_error_with_aria() {
    let component = ValidatedForm::input("email", "bad", Some("Invalid email"));
    let html = test::render(&component);
    assert!(html.contains(r#"aria-invalid="true""#), "Should set aria-invalid on error");
    assert!(html.contains(r#"aria-describedby="email_error""#), "Should set aria-describedby");
    assert!(html.contains(r#"id="email_error""#), "Should render error div with id");
    assert!(html.contains("Invalid email"), "Should render error message");
    assert!(html.contains(r#"role="alert""#), "Error should have role=alert");
}

#[test]
fn test_validated_email_renders_email_type() {
    let component = ValidatedForm::email("email", "user@test.com", None);
    let html = test::render(&component);
    assert!(html.contains(r#"type="email""#), "Should render email input type");
    assert!(html.contains(r#"name="email""#), "Should render name");
}

#[test]
fn test_validated_password_renders_password_type() {
    let component = ValidatedForm::password("pass", "secret", None);
    let html = test::render(&component);
    assert!(html.contains(r#"type="password""#), "Should render password input type");
    assert!(html.contains(r#"value="secret""#), "Should render value");
}

#[test]
fn test_validated_textarea_renders_correctly() {
    let component = ValidatedForm::textarea("bio", "Hello world", None);
    let html = test::render(&component);
    assert!(html.contains("<textarea"), "Should render textarea element");
    assert!(html.contains(r#"name="bio""#), "Should render name attribute");
    assert!(html.contains("Hello world"), "Should render content");
    assert!(html.contains("</textarea>"), "Should close textarea");
}

#[test]
fn test_validated_textarea_error_with_aria() {
    let component = ValidatedForm::textarea("bio", "", Some("Bio is required"));
    let html = test::render(&component);
    assert!(html.contains(r#"aria-invalid="true""#), "Should set aria-invalid on error");
    assert!(html.contains(r#"aria-describedby="bio_error""#), "Should set aria-describedby");
    assert!(html.contains("Bio is required"), "Should render error message");
}

#[test]
fn test_validated_select_renders_options() {
    let options = [("us", "USA"), ("uk", "UK")];
    let component = ValidatedForm::select("country", "uk", &options, None);
    let html = test::render(&component);
    assert!(html.contains("<select"), "Should render select element");
    assert!(html.contains(r#"name="country""#), "Should render name");
    assert!(html.contains(r#"value="us""#), "Should render option value");
    assert!(html.contains("USA"), "Should render option label");
    assert!(html.contains(" selected"), "Should mark uk as selected");
}

#[test]
fn test_validated_select_error_with_aria() {
    let options = [("a", "A")];
    let component = ValidatedForm::select("choice", "a", &options, Some("Required"));
    let html = test::render(&component);
    assert!(html.contains(r#"aria-invalid="true""#), "Should set aria-invalid on error");
    assert!(html.contains("Required"), "Should render error message");
}

#[test]
fn test_validated_error_summary_renders() {
    let mut errors = ValidationErrors::new();
    errors.add("email", "Invalid email");
    errors.add("name", "Name required");
    let component = ValidatedForm::error_summary(&errors);
    let html = test::render(&component);
    assert!(html.contains("form_errors"), "Should have form_errors class");
    assert!(html.contains(r#"role="alert""#), "Should have role=alert");
    assert!(html.contains("email_error"), "Should link to email error");
    assert!(html.contains("name_error"), "Should link to name error");
    assert!(html.contains("Invalid email"), "Should show error message");
}

#[test]
fn test_validated_error_summary_empty_renders_nothing() {
    let errors = ValidationErrors::new();
    let component = ValidatedForm::error_summary(&errors);
    let html = test::render(&component);
    assert!(html.is_empty(), "Empty error summary should render nothing");
}

#[test]
fn test_validated_input_xss_escaping() {
    let component = ValidatedForm::input("field", "<script>alert(1)</script>", Some("<img onerror=alert(1)>"));
    let html = test::render(&component);
    assert!(!html.contains("<script>"), "Should escape script tag in value");
    assert!(!html.contains("<img"), "Should escape img tag in error message");
    assert!(html.contains("&lt;script&gt;"), "Should have escaped script in value");
}

#[test]
fn test_validated_input_self_closing() {
    let component = ValidatedForm::input("name", "val", None);
    let html = test::render(&component);
    assert!(html.contains("/>"), "Input should be self-closing");
}
