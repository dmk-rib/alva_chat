use dioxus::prelude::*;
use crate::{DarkMode, Route};

#[component]
pub fn NavBar() -> Element {
    let dark_mode = use_context::<Signal<DarkMode>>();
    
    rsx! {
        header {
            class: "flex flex-wrap md:justify-start md:flex-nowrap z-50 w-full text-sm py-3 md:py-0",
            class: if dark_mode().0 { "bg-neutral-800" } else { "bg-white" },
            nav {
                aria_label: "Global",
                class: "max-w-[85rem] w-full mx-auto px-4 md:px-6 lg:px-8",
                div { class: "relative md:flex md:items-center md:justify-between",
                    div { class: "flex items-center justify-between",
                        Link { to: Route::Home {} ,
                            a {
                                aria_label: "Svaroh",
                                class: "flex-none text-xl font-semibold",
                                class: if dark_mode().0 { "text-white" },
                                "Svaroh"
                            }
                        }
                        div { class: "md:hidden",
                            button {
                                aria_controls: "navbar-collapse-with-animation",
                                aria_label: "Toggle navigation",
                                r#type: "button",
                                "data-hs-collapse": "#navbar-collapse-with-animation",
                                class: "hs-collapse-toggle flex justify-center items-center size-9 text-sm font-semibold rounded-lg border border-gray-200 text-gray-800 hover:bg-gray-100 disabled:opacity-50 disabled:pointer-events-none",
                                class: if dark_mode().0 { "text-white border-neutral-700 hover:bg-neutral-700" },
                                svg {
                                    fill: "none",
                                    stroke_linecap: "round",
                                    stroke: "currentColor",
                                    stroke_linejoin: "round",
                                    view_box: "0 0 24 24",
                                    stroke_width: "2",
                                    width: "24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    height: "24",
                                    class: "hs-collapse-open:hidden flex-shrink-0 size-4",
                                    line {
                                        x2: "21",
                                        y2: "6",
                                        x1: "3",
                                        y1: "6"
                                    }
                                    line {
                                        y2: "12",
                                        x1: "3",
                                        y1: "12",
                                        x2: "21"
                                    }
                                    line {
                                        x2: "21",
                                        y2: "18",
                                        x1: "3",
                                        y1: "18"
                                    }
                                }
                                svg {
                                    fill: "none",
                                    stroke: "currentColor",
                                    width: "24",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    height: "24",
                                    stroke_linejoin: "round",
                                    stroke_linecap: "round",
                                    stroke_width: "2",
                                    class: "hs-collapse-open:block hidden flex-shrink-0 size-4",
                                    path { d: "M18 6 6 18" }
                                    path { d: "m6 6 12 12" }
                                }
                            }
                        }
                    }
                    div {
                        class: "hs-collapse hidden overflow-hidden transition-all duration-300 basis-full grow md:block",
                        //class: "transition-all duration-300 basis-full grow md:block",
                        id: "navbar-collapse-with-animation",
                        div { 
                            class: "overflow-hidden overflow-y-auto max-h-[75vh] [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-track]:bg-gray-100 [&::-webkit-scrollbar-thumb]:bg-gray-300",
                            class: if dark_mode().0 { "[&::-webkit-scrollbar-track]:bg-neutral-700 [&::-webkit-scrollbar-thumb]:bg-neutral-500" },
                            div { 
                                class: "flex flex-col gap-x-0 mt-5 divide-y divide-dashed md:flex-row md:items-center md:justify-end md:gap-x-7 md:mt-0 md:ps-7 md:divide-y-0 md:divide-solid",
                                class: if dark_mode().0 { "divide-neutral-700" } else { "divide-gray-200" },
                                Link { to: Route::Chat { term: vec![] } ,
                                    a {
                                        aria_current: "page",
                                        class: "font-medium py-3 md:py-6",
                                        class: if dark_mode().0 { "text-blue-500" } else { "text-blue-600" },
                                        "Chat"
                                    }
                                }
                                div { class: "hs-dropdown [--strategy:static] md:[--strategy:absolute] [--adaptive:none] md:[--trigger:hover] py-3 md:py-6",
                                    button {
                                        r#type: "button",
                                        class: "flex items-center w-full font-medium",
                                        class: if dark_mode().0 { "text-neutral-200 hover:text-neutral-500" } else { "text-gray-800 hover:text-gray-600" },
                                        "\n                Company\n                "
                                        svg {
                                            width: "16",
                                            fill: "none",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            height: "16",
                                            view_box: "0 0 16 16",
                                            class: "flex-shrink-0 ms-2 size-2.5",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_width: "2",
                                                d: "M2 5L8.16086 10.6869C8.35239 10.8637 8.64761 10.8637 8.83914 10.6869L15 5",
                                                stroke: "currentColor"
                                            }
                                        }
                                    }
                                    div { 
                                        class: "hs-dropdown-menu transition-[opacity,margin] duration-[0.1ms] md:duration-[150ms] hs-dropdown-open:opacity-100 opacity-0 md:w-80 hidden z-10  md:shadow-2xl rounded-lg py-2 md:p-2  before:absolute top-full before:-top-5 before:start-0 before:w-full before:h-5",
                                        class: if dark_mode().0 { "bg-neutral-800 divide-neutral-700" } else { "bg-white" },
                                        a {
                                            href: "#",
                                            class: "inline-flex gap-x-5 w-full p-4 rounded-lg focus:ring-2 focus:ring-blue-500",
                                            class: if dark_mode().0 { "text-neutral-400 hover:bg-neutral-700 hover:text-neutral-300" } else { "text-gray-600 hover:bg-gray-100" },
                                            svg {
                                                width: "24",
                                                stroke_linecap: "round",
                                                fill: "none",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                stroke_linejoin: "round",
                                                view_box: "0 0 24 24",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                height: "24",
                                                class: "flex-shrink-0 size-5 mt-1",
                                                line {
                                                    y1: "12",
                                                    x1: "22",
                                                    y2: "12",
                                                    x2: "2"
                                                }
                                                path { d: "M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" }
                                                line {
                                                    y1: "16",
                                                    y2: "16",
                                                    x1: "6",
                                                    x2: "6.01"
                                                }
                                                line {
                                                    y1: "16",
                                                    x1: "10",
                                                    x2: "10.01",
                                                    y2: "16"
                                                }
                                            }
                                            div { class: "grow",
                                                span { 
                                                    class: "block font-semibold mb-1",
                                                    class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
                                                    "Data"
                                                }
                                                "\n                    How you get the most accurate and up-to-date data\n                  "
                                            }
                                        }
                                        div { 
                                            class: "my-2 border-t",
                                            class: if dark_mode().0 { "border-neutral-800" } else { "border-gray-100" },
                                        },
                                        a {
                                            href: "#",
                                            class: "inline-flex gap-x-5 w-full p-4 rounded-lg focus:ring-2 focus:ring-blue-500",
                                            class: if dark_mode().0 { "text-neutral-400 hover:bg-neutral-700 hover:text-neutral-300" } else { "text-gray-600 hover:bg-gray-100" },
                                            svg {
                                                stroke_width: "2",
                                                stroke_linecap: "round",
                                                view_box: "0 0 24 24",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                width: "24",
                                                height: "24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_linejoin: "round",
                                                class: "flex-shrink-0 size-5 mt-1",
                                                path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
                                                circle {
                                                    "cx": "9",
                                                    "r": "4",
                                                    "cy": "7"
                                                }
                                                path { d: "M22 21v-2a4 4 0 0 0-3-3.87" }
                                                path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
                                            }
                                            div { class: "grow",
                                                span { 
                                                    class: "block font-semibold mb-1",
                                                    class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
                                                    "Team "
                                                    span { class: "inline ms-1 text-xs bg-blue-600 text-white py-1 px-2 rounded-full",
                                                        "We're hiring"
                                                    }
                                                }
                                                "\n                    Meet the people building products to help your business grow\n                  "
                                            }
                                        }
                                        div { 
                                            class: "my-2 border-t",
                                            class: if dark_mode().0 { "border-neutral-800" } else { "border-gray-100" } 
                                        }
                                        a {
                                            href: "#",
                                            class: "inline-flex gap-x-5 w-full p-4 rounded-lg focus:ring-2 focus:ring-blue-500",
                                            class: if dark_mode().0 { "text-neutral-400 hover:bg-neutral-700 hover:text-neutral-300" } else { "text-gray-600 hover:bg-gray-100" },
                                            svg {
                                                view_box: "0 0 24 24",
                                                width: "24",
                                                fill: "none",
                                                stroke_linejoin: "round",
                                                stroke_width: "2",
                                                stroke: "currentColor",
                                                stroke_linecap: "round",
                                                height: "24",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                class: "flex-shrink-0 size-5 mt-1",
                                                path { d: "M4 22h16a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16a2 2 0 0 1-2 2Zm0 0a2 2 0 0 1-2-2v-9c0-1.1.9-2 2-2h2" }
                                                path { d: "M18 14h-8" }
                                                path { d: "M15 18h-5" }
                                                path { d: "M10 6h8v4h-8V6Z" }
                                            }
                                            div { class: "grow",
                                                span { 
                                                    class: "block font-semibold mb-1",
                                                    class: if dark_mode().0 { "text-neutral-200" } else { "text-gray-800" },
                                                    "Blog"
                                                }
                                                "\n                    The latest news, feature releases, and how to grow with data\n                  "
                                            }
                                        }
                                    }
                                }
                                a {
                                    href: "#",
                                    class: "font-medium py-3 md:py-6",
                                    class: if dark_mode().0 { "text-neutral-200 hover:text-neutral-500" } else { "text-gray-800 hover:text-gray-600" },
                                    "\n              Resources\n            "
                                }
                                a {
                                    href: "#",
                                    class: "font-medium py-3 md:py-6",
                                    class: if dark_mode().0 { "text-neutral-200 hover:text-neutral-500" } else { "text-gray-800 hover:text-gray-600" },
                                    "\n              Join us "
                                    span { class: "py-0.5 px-1.5 rounded-full text-xs font-medium bg-blue-50 border border-blue-200 text-blue-600",
                                        "4"
                                    }
                                }
                                icons::button_toggle { }
                                div { class: "pt-3 md:pt-0",
                                    a {
                                        href: "#",
                                        class: "py-2.5 px-4 inline-flex items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:pointer-events-none",
                                        "\n                Download\n              "
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}

mod icons {
    use crate::DarkMode;
    use super::*;

    #[component]
    pub(super) fn button_toggle() -> Element {
        let mut dark_mode = use_context::<Signal<DarkMode>>();

        if dark_mode().0 {
            rsx!{
                button {
                    r#type: "button",
                    class: "block group flex items-center font-medium",
                    class: if dark_mode().0 { "text-neutral-400 hover:text-neutral-500" } else { "text-gray-600 hover:text-blue-600" },
                    onclick: move |_| dark_mode.set(DarkMode(false)),
                    svg {
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        class: "flex-shrink-0 size-4",
                        circle { cx: "12", cy: "12", r: "4" }
                        path { d: "M12 2v2" }
                        path { d: "M12 20v2" }
                        path { d: "m4.93 4.93 1.41 1.41" }
                        path { d: "m17.66 17.66 1.41 1.41" }
                        path { d: "M2 12h2" }
                        path { d: "M20 12h2" }
                        path { d: "m6.34 17.66-1.41 1.41" }
                        path { d: "m19.07 4.93-1.41 1.41" }
                    },
                    
                }
            }
        } else {
            rsx! {
                button {
                    r#type: "button",
                    class: "block group flex items-center font-medium",
                    class: if dark_mode().0 { "text-neutral-400 hover:text-neutral-500" } else { "text-gray-600 hover:text-blue-600" },
                    onclick: move |_| dark_mode.set(DarkMode(true)),
                    svg {
                        view_box: "0 0 24 24",
                        stroke_width: "2",
                        xmlns: "http://www.w3.org/2000/svg",
                        stroke: "currentColor",
                        width: "24",
                        stroke_linecap: "round",
                        fill: "none",
                        stroke_linejoin: "round",
                        height: "24",
                        class: "flex-shrink-0 size-4",
                        path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                    }
                }
            }
        }
    }
}
