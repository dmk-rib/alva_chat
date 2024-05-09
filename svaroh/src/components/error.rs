use dioxus::prelude::*;

#[component]
pub fn Error() -> Element {
    rsx!(
        section { class: "py-20",
            div { class: "container mx-auto px-4",
                div { class: "flex flex-wrap -mx-4 mb-24 text-center",
                    "An internal error has occurred"
                }
            }
        }
    )
}