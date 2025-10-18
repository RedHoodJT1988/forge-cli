#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

#[server(GetItems, "/api")]
pub async fn get_items() -> Result<Vec<Item>, ServerFnError> {
    unreachable!()
}

#[server(AddItem, "/api")]
pub async fn add_item(name: String) -> Result<(), ServerFnError> {
    unreachable!()
}

pub fn app(cx: Scope) -> Element {
    let items = use_resource(cx, || get_items());
    let new_item_name = use_state(cx, String::new);

    let item_list = match &*items.read() {
        Some(Ok(items)) if !items.is_empty() => rsx! {
            ul { class: "list-none text-left",
                for item in items {
                    li { class: "bg-gray-800 p-3 my-2 rounded-lg", "{item.name}" }
                }
            }
        },
        Some(Ok(_)) => rsx! { p { class: "text-gray-400", "No items yet. Add one below!"} },
        Some(Err(e)) => rsx! { p { class: "text-red-500", "Error loading items: {e}" } },
        None => rsx! { p { "Loading..." } },
    };

    cx.render(rsx! {
        body { class: "bg-gray-800 text-white flex justify-center py-12",
            main { class: "container mx-auto p-8 text-center border border-gray-600 rounded-lg shadow-xl bg-gray-900 w-1/2",
                h1 { class: "text-5xl font-bold mb-4", "Todo List 📝" }
                p { class: "text-xs mt-4 mb-8 text-gray-500", "Note: Requires an 'items' table with 'id' (int primary key auto_increment) and 'name' (varchar) columns." }

                div { id: "item-list", {item_list} }

                form { class: "mt-8",
                    onsubmit: move |_| {
                        to_owned![items, new_item_name];
                        if !new_item_name.get().is_empty() {
                            cx.spawn(async move {
                                if add_item(new_item_name.get().clone()).await.is_ok() {
                                    items.restart();
                                    new_item_name.set("".to_string());
                                }
                            });
                        }
                    },
                    input {
                        class: "bg-gray-700 text-white p-3 rounded-l-lg",
                        placeholder: "New todo item...",
                        value: "{new_item_name}",
                        oninput: move |evt| new_item_name.set(evt.value.clone()),
                    }
                    button { class: "bg-blue-600 hover:bg-blue-700 text-white font-bold p-3 rounded-r-lg", r#type: "submit", "Add Item" }
                }
            }
        }
    })
}