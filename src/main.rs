use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Main {}

    }
}

#[component]
pub fn Main() -> Element {
    let mut item: Signal<String> = use_signal(|| { String::new() });

    let mut items: Signal<Vec<String>> = use_signal(Vec::<String>::new);

    rsx! {
        div {
            div {
                class : "header",
                input {
                    type : "text",
                    class : "input",
                    oninput : move |event| {
                        item.set(event.value());
                    },
                    onkeydown : move |event| {
                        if event.code().to_string() == "Enter".to_string() {
                            println!("{:?}", item);
                            items.write().push(item());
                            println!("{:?}", items);
                        }
                    }
                }
            },
            div {
                for item in items.iter() {
                    div {
                        class : "todo-item",
                        label { {item.to_string()} },
                        button {
                            class : "delete-button",
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}
