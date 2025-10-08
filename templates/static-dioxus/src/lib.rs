#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn app() {
  dioxus_web::launch(App);
}

pub fn App(cx: Scope) -> Element {
  let mut count = use_state(cx, || 0);

  cx.render(rsx! {
    body {
      class: "bg-gray-900 text-white flex items-center justify-center h-screen",
      div {
        class: "text-center",
        h1 { class: "text-5xl font-bold mb-4", "Hello from Forge + Dioxus! 🚀"}
        p {
          class: "text-xl mb-8",
          "Count: ",
          span { class: "font-mono text-green-400", "{count}" }
        }
        button {
          class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
          onclick: move |_| count += 1,
          "Click to increment"
        }
      }
    }
  })
}