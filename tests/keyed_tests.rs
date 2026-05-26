use azumi::{component, html, render_to_string};

// ═════════════════════════════════════════════════
// @keyed Keyed List Update Tests
// ═════════════════════════════════════════════════

#[derive(Clone)]
struct Item {
    id: i32,
    name: String,
}

#[test]
fn test_keyed_generates_data_key_attribute() {
    let items = vec![
        Item { id: 1, name: "First".to_string() },
        Item { id: 2, name: "Second".to_string() },
    ];

    #[azumi::component]
    fn item_list(items: Vec<Item>) -> impl azumi::Component {
        html! {
            <div class={"list"}>
                @for item in &items @keyed(item.id) {
                    <div class={"row"}>{&item.name}</div>
                }
            </div>
        }
    }

    let output = render_to_string(&item_list::render(
        item_list::Props::builder().items(items.clone()).build().unwrap(),
    ));
    assert!(
        output.contains("data-key="),
        "Output should contain data-key attribute. Got: {}",
        output
    );
    let count = output.matches("data-key=").count();
    assert_eq!(
        count, 2,
        "Should have 2 data-key attributes (one per item). Got: {} (count={})",
        output, count
    );
}

#[test]
fn test_keyed_preserves_class_attribute() {
    let items = vec![Item { id: 1, name: "A".to_string() }];

    #[azumi::component]
    fn class_list(items: Vec<Item>) -> impl azumi::Component {
        html! {
            <ul>
                @for item in &items @keyed(item.id) {
                    <li class={"list-item"}>{&item.name}</li>
                }
            </ul>
        }
    }

    let output = render_to_string(&class_list::render(
        class_list::Props::builder().items(items).build().unwrap(),
    ));
    assert!(output.contains("data-key="), "Should have data-key. Got: {}", output);
    assert!(output.contains("class=\"list-item\""), "Should generate class. Got: {}", output);
}

#[test]
fn test_for_without_keyed_no_data_key() {
    let items = vec![
        Item { id: 1, name: "A".to_string() },
        Item { id: 2, name: "B".to_string() },
    ];

    #[azumi::component]
    fn no_key_list(items: Vec<Item>) -> impl azumi::Component {
        html! {
            <div>
                @for item in &items {
                    <span>{&item.name}</span>
                }
            </div>
        }
    }

    let output = render_to_string(&no_key_list::render(
        no_key_list::Props::builder().items(items).build().unwrap(),
    ));
    assert!(
        !output.contains("data-key="),
        "Without @keyed, no data-key should be generated. Got: {}",
        output
    );
}

#[test]
fn test_keyed_with_empty_list() {
    let items: Vec<Item> = vec![];

    #[azumi::component]
    fn empty_list(items: Vec<Item>) -> impl azumi::Component {
        html! {
            <div>
                @for item in &items @keyed(item.id) {
                    <div>{&item.name}</div>
                }
            </div>
        }
    }

    let output = render_to_string(&empty_list::render(
        empty_list::Props::builder().items(items).build().unwrap(),
    ));
    assert!(
        !output.contains("data-key="),
        "Empty list should not generate data-key. Got: {}",
        output
    );
}

#[test]
fn test_keyed_with_string_keys() {
    #[azumi::component]
    fn string_key_list(items: Vec<String>) -> impl azumi::Component {
        html! {
            <div>
                @for item in &items @keyed(item) {
                    <span>{item}</span>
                }
            </div>
        }
    }

    let items = vec!["alpha".to_string(), "beta".to_string()];
    let output = render_to_string(&string_key_list::render(
        string_key_list::Props::builder().items(items).build().unwrap(),
    ));
    assert!(output.contains("data-key="), "String keys should work. Got: {}", output);
    let count = output.matches("data-key=").count();
    assert_eq!(count, 2, "Should have 2 data-keys. Got: {}", output);
}

#[test]
fn test_keyed_only_first_element_gets_data_key() {
    let items = vec![Item { id: 1, name: "A".to_string() }];

    #[azumi::component]
    fn nested_items(items: Vec<Item>) -> impl azumi::Component {
        html! {
            <div>
                @for item in &items @keyed(item.id) {
                    <div class={"outer"}>
                        <span class={"inner"}>{&item.name}</span>
                    </div>
                }
            </div>
        }
    }

    let output = render_to_string(&nested_items::render(
        nested_items::Props::builder().items(items).build().unwrap(),
    ));
    let count = output.matches("data-key=").count();
    assert_eq!(
        count, 1,
        "Only first element per iteration should get data-key. Got: {} (count={})",
        output, count
    );
}

#[test]
fn test_keyed_with_if_inside_loop() {
    let items = vec![
        Item { id: 1, name: "A".to_string() },
        Item { id: 2, name: "B".to_string() },
    ];

    #[azumi::component]
    fn conditional_list(items: Vec<Item>) -> impl azumi::Component {
        html! {
            <div>
                @for item in &items @keyed(item.id) {
                    @if item.id > 0 {
                        <div class={"row"}>{&item.name}</div>
                    }
                }
            </div>
        }
    }

    let output = render_to_string(&conditional_list::render(
        conditional_list::Props::builder().items(items).build().unwrap(),
    ));
    assert!(output.contains("data-key="), "Keyed with @if should work. Got: {}", output);
}

#[test]
fn test_keyed_integration_with_component_state() {
    #[derive(Clone)]
    struct Task {
        id: i32,
        title: String,
    }

    #[azumi::component]
    fn task_view(tasks: Vec<Task>) -> impl azumi::Component {
        html! {
            <div class={"tasks"}>
                @for task in &tasks @keyed(task.id) {
                    <div class={"task-row"}>{&task.title}</div>
                }
            </div>
        }
    }

    let tasks = vec![
        Task { id: 1, title: "Buy milk".to_string() },
        Task { id: 2, title: "Walk dog".to_string() },
    ];
    let output = render_to_string(&task_view::render(
        task_view::Props::builder().tasks(tasks).build().unwrap(),
    ));
    assert!(output.contains("data-key="), "Keyed with component state should work. Got: {}", output);
    let count = output.matches("data-key=").count();
    assert_eq!(count, 2, "Should have 2 data-keys. Got: {}", output);
}
