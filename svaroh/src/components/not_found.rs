use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "h-screen flex flex-col pb-6",
            div { class: "max-w-[50rem] flex flex-col mx-auto size-full",
            header { class: "mb-auto flex justify-center z-50 w-full py-4",
                nav { aria_label: "Global", class: "px-4 sm:px-6 lg:px-8",
                    Link { to: Route::Home { } ,
                        a {
                            aria_label: "brand",
                            class: "flex-none text-xl font-semibold sm:text-3xl", // dark:text-white",
                            "Svaroh"
                        }
                    }
                }
            }
            main { id: "content",
                div { class: "text-center py-10 px-4 sm:px-6 lg:px-8",
                    h1 { class: "block text-7xl font-bold text-gray-800 sm:text-9xl", // dark:text-white",
                        "404"
                    }
                    p { class: "mt-3 text-gray-600", "Oops, something went wrong." } //dark:text-neutral-400
                    p { class: "text-gray-600", //dark:text-neutral-400
                        "Sorry, we couldn't find your page."
                    }
                    div { class: "mt-5 flex flex-col justify-center items-center gap-2 sm:flex-row sm:gap-3",
                        Link { to: Route::Home { } ,
                            a {
                                class: "w-full sm:w-auto py-3 px-4 inline-flex justify-center items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:pointer-events-none",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke_linejoin: "round",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    stroke_linecap: "round",
                                    width: "24",
                                    height: "24",
                                    stroke_width: "2",
                                    stroke: "currentColor",
                                    class: "flex-shrink-0 size-4",
                                    path { "d": "m15 18-6-6 6-6" }
                                }
                                "\n          Back to home\n        "
                            }
                        }
                    }
                }
            }
            footer { class: "mt-auto text-center py-5",
                div { class: "max-w-[85rem] mx-auto px-4 sm:px-6 lg:px-8",
                    p { class: "text-sm text-gray-500", // dark:text-neutral-500",
                        "© All Rights Reserved. 2024."
                    }
                }
            }
            }
        }
    }
}
