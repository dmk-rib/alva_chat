use std::{ str::FromStr, fmt::Display };

use dioxus::prelude::*;
use crate::{ api::*, Route };

#[component]
pub fn Chat(term: Vec<String>) -> Element {
    rsx! {
        div {
            class: "hs-overlay [--auto-close:lg] hs-overlay-open:translate-x-0 -translate-x-full duration-300 transform hidden fixed top-0 start-0 bottom-0 z-[60] w-64 bg-white border-e border-gray-200 overflow-y-auto lg:block lg:translate-x-0 lg:end-auto lg:bottom-0 [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-track]:bg-gray-100 [&::-webkit-scrollbar-thumb]:bg-gray-300 ", //dark:[&::-webkit-scrollbar-track]:bg-neutral-700 ", //dark:[&::-webkit-scrollbar-thumb]:bg-neutral-500 ", //dark:bg-neutral-900 ", //dark:border-neutral-700",
            id: "application-sidebar",
            nav {
                "data-hs-accordion-always-open": "false",
                class: "hs-accordion-group size-full flex flex-col",
                div { class: "flex items-center justify-between pt-4 pe-4 ps-7",
                    Link { to: Route::Home { } ,
                        icons::svaroh_icon {}
                    }
                }
                div { class: "h-full",
                    ul { class: "space-y-1.5 p-4",
                        li {
                            a {
                                href: "#",
                                class: "flex items-center gap-x-3 py-2 px-3 text-sm text-gray-700 rounded-lg hover:bg-gray-100 ", //dark:hover:bg-neutral-900 ", //dark:text-neutral-400 ", //dark:hover:text-neutral-300",
                                svg {
                                    width: "24",
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    "xmlns": "http://www.w3.org/2000/svg",
                                    "fill": "none",
                                    "viewBox": "0 0 24 24",
                                    "stroke": "currentColor",
                                    height: "24",
                                    "stroke-linecap": "round",
                                    class: "flex-shrink-0 size-4",
                                    path { "d": "M5 12h14" }
                                    path { "d": "M12 5v14" }
                                }
                                "\n              New chat\n            "
                            }
                        }
                        li {
                            a {
                                href: "#",
                                class: "flex items-center gap-x-3 py-2 px-3 text-sm text-gray-700 rounded-lg hover:bg-gray-100 ", //dark:hover:bg-neutral-900 ", //dark:text-neutral-400 ", //dark:hover:text-neutral-300",
                                svg {
                                    "fill": "currentColor",
                                    "xmlns": "http://www.w3.org/2000/svg",
                                    width: "16",
                                    "viewBox": "0 0 16 16",
                                    height: "16",
                                    class: "flex-shrink-0 size-4",
                                    path {
                                        "d": "M13.545 2.907a13.227 13.227 0 0 0-3.257-1.011.05.05 0 0 0-.052.025c-.141.25-.297.577-.406.833a12.19 12.19 0 0 0-3.658 0 8.258 8.258 0 0 0-.412-.833.051.051 0 0 0-.052-.025c-1.125.194-2.22.534-3.257 1.011a.041.041 0 0 0-.021.018C.356 6.024-.213 9.047.066 12.032c.001.014.01.028.021.037a13.276 13.276 0 0 0 3.995 2.02.05.05 0 0 0 .056-.019c.308-.42.582-.863.818-1.329a.05.05 0 0 0-.01-.059.051.051 0 0 0-.018-.011 8.875 8.875 0 0 1-1.248-.595.05.05 0 0 1-.02-.066.051.051 0 0 1 .015-.019c.084-.063.168-.129.248-.195a.05.05 0 0 1 .051-.007c2.619 1.196 5.454 1.196 8.041 0a.052.052 0 0 1 .053.007c.08.066.164.132.248.195a.051.051 0 0 1-.004.085 8.254 8.254 0 0 1-1.249.594.05.05 0 0 0-.03.03.052.052 0 0 0 .003.041c.24.465.515.909.817 1.329a.05.05 0 0 0 .056.019 13.235 13.235 0 0 0 4.001-2.02.049.049 0 0 0 .021-.037c.334-3.451-.559-6.449-2.366-9.106a.034.034 0 0 0-.02-.019Zm-8.198 7.307c-.789 0-1.438-.724-1.438-1.612 0-.889.637-1.613 1.438-1.613.807 0 1.45.73 1.438 1.613 0 .888-.637 1.612-1.438 1.612Zm5.316 0c-.788 0-1.438-.724-1.438-1.612 0-.889.637-1.613 1.438-1.613.807 0 1.451.73 1.438 1.613 0 .888-.631 1.612-1.438 1.612Z"
                                    }
                                }
                                "\n              Preline AI Discord\n            "
                            }
                        }
                        li {
                            a {
                                href: "#",
                                class: "flex items-center gap-x-3 py-2 px-3 text-sm text-gray-700 rounded-lg hover:bg-gray-100 ", //dark:hover:bg-neutral-900 ", //dark:text-neutral-400 ", //dark:hover:text-neutral-300",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "stroke-linecap": "round",
                                    "xmlns": "http://www.w3.org/2000/svg",
                                    width: "24",
                                    "stroke-width": "2",
                                    height: "24",
                                    "stroke": "currentColor",
                                    "fill": "none",
                                    "stroke-linejoin": "round",
                                    class: "flex-shrink-0 size-4",
                                    path { "d": "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                    polyline { "points": "7 10 12 15 17 10" }
                                    line {
                                        "x2": "12",
                                        "y2": "3",
                                        "y1": "15",
                                        "x1": "12"
                                    }
                                }
                                "\n              Save conversation\n            "
                            }
                        }
                        li {
                            a {
                                href: "#",
                                class: "flex items-center gap-x-3 py-2 px-3 text-sm text-gray-700 rounded-lg hover:bg-gray-100 ", //dark:hover:bg-neutral-900 ", //dark:text-neutral-400 ", //dark:hover:text-neutral-300",
                                svg {
                                    "stroke-linejoin": "round",
                                    "xmlns": "http://www.w3.org/2000/svg",
                                    "stroke-linecap": "round",
                                    "stroke": "currentColor",
                                    height: "24",
                                    width: "24",
                                    "fill": "none",
                                    "viewBox": "0 0 24 24",
                                    "stroke-width": "2",
                                    class: "flex-shrink-0 size-4",
                                    path { "d": "m22 2-7 20-4-9-9-4Z" }
                                    path { "d": "M22 2 11 13" }
                                }
                                "\n              Updates & FAQ\n            "
                            }
                        }
                        li {
                            a {
                                href: "#",
                                class: "flex items-center gap-x-3 py-2 px-3 text-sm text-gray-700 rounded-lg hover:bg-gray-100 ", //dark:hover:bg-neutral-900 ", //dark:text-neutral-400 ", //dark:hover:text-neutral-300",
                                svg {
                                    "xmlns": "http://www.w3.org/2000/svg",
                                    "fill": "none",
                                    "stroke-width": "2",
                                    "stroke": "currentColor",
                                    "stroke-linejoin": "round",
                                    width: "24",
                                    height: "24",
                                    "viewBox": "0 0 24 24",
                                    "stroke-linecap": "round",
                                    class: "flex-shrink-0 size-4 text-blue-600",
                                    polygon { "points": "13 2 3 14 12 14 11 22 21 10 12 10 13 2" }
                                }
                                span { class: "bg-clip-text bg-gradient-to-tl from-blue-600 to-violet-600 text-transparent",
                                    "Upgrade Plan"
                                }
                            }
                        }
                    }
                }
                div { class: "mt-auto",
                    div { class: "py-2.5 px-7",
                        p { class: "inline-flex items-center gap-x-2 text-xs text-green-600",
                            span { class: "block size-1.5 rounded-full bg-green-600" }
                            "\n            Active 12,320 people\n          "
                        }
                    }
                    div { class: "p-4 border-t border-gray-200 ", //dark:border-neutral-700",
                        a {
                            href: "#",
                            class: "flex justify-between items-center gap-x-3 py-2 px-3 text-sm text-gray-700 rounded-lg hover:bg-gray-100 ", //dark:hover:bg-neutral-900 ", //dark:text-neutral-400 ", //dark:hover:text-neutral-300",
                            "\n            Sign in\n            "
                            svg {
                                "xmlns": "http://www.w3.org/2000/svg",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                height: "24",
                                "fill": "none",
                                width: "24",
                                "stroke": "currentColor",
                                "stroke-linejoin": "round",
                                "viewBox": "0 0 24 24",
                                class: "flex-shrink-0 size-4",
                                path { "d": "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" }
                                polyline { "points": "10 17 15 12 10 7" }
                                line {
                                    "y2": "12",
                                    "x2": "3",
                                    "y1": "12",
                                    "x1": "15"
                                }
                            }
                        }
                    }
                }
            }
        }
        div { class: "relative h-screen w-full lg:ps-64",
            div { class: "py-10 lg:py-14",
                div { class: "max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto text-center",
                    h1 { class: "text-3xl font-bold text-gray-800 sm:text-4xl ", //dark:text-white",
                        "\n          Welcome to Preline AI\n        "
                    }
                    p { class: "mt-3 text-gray-600 ", //dark:text-neutral-400",
                        "\n          Your AI-powered copilot for the web\n        "
                    }
                }
                ul { class: "mt-16 space-y-5",
                    li { class: "max-w-4xl py-2 px-4 sm:px-6 lg:px-8 mx-auto flex gap-x-2 sm:gap-x-4",
                        svg {
                            "xmlns": "http://www.w3.org/2000/svg",
                            "fill": "none",
                            height: "38",
                            width: "38",
                            "viewBox": "0 0 38 38",
                            class: "flex-shrink-0 w-[2.375rem] h-[2.375rem] rounded-full",
                            rect {
                                width: "38",
                                height: "38",
                                "rx": "6",
                                "fill": "#2563EB"
                            }
                            path {
                                "stroke": "white",
                                "stroke-width": "1.5",
                                "d": "M10 28V18.64C10 13.8683 14.0294 10 19 10C23.9706 10 28 13.8683 28 18.64C28 23.4117 23.9706 27.28 19 27.28H18.25"
                            }
                            path {
                                "stroke": "white",
                                "stroke-width": "1.5",
                                "d": "M13 28V18.7552C13 15.5104 15.6863 12.88 19 12.88C22.3137 12.88 25 15.5104 25 18.7552C25 22 22.3137 24.6304 19 24.6304H18.25"
                            }
                            ellipse {
                                "rx": "3.75",
                                "ry": "3.6",
                                "fill": "white",
                                "cx": "19",
                                "cy": "18.6554"
                            }
                        }
                        div { class: "space-y-3",
                            h2 { class: "font-medium text-gray-800 ", //dark:text-white",
                                "\n              How can we help?\n            "
                            }
                            div { class: "space-y-1.5",
                                p { class: "mb-1.5 text-sm text-gray-800 ", //dark:text-white",
                                    "\n                You can ask questions like:\n              "
                                }
                                ul { class: "list-disc list-outside space-y-1.5 ps-3.5",
                                    li { class: "text-sm text-gray-800 ", //dark:text-white",
                                        "\n                  What's Preline UI?\n                "
                                    }
                                    li { class: "text-sm text-gray-800 ", //dark:text-white",
                                        "\n                  How many Starter Pages & Examples are there?\n                "
                                    }
                                    li { class: "text-sm text-gray-800 ", //dark:text-white",
                                        "\n                  Is there a PRO version?\n                "
                                    }
                                }
                            }
                        }
                    }
                    li { class: "py-2 sm:py-4",
                        div { class: "max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto",
                            div { class: "max-w-2xl flex gap-x-2 sm:gap-x-4",
                                span { class: "flex-shrink-0 inline-flex items-center justify-center size-[38px] rounded-full bg-gray-600",
                                    span { class: "text-sm font-medium text-white leading-none",
                                        "AZ"
                                    }
                                }
                                div { class: "grow mt-2 space-y-3",
                                    p { class: "text-gray-800 ", //dark:text-neutral-200",
                                        "\n                  what's preline ui?\n                "
                                    }
                                }
                            }
                        }
                    }
                    li { class: "max-w-4xl py-2 px-4 sm:px-6 lg:px-8 mx-auto flex gap-x-2 sm:gap-x-4",
                        svg {
                            height: "38",
                            "fill": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 38 38",
                            width: "38",
                            class: "flex-shrink-0 w-[2.375rem] h-[2.375rem] rounded-full",
                            rect {
                                "fill": "#2563EB",
                                "rx": "6",
                                width: "38",
                                height: "38"
                            }
                            path {
                                "d": "M10 28V18.64C10 13.8683 14.0294 10 19 10C23.9706 10 28 13.8683 28 18.64C28 23.4117 23.9706 27.28 19 27.28H18.25",
                                "stroke": "white",
                                "stroke-width": "1.5"
                            }
                            path {
                                "stroke-width": "1.5",
                                "d": "M13 28V18.7552C13 15.5104 15.6863 12.88 19 12.88C22.3137 12.88 25 15.5104 25 18.7552C25 22 22.3137 24.6304 19 24.6304H18.25",
                                "stroke": "white"
                            }
                            ellipse {
                                "cy": "18.6554",
                                "cx": "19",
                                "rx": "3.75",
                                "fill": "white",
                                "ry": "3.6"
                            }
                        }
                        div { class: "grow max-w-[90%] md:max-w-2xl w-full space-y-3",
                            div { class: "space-y-3",
                                p { class: "text-sm text-gray-800 ", //dark:text-white",
                                    "\n                Preline UI is an open-source set of prebuilt UI components based on the utility-first Tailwind CSS framework.\n              "
                                }
                                div { class: "space-y-1.5",
                                    p { class: "text-sm text-gray-800 ", //dark:text-white",
                                        "\n                  Here're some links to get started\n                "
                                    }
                                    ul {
                                        li {
                                            a {
                                                href: "../docs/index.html",
                                                class: "text-sm text-blue-600 decoration-2 hover:underline font-medium ", //dark:text-blue-500 ", //dark:hover:text-blue-400",
                                                "\n                      Installation Guide\n                    "
                                            }
                                        }
                                        li {
                                            a {
                                                href: "../docs/frameworks.html",
                                                class: "text-sm text-blue-600 decoration-2 hover:underline font-medium ", //dark:text-blue-500 ", //dark:hover:text-blue-400",
                                                "\n                      Framework Guides\n                    "
                                            }
                                        }
                                    }
                                }
                            }
                            div {
                                div { class: "sm:flex sm:justify-between",
                                    div {
                                        div { class: "inline-flex border border-gray-200 rounded-full p-0.5 ", //dark:border-neutral-700",
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-linecap": "round",
                                                    "fill": "none",
                                                    width: "24",
                                                    height: "24",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "stroke": "currentColor",
                                                    "stroke-width": "2",
                                                    "stroke-linejoin": "round",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M7 10v12" }
                                                    path { "d": "M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z" }
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    "stroke-width": "2",
                                                    "stroke-linejoin": "round",
                                                    "stroke-linecap": "round",
                                                    "viewBox": "0 0 24 24",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "fill": "none",
                                                    height: "24",
                                                    width: "24",
                                                    "stroke": "currentColor",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M17 14V2" }
                                                    path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                                }
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                width: "24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "fill": "none",
                                                height: "24",
                                                "stroke-linecap": "round",
                                                "stroke": "currentColor",
                                                "stroke-width": "2",
                                                "viewBox": "0 0 24 24",
                                                "stroke-linejoin": "round",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M17 14V2" }
                                                path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                            }
                                            "\n                    Copy\n                  "
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "viewBox": "0 0 24 24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke-width": "2",
                                                height: "24",
                                                "stroke-linejoin": "round",
                                                width: "24",
                                                "fill": "none",
                                                "stroke": "currentColor",
                                                "stroke-linecap": "round",
                                                class: "flex-shrink-0 size-4",
                                                circle {
                                                    "cy": "5",
                                                    "r": "3",
                                                    "cx": "18"
                                                }
                                                circle {
                                                    "cy": "12",
                                                    "cx": "6",
                                                    "r": "3"
                                                }
                                                circle {
                                                    "r": "3",
                                                    "cx": "18",
                                                    "cy": "19"
                                                }
                                                line {
                                                    "y1": "13.51",
                                                    "x2": "15.42",
                                                    "x1": "8.59",
                                                    "y2": "17.49"
                                                }
                                                line {
                                                    "x1": "15.41",
                                                    "y1": "6.51",
                                                    "x2": "8.59",
                                                    "y2": "10.49"
                                                }
                                            }
                                            "\n                    Share\n                  "
                                        }
                                    }
                                    div { class: "mt-1 sm:mt-0",
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                height: "24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke-linecap": "round",
                                                "stroke": "currentColor",
                                                width: "24",
                                                "viewBox": "0 0 24 24",
                                                "stroke-linejoin": "round",
                                                "fill": "none",
                                                "stroke-width": "2",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" }
                                                path { "d": "M21 3v5h-5" }
                                            }
                                            "\n                    New answer\n                  "
                                        }
                                    }
                                }
                            }
                        }
                    }
                    li { class: "py-2 sm:py-4",
                        div { class: "max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto",
                            div { class: "max-w-2xl flex gap-x-2 sm:gap-x-4",
                                span { class: "flex-shrink-0 inline-flex items-center justify-center size-[38px] rounded-full bg-gray-600",
                                    span { class: "text-sm font-medium text-white leading-none",
                                        "AZ"
                                    }
                                }
                                div { class: "grow mt-2 space-y-3",
                                    p { class: "text-gray-800 ", //dark:text-neutral-200",
                                        "\n                  what's preline ui figma?\n                "
                                    }
                                    div { class: "mt-3",
                                        button {
                                            r#type: "button",
                                            class: "p-2 inline-flex justify-center items-center gap-x-1 rounded-lg border font-medium bg-white text-gray-700 shadow-sm align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-white focus:ring-blue-600 text-xs ", //dark:bg-neutral-900 ", //dark:hover:bg-neutral-800 ", //dark:border-neutral-700 ", //dark:text-neutral-400 ", //dark:hover:text-white ", //dark:focus:ring-offset-gray-800",
                                            svg {
                                                "viewBox": "0 0 24 24",
                                                "stroke-linecap": "round",
                                                width: "24",
                                                "stroke-width": "2",
                                                height: "24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "fill": "none",
                                                "stroke-linejoin": "round",
                                                "stroke": "currentColor",
                                                class: "flex-shrink-0 size-4",
                                                polygon { "points": "5 3 19 12 5 21 5 3" }
                                            }
                                            "\n                    Voice message\n                  "
                                        }
                                    }
                                }
                            }
                        }
                    }
                    li { class: "max-w-4xl py-2 px-4 sm:px-6 lg:px-8 mx-auto flex gap-x-2 sm:gap-x-4",
                        svg {
                            "viewBox": "0 0 38 38",
                            "fill": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            width: "38",
                            height: "38",
                            class: "flex-shrink-0 w-[2.375rem] h-[2.375rem] rounded-full",
                            rect {
                                height: "38",
                                "rx": "6",
                                width: "38",
                                "fill": "#2563EB"
                            }
                            path {
                                "stroke": "white",
                                "stroke-width": "1.5",
                                "d": "M10 28V18.64C10 13.8683 14.0294 10 19 10C23.9706 10 28 13.8683 28 18.64C28 23.4117 23.9706 27.28 19 27.28H18.25"
                            }
                            path {
                                "stroke": "white",
                                "d": "M13 28V18.7552C13 15.5104 15.6863 12.88 19 12.88C22.3137 12.88 25 15.5104 25 18.7552C25 22 22.3137 24.6304 19 24.6304H18.25",
                                "stroke-width": "1.5"
                            }
                            ellipse {
                                "cx": "19",
                                "rx": "3.75",
                                "ry": "3.6",
                                "cy": "18.6554",
                                "fill": "white"
                            }
                        }
                        div { class: "grow max-w-[90%] md:max-w-2xl w-full space-y-3",
                            div { class: "space-y-3",
                                p { class: "text-sm text-gray-800 ", //dark:text-white",
                                    "\n                Preline UI Figma is the largest free design system for Figma, crafted with Tailwind CSS styles and Preline UI components with extra top-notch additions.\n              "
                                }
                                div { class: "space-y-1.5",
                                    p { class: "text-sm text-gray-800 ", //dark:text-white",
                                        "\n                  With the features like:\n                "
                                    }
                                    ul { class: "list-disc list-outside space-y-1.5 ps-3.5",
                                        li { class: "text-sm text-gray-800 ", //dark:text-white",
                                            "\n                    12-column Grid System\n                  "
                                        }
                                        li { class: "text-sm text-gray-800 ", //dark:text-white",
                                            "\n                    Easily find UI elements\n                  "
                                        }
                                        li { class: "text-sm text-gray-800 ", //dark:text-white",
                                            "\n                    Variants and Properties\n                  "
                                        }
                                        li { class: "text-sm text-gray-800 ", //dark:text-white",
                                            "\n                    Tailwind CSS Color styles\n                  "
                                        }
                                        li { class: "text-sm text-gray-800 ", //dark:text-white",
                                            "\n                    Auto layout and constraints\n                  "
                                        }
                                    }
                                }
                            }
                            div {
                                div { class: "sm:flex sm:justify-between",
                                    div {
                                        div { class: "inline-flex border border-gray-200 rounded-full p-0.5 ", //dark:border-neutral-700",
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    "fill": "none",
                                                    height: "24",
                                                    "stroke-linejoin": "round",
                                                    "stroke": "currentColor",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-linecap": "round",
                                                    "stroke-width": "2",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    width: "24",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M7 10v12" }
                                                    path { "d": "M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z" }
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    "stroke": "currentColor",
                                                    "stroke-linecap": "round",
                                                    "stroke-linejoin": "round",
                                                    "stroke-width": "2",
                                                    height: "24",
                                                    "viewBox": "0 0 24 24",
                                                    width: "24",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "fill": "none",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M17 14V2" }
                                                    path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                                }
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "stroke-width": "2",
                                                "stroke": "currentColor",
                                                "fill": "none",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "viewBox": "0 0 24 24",
                                                width: "24",
                                                "stroke-linecap": "round",
                                                height: "24",
                                                "stroke-linejoin": "round",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M17 14V2" }
                                                path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                            }
                                            "\n                    Copy\n                  "
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                height: "24",
                                                "stroke-linecap": "round",
                                                "stroke-linejoin": "round",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke": "currentColor",
                                                "stroke-width": "2",
                                                width: "24",
                                                "fill": "none",
                                                "viewBox": "0 0 24 24",
                                                class: "flex-shrink-0 size-4",
                                                circle {
                                                    "cy": "5",
                                                    "cx": "18",
                                                    "r": "3"
                                                }
                                                circle {
                                                    "cx": "6",
                                                    "r": "3",
                                                    "cy": "12"
                                                }
                                                circle {
                                                    "r": "3",
                                                    "cx": "18",
                                                    "cy": "19"
                                                }
                                                line {
                                                    "y1": "13.51",
                                                    "y2": "17.49",
                                                    "x2": "15.42",
                                                    "x1": "8.59"
                                                }
                                                line {
                                                    "y2": "10.49",
                                                    "x1": "15.41",
                                                    "y1": "6.51",
                                                    "x2": "8.59"
                                                }
                                            }
                                            "\n                    Share\n                  "
                                        }
                                    }
                                    div { class: "mt-1 sm:mt-0",
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "fill": "none",
                                                "stroke": "currentColor",
                                                "stroke-linejoin": "round",
                                                "stroke-width": "2",
                                                height: "24",
                                                "stroke-linecap": "round",
                                                "viewBox": "0 0 24 24",
                                                width: "24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" }
                                                path { "d": "M21 3v5h-5" }
                                            }
                                            "\n                    New answer\n                  "
                                        }
                                    }
                                }
                            }
                        }
                    }
                    li { class: "py-2 sm:py-4",
                        div { class: "max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto",
                            div { class: "max-w-2xl flex gap-x-2 sm:gap-x-4",
                                span { class: "flex-shrink-0 inline-flex items-center justify-center size-[38px] rounded-full bg-gray-600",
                                    span { class: "text-sm font-medium text-white leading-none",
                                        "AZ"
                                    }
                                }
                                div { class: "grow mt-2 space-y-3",
                                    p { class: "text-gray-800 ", //dark:text-neutral-200",
                                        "\n                  create a table example with preline using avatars, badges and progress bars\n                "
                                    }
                                }
                            }
                        }
                    }
                    li { class: "max-w-4xl py-2 px-4 sm:px-6 lg:px-8 mx-auto flex gap-x-2 sm:gap-x-4",
                        svg {
                            "fill": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            width: "38",
                            height: "38",
                            "viewBox": "0 0 38 38",
                            class: "flex-shrink-0 w-[2.375rem] h-[2.375rem] rounded-full",
                            rect {
                                "fill": "#2563EB",
                                height: "38",
                                "rx": "6",
                                width: "38"
                            }
                            path {
                                "stroke": "white",
                                "stroke-width": "1.5",
                                "d": "M10 28V18.64C10 13.8683 14.0294 10 19 10C23.9706 10 28 13.8683 28 18.64C28 23.4117 23.9706 27.28 19 27.28H18.25"
                            }
                            path {
                                "d": "M13 28V18.7552C13 15.5104 15.6863 12.88 19 12.88C22.3137 12.88 25 15.5104 25 18.7552C25 22 22.3137 24.6304 19 24.6304H18.25",
                                "stroke": "white",
                                "stroke-width": "1.5"
                            }
                            ellipse {
                                "cy": "18.6554",
                                "cx": "19",
                                "rx": "3.75",
                                "ry": "3.6",
                                "fill": "white"
                            }
                        }
                        div { class: "grow max-w-[90%] md:max-w-2xl w-full space-y-3",
                            div { class: "space-y-3",
                                p { class: "text-sm text-gray-800 ", //dark:text-white",
                                    "\n                Hold on a sec...\n              "
                                }
                            }
                            div { class: "bg-white border border-gray-200 rounded-xl shadow-sm overflow-hidden ", //dark:bg-neutral-900 ", //dark:border-neutral-700",
                                div { class: "-m-1.5 overflow-x-auto",
                                    div { class: "p-1.5 min-w-full inline-block align-middle",
                                        table { class: "min-w-full divide-y divide-gray-200 ", //dark:divide-neutral-700",
                                            thead { class: "bg-gray-50 ", //dark:bg-neutral-800",
                                                tr {
                                                    th {
                                                        scope: "col",
                                                        class: "px-6 py-3 text-start",
                                                        div { class: "flex items-center gap-x-2",
                                                            span { class: "text-xs font-semibold uppercase tracking-wide text-gray-800 ", //dark:text-neutral-200",
                                                                "\n                              Name\n                            "
                                                            }
                                                        }
                                                    }
                                                    th {
                                                        scope: "col",
                                                        class: "px-6 py-3 text-start",
                                                        div { class: "flex items-center gap-x-2",
                                                            span { class: "text-xs font-semibold uppercase tracking-wide text-gray-800 ", //dark:text-neutral-200",
                                                                "\n                              Status\n                            "
                                                            }
                                                        }
                                                    }
                                                    th {
                                                        scope: "col",
                                                        class: "px-6 py-3 text-start",
                                                        div { class: "flex items-center gap-x-2",
                                                            span { class: "text-xs font-semibold uppercase tracking-wide text-gray-800 ", //dark:text-neutral-200",
                                                                "\n                              Portfolio\n                            "
                                                            }
                                                        }
                                                    }
                                                    th {
                                                        scope: "col",
                                                        class: "px-6 py-3 text-start",
                                                        div { class: "flex items-center gap-x-2",
                                                            span { class: "text-xs font-semibold uppercase tracking-wide text-gray-800 ", //dark:text-neutral-200",
                                                                "\n                              Created\n                            "
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            tbody { class: "divide-y divide-gray-200 ", //dark:divide-neutral-700",
                                                tr {
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            div { class: "flex items-center gap-x-3",
                                                                img {
                                                                    alt: "Image Description",
                                                                    src: "https://images.unsplash.com/photo-1531927557220-a9e23c1e4794?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=facearea&facepad=2&w=300&h=300&q=80",
                                                                    class: "inline-block size-[38px] rounded-full"
                                                                }
                                                                div { class: "grow",
                                                                    span { class: "block text-sm font-semibold text-gray-800 ", //dark:text-neutral-200",
                                                                        "Christina Bersh"
                                                                    }
                                                                    span { class: "block text-sm text-gray-500 ", //dark:text-neutral-500",
                                                                        "christina@site.com"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            span { class: "inline-flex items-center gap-1.5 py-0.5 px-2 rounded-full text-xs font-medium bg-green-100 text-green-800 ", //dark:bg-green-900 ", //dark:text-green-200",
                                                                svg {
                                                                    "fill": "currentColor",
                                                                    "viewBox": "0 0 16 16",
                                                                    width: "16",
                                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                                    height: "16",
                                                                    class: "size-2.5",
                                                                    path { "d": "M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z" }
                                                                }
                                                                "\n                              Active\n                            "
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            div { class: "flex items-center gap-x-3",
                                                                span { class: "text-xs text-gray-500 ", //dark:text-neutral-500",
                                                                    "1/5"
                                                                }
                                                                div {
                                                                    "aria-valuenow": "25",
                                                                    "aria-valuemax": "100",
                                                                    "aria-valuemin": "0",
                                                                    role: "progressbar",
                                                                    class: "flex w-full h-1.5 bg-gray-200 rounded-full overflow-hidden ", //dark:bg-neutral-700",
                                                                    div {
                                                                        style: "width: 25%",
                                                                        class: "flex flex-col justify-center overflow-hidden bg-gray-800 text-xs text-white text-center whitespace-nowrap ", //dark:bg-neutral-200"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            span { class: "text-sm text-gray-500 ", //dark:text-neutral-500",
                                                                "28 Dec, 12:12"
                                                            }
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            div { class: "flex items-center gap-x-3",
                                                                img {
                                                                    alt: "Image Description",
                                                                    src: "https://images.unsplash.com/photo-1568602471122-7832951cc4c5?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=facearea&facepad=2&w=300&h=300&q=80",
                                                                    class: "inline-block size-[38px] rounded-full"
                                                                }
                                                                div { class: "grow",
                                                                    span { class: "block text-sm font-semibold text-gray-800 ", //dark:text-neutral-200",
                                                                        "David Harrison"
                                                                    }
                                                                    span { class: "block text-sm text-gray-500 ", //dark:text-neutral-500",
                                                                        "david@site.com"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            span { class: "inline-flex items-center gap-1.5 py-0.5 px-2 rounded-full text-xs font-medium bg-orange-100 text-orange-800 ", //dark:bg-orange-900 ", //dark:text-orange-200",
                                                                svg {
                                                                    "fill": "currentColor",
                                                                    "viewBox": "0 0 16 16",
                                                                    height: "16",
                                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                                    width: "16",
                                                                    class: "size-2.5",
                                                                    path { "d": "M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z" }
                                                                }
                                                                "\n                              Warning\n                            "
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            div { class: "flex items-center gap-x-3",
                                                                span { class: "text-xs text-gray-500 ", //dark:text-neutral-500",
                                                                    "3/5"
                                                                }
                                                                div {
                                                                    role: "progressbar",
                                                                    "aria-valuenow": "78",
                                                                    "aria-valuemax": "100",
                                                                    "aria-valuemin": "0",
                                                                    class: "flex w-full h-1.5 bg-gray-200 rounded-full overflow-hidden ", //dark:bg-neutral-700",
                                                                    div {
                                                                        style: "width: 78%",
                                                                        class: "flex flex-col justify-center overflow-hidden bg-gray-800 text-xs text-white text-center whitespace-nowrap ", //dark:bg-neutral-200"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            span { class: "text-sm text-gray-500 ", //dark:text-neutral-500",
                                                                "20 Dec, 09:27"
                                                            }
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            div { class: "flex items-center gap-x-3",
                                                                span { class: "inline-flex items-center justify-center size-[38px] rounded-full bg-gray-300 ", //dark:bg-neutral-700",
                                                                    span { class: "font-medium text-gray-800 leading-none ", //dark:text-neutral-200",
                                                                        "A"
                                                                    }
                                                                }
                                                                div { class: "grow",
                                                                    span { class: "block text-sm font-semibold text-gray-800 ", //dark:text-neutral-200",
                                                                        "Anne Richard"
                                                                    }
                                                                    span { class: "block text-sm text-gray-500 ", //dark:text-neutral-500",
                                                                        "anne@site.com"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            span { class: "inline-flex items-center gap-1.5 py-0.5 px-2 rounded-full text-xs font-medium bg-green-100 text-green-800 ", //dark:bg-green-900 ", //dark:text-green-200",
                                                                svg {
                                                                    width: "16",
                                                                    "viewBox": "0 0 16 16",
                                                                    "fill": "currentColor",
                                                                    height: "16",
                                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                                    class: "size-2.5",
                                                                    path { "d": "M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z" }
                                                                }
                                                                "\n                              Active\n                            "
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            div { class: "flex items-center gap-x-3",
                                                                span { class: "text-xs text-gray-500 ", //dark:text-neutral-500",
                                                                    "5/5"
                                                                }
                                                                div {
                                                                    "aria-valuemin": "0",
                                                                    role: "progressbar",
                                                                    "aria-valuemax": "100",
                                                                    "aria-valuenow": "100",
                                                                    class: "flex w-full h-1.5 bg-gray-200 rounded-full overflow-hidden ", //dark:bg-neutral-700",
                                                                    div {
                                                                        style: "width: 100%",
                                                                        class: "flex flex-col justify-center overflow-hidden bg-gray-800 text-xs text-white text-center whitespace-nowrap ", //dark:bg-neutral-200"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            span { class: "text-sm text-gray-500 ", //dark:text-neutral-500",
                                                                "18 Dec, 15:20"
                                                            }
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            div { class: "flex items-center gap-x-3",
                                                                img {
                                                                    alt: "Image Description",
                                                                    src: "https://images.unsplash.com/photo-1541101767792-f9b2b1c4f127?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&&auto=format&fit=facearea&facepad=3&w=300&h=300&q=80",
                                                                    class: "inline-block size-[38px] rounded-full"
                                                                }
                                                                div { class: "grow",
                                                                    span { class: "block text-sm font-semibold text-gray-800 ", //dark:text-neutral-200",
                                                                        "Samia Kartoon"
                                                                    }
                                                                    span { class: "block text-sm text-gray-500 ", //dark:text-neutral-500",
                                                                        "samia@site.com"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            span { class: "inline-flex items-center gap-1.5 py-0.5 px-2 rounded-full text-xs font-medium bg-green-100 text-green-800 ", //dark:bg-green-900 ", //dark:text-green-200",
                                                                svg {
                                                                    "fill": "currentColor",
                                                                    height: "16",
                                                                    "viewBox": "0 0 16 16",
                                                                    width: "16",
                                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                                    class: "size-2.5",
                                                                    path { "d": "M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z" }
                                                                }
                                                                "\n                              Active\n                            "
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            div { class: "flex items-center gap-x-3",
                                                                span { class: "text-xs text-gray-500 ", //dark:text-neutral-500",
                                                                    "0/5"
                                                                }
                                                                div {
                                                                    role: "progressbar",
                                                                    "aria-valuenow": "1",
                                                                    "aria-valuemax": "100",
                                                                    "aria-valuemin": "0",
                                                                    class: "flex w-full h-1.5 bg-gray-200 rounded-full overflow-hidden ", //dark:bg-neutral-700",
                                                                    div {
                                                                        style: "width: 1%",
                                                                        class: "flex flex-col justify-center overflow-hidden bg-gray-800 text-xs text-white text-center whitespace-nowrap ", //dark:bg-neutral-200"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    td { class: "size-px whitespace-nowrap",
                                                        div { class: "px-6 py-3",
                                                            span { class: "text-sm text-gray-500 ", //dark:text-neutral-500",
                                                                "18 Dec, 15:20"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div {
                                div { class: "sm:flex sm:justify-between",
                                    div {
                                        div { class: "inline-flex border border-gray-200 rounded-full p-0.5 ", //dark:border-neutral-700",
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    height: "24",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-width": "2",
                                                    "stroke-linecap": "round",
                                                    "stroke": "currentColor",
                                                    width: "24",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "stroke-linejoin": "round",
                                                    "fill": "none",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M7 10v12" }
                                                    path { "d": "M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z" }
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "fill": "none",
                                                    width: "24",
                                                    "stroke-width": "2",
                                                    "stroke-linejoin": "round",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke": "currentColor",
                                                    height: "24",
                                                    "stroke-linecap": "round",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M17 14V2" }
                                                    path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                                }
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "stroke-linecap": "round",
                                                "fill": "none",
                                                "stroke-linejoin": "round",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "viewBox": "0 0 24 24",
                                                width: "24",
                                                "stroke-width": "2",
                                                "stroke": "currentColor",
                                                height: "24",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M17 14V2" }
                                                path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                            }
                                            "\n                    Copy\n                  "
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "stroke-linejoin": "round",
                                                "viewBox": "0 0 24 24",
                                                "fill": "none",
                                                "stroke": "currentColor",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                height: "24",
                                                "stroke-linecap": "round",
                                                width: "24",
                                                "stroke-width": "2",
                                                class: "flex-shrink-0 size-4",
                                                circle {
                                                    "cx": "18",
                                                    "cy": "5",
                                                    "r": "3"
                                                }
                                                circle {
                                                    "r": "3",
                                                    "cx": "6",
                                                    "cy": "12"
                                                }
                                                circle {
                                                    "cy": "19",
                                                    "cx": "18",
                                                    "r": "3"
                                                }
                                                line {
                                                    "y2": "17.49",
                                                    "x2": "15.42",
                                                    "y1": "13.51",
                                                    "x1": "8.59"
                                                }
                                                line {
                                                    "x1": "15.41",
                                                    "y1": "6.51",
                                                    "x2": "8.59",
                                                    "y2": "10.49"
                                                }
                                            }
                                            "\n                    Share\n                  "
                                        }
                                    }
                                    div { class: "mt-1 sm:mt-0",
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "fill": "none",
                                                "stroke-width": "2",
                                                "stroke-linejoin": "round",
                                                "viewBox": "0 0 24 24",
                                                "stroke-linecap": "round",
                                                height: "24",
                                                "stroke": "currentColor",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                width: "24",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" }
                                                path { "d": "M21 3v5h-5" }
                                            }
                                            "\n                    New answer\n                  "
                                        }
                                    }
                                }
                            }
                        }
                    }
                    li { class: "py-2 sm:py-4",
                        div { class: "max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto",
                            div { class: "max-w-2xl flex gap-x-2 sm:gap-x-4",
                                span { class: "flex-shrink-0 inline-flex items-center justify-center size-[38px] rounded-full bg-gray-600",
                                    span { class: "text-sm font-medium text-white leading-none",
                                        "AZ"
                                    }
                                }
                                div { class: "grow mt-2 space-y-3",
                                    p { class: "text-gray-800 ", //dark:text-neutral-200",
                                        "\n                  show me the code\n                "
                                    }
                                }
                            }
                        }
                    }
                    li { class: "max-w-4xl py-2 px-4 sm:px-6 lg:px-8 mx-auto flex gap-x-2 sm:gap-x-4",
                        svg {
                            width: "38",
                            height: "38",
                            "fill": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 38 38",
                            class: "flex-shrink-0 w-[2.375rem] h-[2.375rem] rounded-full",
                            rect {
                                "rx": "6",
                                height: "38",
                                width: "38",
                                "fill": "#2563EB"
                            }
                            path {
                                "d": "M10 28V18.64C10 13.8683 14.0294 10 19 10C23.9706 10 28 13.8683 28 18.64C28 23.4117 23.9706 27.28 19 27.28H18.25",
                                "stroke": "white",
                                "stroke-width": "1.5"
                            }
                            path {
                                "stroke-width": "1.5",
                                "d": "M13 28V18.7552C13 15.5104 15.6863 12.88 19 12.88C22.3137 12.88 25 15.5104 25 18.7552C25 22 22.3137 24.6304 19 24.6304H18.25",
                                "stroke": "white"
                            }
                            ellipse {
                                "cx": "19",
                                "cy": "18.6554",
                                "rx": "3.75",
                                "ry": "3.6",
                                "fill": "white"
                            }
                        }
                        div { class: "grow max-w-[90%] md:max-w-2xl w-full space-y-3",
                            div { class: "space-y-3",
                                p { class: "text-sm text-gray-800 ", //dark:text-white",
                                    "\n                Of course!\n              "
                                }
                            }
                            div { class: "mt-3 flex-none min-w-full bg-gray-800 font-mono text-sm p-5 rounded-lg ", //dark:bg-neutral-800",
                                span { class: "text-red-500", "<" }
                                span { class: "text-red-500",
                                    "table "
                                    span { class: "text-gray-300",
                                        "class=\""
                                        span { class: "text-sky-400",
                                            "min-w-full divide-y divide-gray-200", //dark:divide-neutral-700"
                                        }
                                        "\""
                                    }
                                    span { class: "text-red-500", ">" }
                                    span { class: "block" }
                                    span { class: "ms-5 text-red-500", "<" }
                                    span { class: "text-red-500",
                                        "thead "
                                        span { class: "text-gray-300",
                                            "class=\""
                                            span { class: "text-sky-400", "bg-gray-50" } //dark:bg-neutral-800" }
                                            "\""
                                        }
                                        span { class: "text-red-500", ">" }
                                        span { class: "block" }
                                        span { class: "text-red-500",
                                            span { class: "ms-10 text-gray-500", //dark:text-neutral-500",
                                                "..."
                                            }
                                        }
                                    }
                                }
                            }
                            div {
                                div { class: "sm:flex sm:justify-between",
                                    div {
                                        div { class: "inline-flex border border-gray-200 rounded-full p-0.5 ", //dark:border-neutral-700",
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    "stroke-linecap": "round",
                                                    "fill": "none",
                                                    width: "24",
                                                    "stroke": "currentColor",
                                                    "stroke-linejoin": "round",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-width": "2",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    height: "24",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M7 10v12" }
                                                    path { "d": "M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z" }
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    height: "24",
                                                    "stroke": "currentColor",
                                                    "fill": "none",
                                                    "stroke-width": "2",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "stroke-linecap": "round",
                                                    width: "24",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-linejoin": "round",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M17 14V2" }
                                                    path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                                }
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "fill": "none",
                                                "stroke-linejoin": "round",
                                                "stroke": "currentColor",
                                                width: "24",
                                                height: "24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "viewBox": "0 0 24 24",
                                                "stroke-width": "2",
                                                "stroke-linecap": "round",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M17 14V2" }
                                                path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                            }
                                            "\n                    Copy\n                  "
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "stroke-linejoin": "round",
                                                width: "24",
                                                "stroke-width": "2",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                height: "24",
                                                "fill": "none",
                                                "stroke": "currentColor",
                                                "viewBox": "0 0 24 24",
                                                "stroke-linecap": "round",
                                                class: "flex-shrink-0 size-4",
                                                circle {
                                                    "cx": "18",
                                                    "r": "3",
                                                    "cy": "5"
                                                }
                                                circle {
                                                    "cx": "6",
                                                    "cy": "12",
                                                    "r": "3"
                                                }
                                                circle {
                                                    "cy": "19",
                                                    "r": "3",
                                                    "cx": "18"
                                                }
                                                line {
                                                    "y1": "13.51",
                                                    "x2": "15.42",
                                                    "x1": "8.59",
                                                    "y2": "17.49"
                                                }
                                                line {
                                                    "y1": "6.51",
                                                    "x2": "8.59",
                                                    "y2": "10.49",
                                                    "x1": "15.41"
                                                }
                                            }
                                            "\n                    Share\n                  "
                                        }
                                    }
                                    div { class: "mt-1 sm:mt-0",
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "stroke-width": "2",
                                                "fill": "none",
                                                "stroke-linecap": "round",
                                                height: "24",
                                                width: "24",
                                                "stroke": "currentColor",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke-linejoin": "round",
                                                "viewBox": "0 0 24 24",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" }
                                                path { "d": "M21 3v5h-5" }
                                            }
                                            "\n                    New answer\n                  "
                                        }
                                    }
                                }
                            }
                        }
                    }
                    li { class: "py-2 sm:py-4",
                        div { class: "max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto",
                            div { class: "max-w-2xl flex gap-x-2 sm:gap-x-4",
                                span { class: "flex-shrink-0 inline-flex items-center justify-center size-[38px] rounded-full bg-gray-600",
                                    span { class: "text-sm font-medium text-white leading-none",
                                        "AZ"
                                    }
                                }
                                div { class: "grow mt-2 space-y-3",
                                    p { class: "text-gray-800 ", //dark:text-neutral-200",
                                        "\n                  quiz me about tailwindcss\n                "
                                    }
                                }
                            }
                        }
                    }
                    li { class: "max-w-4xl py-2 px-4 sm:px-6 lg:px-8 mx-auto flex gap-x-2 sm:gap-x-4",
                        svg {
                            "fill": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 38 38",
                            width: "38",
                            height: "38",
                            class: "flex-shrink-0 w-[2.375rem] h-[2.375rem] rounded-full",
                            rect {
                                height: "38",
                                "fill": "#2563EB",
                                "rx": "6",
                                width: "38"
                            }
                            path {
                                "d": "M10 28V18.64C10 13.8683 14.0294 10 19 10C23.9706 10 28 13.8683 28 18.64C28 23.4117 23.9706 27.28 19 27.28H18.25",
                                "stroke": "white",
                                "stroke-width": "1.5"
                            }
                            path {
                                "stroke": "white",
                                "d": "M13 28V18.7552C13 15.5104 15.6863 12.88 19 12.88C22.3137 12.88 25 15.5104 25 18.7552C25 22 22.3137 24.6304 19 24.6304H18.25",
                                "stroke-width": "1.5"
                            }
                            ellipse {
                                "cx": "19",
                                "ry": "3.6",
                                "rx": "3.75",
                                "fill": "white",
                                "cy": "18.6554"
                            }
                        }
                        div { class: "grow max-w-[90%] md:max-w-2xl w-full space-y-3",
                            div { class: "space-y-3",
                                p { class: "text-sm text-gray-800 ", //dark:text-white",
                                    "\n                Sure!\n              "
                                }
                            }
                            div {
                                button {
                                    r#type: "button",
                                    class: "mb-2.5 me-1.5 py-2 px-3 inline-flex justify-center items-center gap-x-2 rounded-lg border border-blue-600 bg-white text-blue-600 align-middle hover:bg-blue-50 text-sm ", //dark:bg-neutral-900 ", //dark:text-blue-500 ", //dark:border-blue-500 ", //dark:hover:text-blue-400 ", //dark:hover:border-blue-400",
                                    "\n                Is Tailwind CSS a free library?\n              "
                                }
                                button {
                                    r#type: "button",
                                    class: "mb-2.5 me-1.5 py-2 px-3 inline-flex justify-center items-center gap-x-2 rounded-lg border border-blue-600 bg-white text-blue-600 align-middle hover:bg-blue-50 text-sm ", //dark:bg-neutral-900 ", //dark:text-blue-500 ", //dark:border-blue-500 ", //dark:hover:text-blue-400 ", //dark:hover:border-blue-400",
                                    "\n                What's the latest Tailwind CSS version?\n              "
                                }
                                button {
                                    r#type: "button",
                                    class: "mb-2.5 me-1.5 py-2 px-3 inline-flex justify-center items-center gap-x-2 rounded-lg border border-blue-600 bg-white text-blue-600 align-middle hover:bg-blue-50 text-sm ", //dark:bg-neutral-900 ", //dark:text-blue-500 ", //dark:border-blue-500 ", //dark:hover:text-blue-400 ", //dark:hover:border-blue-400",
                                    "\n                Is it a utility-class based?\n              "
                                }
                            }
                            div {
                                div { class: "sm:flex sm:justify-between",
                                    div {
                                        div { class: "inline-flex border border-gray-200 rounded-full p-0.5 ", //dark:border-neutral-700",
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    width: "24",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "stroke": "currentColor",
                                                    "fill": "none",
                                                    "stroke-linejoin": "round",
                                                    "stroke-linecap": "round",
                                                    "stroke-width": "2",
                                                    "viewBox": "0 0 24 24",
                                                    height: "24",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M7 10v12" }
                                                    path { "d": "M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z" }
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    "stroke-width": "2",
                                                    "fill": "none",
                                                    height: "24",
                                                    "stroke": "currentColor",
                                                    "stroke-linecap": "round",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "stroke-linejoin": "round",
                                                    "viewBox": "0 0 24 24",
                                                    width: "24",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M17 14V2" }
                                                    path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                                }
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "fill": "none",
                                                height: "24",
                                                width: "24",
                                                "viewBox": "0 0 24 24",
                                                "stroke-linecap": "round",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke": "currentColor",
                                                "stroke-width": "2",
                                                "stroke-linejoin": "round",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M17 14V2" }
                                                path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                            }
                                            "\n                    Copy\n                  "
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                height: "24",
                                                width: "24",
                                                "stroke-width": "2",
                                                "stroke": "currentColor",
                                                "fill": "none",
                                                "viewBox": "0 0 24 24",
                                                "stroke-linecap": "round",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke-linejoin": "round",
                                                class: "flex-shrink-0 size-4",
                                                circle {
                                                    "r": "3",
                                                    "cx": "18",
                                                    "cy": "5"
                                                }
                                                circle {
                                                    "cy": "12",
                                                    "r": "3",
                                                    "cx": "6"
                                                }
                                                circle {
                                                    "cx": "18",
                                                    "cy": "19",
                                                    "r": "3"
                                                }
                                                line {
                                                    "y1": "13.51",
                                                    "y2": "17.49",
                                                    "x2": "15.42",
                                                    "x1": "8.59"
                                                }
                                                line {
                                                    "x1": "15.41",
                                                    "x2": "8.59",
                                                    "y2": "10.49",
                                                    "y1": "6.51"
                                                }
                                            }
                                            "\n                    Share\n                  "
                                        }
                                    }
                                    div { class: "mt-1 sm:mt-0",
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                width: "24",
                                                "stroke-width": "2",
                                                "stroke-linejoin": "round",
                                                "stroke-linecap": "round",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                height: "24",
                                                "fill": "none",
                                                "viewBox": "0 0 24 24",
                                                "stroke": "currentColor",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" }
                                                path { "d": "M21 3v5h-5" }
                                            }
                                            "\n                    New answer\n                  "
                                        }
                                    }
                                }
                            }
                        }
                    }
                    li { class: "py-2 sm:py-4",
                        div { class: "max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto",
                            div { class: "max-w-2xl flex gap-x-2 sm:gap-x-4",
                                span { class: "flex-shrink-0 inline-flex items-center justify-center size-[38px] rounded-full bg-gray-600",
                                    span { class: "text-sm font-medium text-white leading-none",
                                        "AZ"
                                    }
                                }
                                div { class: "grow mt-2 space-y-3",
                                    p { class: "text-gray-800 ", //dark:text-neutral-200",
                                        "\n                  generate 3-dimensional abstract images\n                "
                                    }
                                }
                            }
                        }
                    }
                    li { class: "max-w-4xl py-2 px-4 sm:px-6 lg:px-8 mx-auto flex gap-x-2 sm:gap-x-4",
                        svg {
                            width: "38",
                            height: "38",
                            "viewBox": "0 0 38 38",
                            "fill": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            class: "flex-shrink-0 w-[2.375rem] h-[2.375rem] rounded-full",
                            rect {
                                "rx": "6",
                                width: "38",
                                height: "38",
                                "fill": "#2563EB"
                            }
                            path {
                                "stroke-width": "1.5",
                                "d": "M10 28V18.64C10 13.8683 14.0294 10 19 10C23.9706 10 28 13.8683 28 18.64C28 23.4117 23.9706 27.28 19 27.28H18.25",
                                "stroke": "white"
                            }
                            path {
                                "stroke-width": "1.5",
                                "stroke": "white",
                                "d": "M13 28V18.7552C13 15.5104 15.6863 12.88 19 12.88C22.3137 12.88 25 15.5104 25 18.7552C25 22 22.3137 24.6304 19 24.6304H18.25"
                            }
                            ellipse {
                                "fill": "white",
                                "cx": "19",
                                "cy": "18.6554",
                                "ry": "3.6",
                                "rx": "3.75"
                            }
                        }
                        div { class: "grow max-w-[90%] md:max-w-2xl w-full space-y-3",
                            div { class: "space-y-3",
                                p { class: "text-sm text-gray-800 ", //dark:text-white",
                                    "\n                Here you go...\n              "
                                }
                            }
                            div { class: "grid grid-cols-2 gap-1 rounded-lg overflow-hidden",
                                div { class: "aspect-w-16 aspect-h-9",
                                    img {
                                        src: "https://images.unsplash.com/photo-1677644334825-0eb411012ac0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=3343&q=80",
                                        alt: "Deep Learning",
                                        class: "w-full object-cover"
                                    }
                                }
                                div { class: "aspect-w-16 aspect-h-9",
                                    img {
                                        alt: "Deep Learning",
                                        src: "https://images.unsplash.com/photo-1680868543815-b8666dba60f7?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2532&q=80",
                                        class: "w-full object-cover"
                                    }
                                }
                                div { class: "aspect-w-16 aspect-h-9",
                                    img {
                                        src: "https://images.unsplash.com/photo-1680193895115-b51b4ed5392f?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1035&q=80",
                                        alt: "Deep Learning",
                                        class: "w-full object-cover"
                                    }
                                }
                                div { class: "aspect-w-16 aspect-h-9",
                                    img {
                                        alt: "Deep Learning",
                                        src: "https://images.unsplash.com/photo-1680587590161-3a1dd77a7609?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2532&q=80",
                                        class: "w-full object-cover"
                                    }
                                }
                            }
                            div {
                                div { class: "sm:flex sm:justify-between",
                                    div {
                                        div { class: "inline-flex border border-gray-200 rounded-full p-0.5 ", //dark:border-neutral-700",
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    width: "24",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "fill": "none",
                                                    "stroke-width": "2",
                                                    "stroke-linejoin": "round",
                                                    "stroke-linecap": "round",
                                                    "stroke": "currentColor",
                                                    height: "24",
                                                    "viewBox": "0 0 24 24",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M7 10v12" }
                                                    path { "d": "M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z" }
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    "stroke-width": "2",
                                                    "fill": "none",
                                                    "stroke": "currentColor",
                                                    "viewBox": "0 0 24 24",
                                                    "stroke-linecap": "round",
                                                    height: "24",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    width: "24",
                                                    "stroke-linejoin": "round",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M17 14V2" }
                                                    path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                                }
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "fill": "none",
                                                height: "24",
                                                "stroke": "currentColor",
                                                "viewBox": "0 0 24 24",
                                                "stroke-linejoin": "round",
                                                "stroke-width": "2",
                                                width: "24",
                                                "stroke-linecap": "round",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M17 14V2" }
                                                path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                            }
                                            "\n                    Copy\n                  "
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "viewBox": "0 0 24 24",
                                                "fill": "none",
                                                "stroke-width": "2",
                                                "stroke-linecap": "round",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                width: "24",
                                                height: "24",
                                                "stroke": "currentColor",
                                                "stroke-linejoin": "round",
                                                class: "flex-shrink-0 size-4",
                                                circle {
                                                    "r": "3",
                                                    "cx": "18",
                                                    "cy": "5"
                                                }
                                                circle {
                                                    "r": "3",
                                                    "cx": "6",
                                                    "cy": "12"
                                                }
                                                circle {
                                                    "cx": "18",
                                                    "r": "3",
                                                    "cy": "19"
                                                }
                                                line {
                                                    "y1": "13.51",
                                                    "y2": "17.49",
                                                    "x1": "8.59",
                                                    "x2": "15.42"
                                                }
                                                line {
                                                    "y1": "6.51",
                                                    "x1": "15.41",
                                                    "y2": "10.49",
                                                    "x2": "8.59"
                                                }
                                            }
                                            "\n                    Share\n                  "
                                        }
                                    }
                                    div { class: "mt-1 sm:mt-0",
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke-linecap": "round",
                                                "viewBox": "0 0 24 24",
                                                "stroke-linejoin": "round",
                                                "stroke": "currentColor",
                                                "fill": "none",
                                                "stroke-width": "2",
                                                height: "24",
                                                width: "24",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" }
                                                path { "d": "M21 3v5h-5" }
                                            }
                                            "\n                    New answer\n                  "
                                        }
                                    }
                                }
                            }
                        }
                    }
                    li { class: "py-2 sm:py-4",
                        div { class: "max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto",
                            div { class: "max-w-2xl flex gap-x-2 sm:gap-x-4",
                                span { class: "flex-shrink-0 inline-flex items-center justify-center size-[38px] rounded-full bg-gray-600",
                                    span { class: "text-sm font-medium text-white leading-none",
                                        "AZ"
                                    }
                                }
                                div { class: "grow mt-2 space-y-3",
                                    p { class: "text-gray-800 ", //dark:text-neutral-200",
                                        "\n                  what's tailwindcss?\n                "
                                    }
                                }
                            }
                        }
                    }
                    li { class: "max-w-4xl py-2 px-4 sm:px-6 lg:px-8 mx-auto flex gap-x-2 sm:gap-x-4",
                        svg {
                            width: "38",
                            "fill": "none",
                            "viewBox": "0 0 38 38",
                            height: "38",
                            "xmlns": "http://www.w3.org/2000/svg",
                            class: "flex-shrink-0 w-[2.375rem] h-[2.375rem] rounded-full",
                            rect {
                                height: "38",
                                "fill": "#2563EB",
                                "rx": "6",
                                width: "38"
                            }
                            path {
                                "stroke": "white",
                                "d": "M10 28V18.64C10 13.8683 14.0294 10 19 10C23.9706 10 28 13.8683 28 18.64C28 23.4117 23.9706 27.28 19 27.28H18.25",
                                "stroke-width": "1.5"
                            }
                            path {
                                "d": "M13 28V18.7552C13 15.5104 15.6863 12.88 19 12.88C22.3137 12.88 25 15.5104 25 18.7552C25 22 22.3137 24.6304 19 24.6304H18.25",
                                "stroke-width": "1.5",
                                "stroke": "white"
                            }
                            ellipse {
                                "fill": "white",
                                "rx": "3.75",
                                "cy": "18.6554",
                                "cx": "19",
                                "ry": "3.6"
                            }
                        }
                        div { class: "grow max-w-[90%] md:max-w-2xl w-full space-y-3",
                            div { class: "space-y-3",
                                p { class: "text-sm text-gray-800 ", //dark:text-white",
                                    "\n                Tailwind CSS is an open source CSS framework. The main feature of this library is that, unlike other CSS frameworks like Bootstrap, it does not provide a series of predefined classes for elements such as buttons or tables.\n              "
                                }
                                div { class: "space-y-1.5",
                                    ul {
                                        li {
                                            a {
                                                href: "#",
                                                class: "text-sm text-blue-600 decoration-2 hover:underline font-medium ", //dark:text-blue-500 ", //dark:hover:text-blue-400",
                                                "\n                      Get started with Tailwind CSS\n                    "
                                            }
                                        }
                                        li {
                                            a {
                                                href: "#",
                                                class: "text-sm text-blue-600 decoration-2 hover:underline font-medium ", //dark:text-blue-500 ", //dark:hover:text-blue-400",
                                                "\n                      Tailwind CSS Installation guide\n                    "
                                            }
                                        }
                                    }
                                }
                            }
                            div {
                                div { class: "sm:flex sm:justify-between",
                                    div {
                                        div { class: "inline-flex border border-gray-200 rounded-full p-0.5 ", //dark:border-neutral-700",
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "stroke-width": "2",
                                                    height: "24",
                                                    width: "24",
                                                    "stroke": "currentColor",
                                                    "stroke-linecap": "round",
                                                    "stroke-linejoin": "round",
                                                    "fill": "none",
                                                    "viewBox": "0 0 24 24",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M7 10v12" }
                                                    path { "d": "M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2h0a3.13 3.13 0 0 1 3 3.88Z" }
                                                }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-full text-gray-500 hover:bg-blue-100 hover:text-blue-800 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:bg-blue-900 ", //dark:hover:text-blue-200",
                                                svg {
                                                    "fill": "none",
                                                    "viewBox": "0 0 24 24",
                                                    "xmlns": "http://www.w3.org/2000/svg",
                                                    "stroke-width": "2",
                                                    width: "24",
                                                    "stroke-linejoin": "round",
                                                    "stroke-linecap": "round",
                                                    height: "24",
                                                    "stroke": "currentColor",
                                                    class: "flex-shrink-0 size-4",
                                                    path { "d": "M17 14V2" }
                                                    path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                                }
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "stroke-linecap": "round",
                                                "stroke-width": "2",
                                                width: "24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke": "currentColor",
                                                "stroke-linejoin": "round",
                                                height: "24",
                                                "fill": "none",
                                                "viewBox": "0 0 24 24",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M17 14V2" }
                                                path { "d": "M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22h0a3.13 3.13 0 0 1-3-3.88Z" }
                                            }
                                            "\n                    Copy\n                  "
                                        }
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "stroke": "currentColor",
                                                "stroke-linejoin": "round",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                width: "24",
                                                "viewBox": "0 0 24 24",
                                                height: "24",
                                                "fill": "none",
                                                "stroke-width": "2",
                                                "stroke-linecap": "round",
                                                class: "flex-shrink-0 size-4",
                                                circle {
                                                    "cx": "18",
                                                    "cy": "5",
                                                    "r": "3"
                                                }
                                                circle {
                                                    "cx": "6",
                                                    "r": "3",
                                                    "cy": "12"
                                                }
                                                circle {
                                                    "r": "3",
                                                    "cx": "18",
                                                    "cy": "19"
                                                }
                                                line {
                                                    "x2": "15.42",
                                                    "y1": "13.51",
                                                    "x1": "8.59",
                                                    "y2": "17.49"
                                                }
                                                line {
                                                    "y1": "6.51",
                                                    "y2": "10.49",
                                                    "x2": "8.59",
                                                    "x1": "15.41"
                                                }
                                            }
                                            "\n                    Share\n                  "
                                        }
                                    }
                                    div { class: "mt-1 sm:mt-0",
                                        button {
                                            r#type: "button",
                                            class: "py-2 px-3 inline-flex items-center gap-x-2 text-sm rounded-full border border-transparent text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:text-neutral-400 ", //dark:hover:bg-neutral-800",
                                            svg {
                                                "stroke-linejoin": "round",
                                                width: "24",
                                                "viewBox": "0 0 24 24",
                                                "xmlns": "http://www.w3.org/2000/svg",
                                                "stroke-linecap": "round",
                                                "stroke-width": "2",
                                                height: "24",
                                                "fill": "none",
                                                "stroke": "currentColor",
                                                class: "flex-shrink-0 size-4",
                                                path { "d": "M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" }
                                                path { "d": "M21 3v5h-5" }
                                            }
                                            "\n                    New answer\n                  "
                                        }
                                    }
                                }
                            }
                        }
                    }
                    li { class: "py-2 sm:py-4",
                        div { class: "max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto",
                            div { class: "max-w-2xl flex gap-x-2 sm:gap-x-4",
                                div {
                                    div { class: "text-end",
                                        button {
                                            r#type: "button",
                                            class: "ms-1.5 py-2 px-3 inline-flex justify-center items-center gap-x-2 rounded-lg border border-blue-600 bg-white text-blue-600 align-middle hover:bg-blue-50 text-sm ", //dark:bg-neutral-900 ", //dark:text-blue-500 ", //dark:border-blue-500 ", //dark:hover:text-blue-400 ", //dark:hover:border-blue-400",
                                            "\n                    What is the use of Tailwind CSS?\n                  "
                                        }
                                        button {
                                            r#type: "button",
                                            class: "ms-1.5 py-2 px-3 inline-flex justify-center items-center gap-x-2 rounded-lg border border-blue-600 bg-white text-blue-600 align-middle hover:bg-blue-50 text-sm ", //dark:bg-neutral-900 ", //dark:text-blue-500 ", //dark:border-blue-500 ", //dark:hover:text-blue-400 ", //dark:hover:border-blue-400",
                                            "\n                    What is the difference between Tailwind CSS and CSS?\n                  "
                                        }
                                    }
                                }
                            }
                        }
                    }
                    li { class: "py-2 sm:py-4",
                        div { class: "max-w-4xl px-4 sm:px-6 lg:px-8 mx-auto",
                            div { class: "max-w-2xl flex gap-x-2 sm:gap-x-4",
                                span { class: "flex-shrink-0 inline-flex items-center justify-center size-[38px] rounded-full bg-gray-600",
                                    span { class: "text-sm font-medium text-white leading-none",
                                        "AZ"
                                    }
                                }
                                div { class: "grow mt-2 space-y-3",
                                    p { class: "text-gray-800 ", //dark:text-neutral-200",
                                        "\n                  2 files uploaded\n                "
                                    }
                                    ul { class: "flex flex-col justify-end text-start -space-y-px",
                                        li { class: "flex items-center gap-x-2 p-3 text-sm bg-white border text-gray-800 first:rounded-t-lg first:mt-0 last:rounded-b-lg ", //dark:bg-neutral-900 ", //dark:border-neutral-700 ", //dark:text-neutral-200",
                                            div { class: "w-full flex justify-between truncate",
                                                span { class: "me-3 flex-1 w-0 truncate",
                                                    "\n                        resume_web_ui_developer.csv\n                      "
                                                }
                                                button {
                                                    r#type: "button",
                                                    class: "flex items-center gap-x-2 text-gray-500 hover:text-blue-600 whitespace-nowrap ", //dark:text-neutral-500 ", //dark:hover:text-blue-500",
                                                    svg {
                                                        "viewBox": "0 0 24 24",
                                                        height: "24",
                                                        "stroke-linejoin": "round",
                                                        "fill": "none",
                                                        "stroke-width": "2",
                                                        "stroke": "currentColor",
                                                        "stroke-linecap": "round",
                                                        "xmlns": "http://www.w3.org/2000/svg",
                                                        width: "24",
                                                        class: "flex-shrink-0 size-4",
                                                        path { "d": "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                                        polyline { "points": "7 10 12 15 17 10" }
                                                        line {
                                                            "y2": "3",
                                                            "x2": "12",
                                                            "x1": "12",
                                                            "y1": "15"
                                                        }
                                                    }
                                                    "\n                        Download\n                      "
                                                }
                                            }
                                        }
                                        li { class: "flex items-center gap-x-2 p-3 text-sm bg-white border text-gray-800 first:rounded-t-lg first:mt-0 last:rounded-b-lg ", //dark:bg-neutral-900 ", //dark:border-neutral-700 ", //dark:text-neutral-200",
                                            div { class: "w-full flex justify-between truncate",
                                                span { class: "me-3 flex-1 w-0 truncate",
                                                    "\n                        coverletter_web_ui_developer.pdf\n                      "
                                                }
                                                button {
                                                    r#type: "button",
                                                    class: "flex items-center gap-x-2 text-gray-500 hover:text-blue-600 whitespace-nowrap ", //dark:text-neutral-500 ", //dark:hover:text-blue-500",
                                                    svg {
                                                        "fill": "none",
                                                        width: "24",
                                                        height: "24",
                                                        "stroke": "currentColor",
                                                        "stroke-linejoin": "round",
                                                        "stroke-width": "2",
                                                        "viewBox": "0 0 24 24",
                                                        "stroke-linecap": "round",
                                                        "xmlns": "http://www.w3.org/2000/svg",
                                                        class: "flex-shrink-0 size-4",
                                                        path { "d": "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                                        polyline { "points": "7 10 12 15 17 10" }
                                                        line {
                                                            "x1": "12",
                                                            "y2": "3",
                                                            "x2": "12",
                                                            "y1": "15"
                                                        }
                                                    }
                                                    "\n                        Download\n                      "
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            footer { class: "max-w-4xl mx-auto sticky bottom-0 z-10 p-3 sm:py-6",
                div { class: "lg:hidden flex justify-end mb-2 sm:mb-3",
                    button {
                        r#type: "button",
                        "aria-controls": "application-sidebar",
                        "data-hs-overlay": "#application-sidebar",
                        "aria-label": "Toggle navigation",
                        class: "p-2 inline-flex items-center gap-x-2 text-xs font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none ", //dark:bg-neutral-900 ", //dark:border-neutral-700 ", //dark:text-white ", //dark:hover:bg-neutral-800",
                        svg {
                            width: "24",
                            "fill": "none",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 24 24",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            height: "24",
                            "stroke": "currentColor",
                            class: "flex-shrink-0 size-3.5",
                            line {
                                "x1": "3",
                                "x2": "21",
                                "y2": "6",
                                "y1": "6"
                            }
                            line {
                                "x1": "3",
                                "y2": "12",
                                "x2": "21",
                                "y1": "12"
                            }
                            line {
                                "x1": "3",
                                "y2": "18",
                                "x2": "21",
                                "y1": "18"
                            }
                        }
                        span { "Sidebar" }
                    }
                }
                div { class: "relative",
                    textarea {
                        placeholder: "Ask me anything...",
                        class: "p-4 pb-12 block w-full bg-gray-100 border-gray-200 rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 ", //dark:bg-neutral-800 ", //dark:border-neutral-700 ", //dark:text-neutral-400 ", //dark:placeholder-neutral-500 ", //dark:focus:ring-neutral-600"
                    }
                    div { class: "absolute bottom-px inset-x-px p-2 rounded-b-md bg-gray-100 ", //dark:bg-neutral-800",
                        div { class: "flex justify-between items-center",
                            div { class: "flex items-center",
                                button {
                                    r#type: "button",
                                    class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-lg text-gray-500 hover:text-blue-600 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:text-blue-500",
                                    svg {
                                        "stroke-width": "2",
                                        width: "24",
                                        "fill": "none",
                                        "viewBox": "0 0 24 24",
                                        "stroke-linecap": "round",
                                        height: "24",
                                        "xmlns": "http://www.w3.org/2000/svg",
                                        "stroke": "currentColor",
                                        "stroke-linejoin": "round",
                                        class: "flex-shrink-0 size-4",
                                        rect {
                                            "y": "3",
                                            "x": "3",
                                            height: "18",
                                            width: "18",
                                            "rx": "2"
                                        }
                                        line {
                                            "x2": "15",
                                            "x1": "9",
                                            "y2": "9",
                                            "y1": "15"
                                        }
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-lg text-gray-500 hover:text-blue-600 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:text-blue-500",
                                    svg {
                                        "fill": "none",
                                        height: "24",
                                        "xmlns": "http://www.w3.org/2000/svg",
                                        width: "24",
                                        "stroke": "currentColor",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "viewBox": "0 0 24 24",
                                        "stroke-width": "2",
                                        class: "flex-shrink-0 size-4",
                                        path { "d": "m21.44 11.05-9.19 9.19a6 6 0 0 1-8.49-8.49l8.57-8.57A4 4 0 1 1 18 8.84l-8.59 8.57a2 2 0 0 1-2.83-2.83l8.49-8.48" }
                                    }
                                }
                            }
                            div { class: "flex items-center gap-x-1",
                                button {
                                    r#type: "button",
                                    class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-lg text-gray-500 hover:text-blue-600 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500 ", //dark:text-neutral-500 ", //dark:hover:text-blue-500",
                                    svg {
                                        "stroke-width": "2",
                                        "stroke-linecap": "round",
                                        width: "24",
                                        height: "24",
                                        "stroke": "currentColor",
                                        "xmlns": "http://www.w3.org/2000/svg",
                                        "stroke-linejoin": "round",
                                        "viewBox": "0 0 24 24",
                                        "fill": "none",
                                        class: "flex-shrink-0 size-4",
                                        path { "d": "M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" }
                                        path { "d": "M19 10v2a7 7 0 0 1-14 0v-2" }
                                        line {
                                            "x1": "12",
                                            "y2": "22",
                                            "y1": "19",
                                            "x2": "12"
                                        }
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "inline-flex flex-shrink-0 justify-center items-center size-8 rounded-lg text-white bg-blue-600 hover:bg-blue-500 focus:z-10 focus:outline-none focus:ring-2 focus:ring-blue-500",
                                    svg {
                                        "fill": "currentColor",
                                        "viewBox": "0 0 16 16",
                                        height: "16",
                                        "xmlns": "http://www.w3.org/2000/svg",
                                        width: "16",
                                        class: "flex-shrink-0 size-3.5",
                                        path { "d": "M15.964.686a.5.5 0 0 0-.65-.65L.767 5.855H.766l-.452.18a.5.5 0 0 0-.082.887l.41.26.001.002 4.995 3.178 3.178 4.995.002.002.26.41a.5.5 0 0 0 .886-.083l6-15Zm-1.833 1.89L6.637 10.07l-.215-.338a.5.5 0 0 0-.154-.154l-.338-.215 7.494-7.494 1.178-.471-.47 1.178Z" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


mod icons {
    use super::*;

    #[component]
    pub(super) fn svaroh_icon() -> Element {
        rsx! {
            svg {
                "fill": "none",
                "xmlns": "http://www.w3.org/2000/svg",
                "viewBox": "0 0 116 32",
                width: "116",
                height: "32",
                class: "w-28 h-auto",
                path {
                    "fill": "currentColor",
                    "d": "M33.5696 30.2968V10.7968H37.4474V13.1789H37.6229C37.7952 12.7972 38.0445 12.4094 38.3707 12.0155C38.7031 11.6154 39.134 11.283 39.6634 11.0183C40.1989 10.7475 40.8636 10.6121 41.6577 10.6121C42.6918 10.6121 43.6458 10.8829 44.5199 11.4246C45.3939 11.9601 46.0926 12.7695 46.6158 13.8529C47.139 14.93 47.4006 16.2811 47.4006 17.9061C47.4006 19.488 47.1451 20.8237 46.6342 21.9132C46.1295 22.9966 45.4401 23.8183 44.5661 24.3784C43.6982 24.9324 42.7256 25.2094 41.6484 25.2094C40.8852 25.2094 40.2358 25.0832 39.7003 24.8308C39.1709 24.5785 38.737 24.2615 38.3984 23.8799C38.0599 23.4921 37.8014 23.1012 37.6229 22.7073H37.5028V30.2968H33.5696ZM37.4197 17.8877C37.4197 18.7309 37.5367 19.4665 37.7706 20.0943C38.0045 20.7222 38.343 21.2115 38.7862 21.5624C39.2294 21.9071 39.768 22.0794 40.402 22.0794C41.0421 22.0794 41.5838 21.904 42.027 21.5532C42.4702 21.1961 42.8056 20.7037 43.0334 20.0759C43.2673 19.4419 43.3842 18.7125 43.3842 17.8877C43.3842 17.069 43.2704 16.3488 43.0426 15.7272C42.8149 15.1055 42.4794 14.6192 42.0362 14.2683C41.593 13.9175 41.0483 13.7421 40.402 13.7421C39.7618 13.7421 39.2202 13.9113 38.777 14.2499C38.34 14.5884 38.0045 15.0685 37.7706 15.6902C37.5367 16.3119 37.4197 17.0444 37.4197 17.8877ZM49.2427 24.9786V10.7968H53.0559V13.2712H53.2037C53.4622 12.391 53.8961 11.7262 54.5055 11.2769C55.1149 10.8214 55.8166 10.5936 56.6106 10.5936C56.8076 10.5936 57.02 10.6059 57.2477 10.6306C57.4754 10.6552 57.6755 10.689 57.8478 10.7321V14.2222C57.6632 14.1668 57.4077 14.1175 57.0815 14.0745C56.7553 14.0314 56.4567 14.0098 56.1859 14.0098C55.6073 14.0098 55.0903 14.136 54.6348 14.3884C54.1854 14.6346 53.8284 14.9793 53.5638 15.4225C53.3052 15.8657 53.176 16.3765 53.176 16.9551V24.9786H49.2427ZM64.9043 25.2556C63.4455 25.2556 62.1898 24.9601 61.1373 24.3692C60.0909 23.7721 59.2845 22.9289 58.7182 21.8394C58.1519 20.7437 57.8688 19.448 57.8688 17.9523C57.8688 16.4935 58.1519 15.2132 58.7182 14.1114C59.2845 13.0096 60.0816 12.1509 61.1096 11.5354C62.1437 10.9199 63.3563 10.6121 64.7474 10.6121C65.683 10.6121 66.5539 10.7629 67.3603 11.0645C68.1728 11.36 68.8806 11.8062 69.4839 12.4033C70.0932 13.0004 70.5672 13.7513 70.9057 14.6561C71.2443 15.5548 71.4135 16.6074 71.4135 17.8138V18.8941H59.4384V16.4566H67.7111C67.7111 15.8903 67.588 15.3886 67.3418 14.9516C67.0956 14.5146 66.754 14.1729 66.317 13.9267C65.8861 13.6744 65.3844 13.5482 64.812 13.5482C64.2149 13.5482 63.6856 13.6867 63.2239 13.9637C62.7684 14.2345 62.4114 14.6007 62.1529 15.0624C61.8944 15.5179 61.762 16.0257 61.7559 16.5858V18.9033C61.7559 19.605 61.8851 20.2113 62.1437 20.7222C62.4083 21.2331 62.7807 21.627 63.2608 21.904C63.741 22.181 64.3103 22.3195 64.9689 22.3195C65.406 22.3195 65.8061 22.2579 66.1692 22.1348C66.5324 22.0117 66.8432 21.8271 67.1018 21.5808C67.3603 21.3346 67.5572 21.033 67.6927 20.676L71.3304 20.9161C71.1458 21.7901 70.7672 22.5534 70.1948 23.2058C69.6285 23.8522 68.896 24.3569 67.9974 24.7201C67.1048 25.0771 66.0738 25.2556 64.9043 25.2556ZM77.1335 6.06949V24.9786H73.2003V6.06949H77.1335ZM79.5043 24.9786V10.7968H83.4375V24.9786H79.5043ZM81.4801 8.96863C80.8954 8.96863 80.3937 8.77474 79.9752 8.38696C79.5628 7.99302 79.3566 7.52214 79.3566 6.97431C79.3566 6.43265 79.5628 5.96792 79.9752 5.58014C80.3937 5.1862 80.8954 4.98923 81.4801 4.98923C82.0649 4.98923 82.5635 5.1862 82.9759 5.58014C83.3944 5.96792 83.6037 6.43265 83.6037 6.97431C83.6037 7.52214 83.3944 7.99302 82.9759 8.38696C82.5635 8.77474 82.0649 8.96863 81.4801 8.96863ZM89.7415 16.7797V24.9786H85.8083V10.7968H89.5569V13.2989H89.723C90.037 12.4741 90.5632 11.8216 91.3019 11.3415C92.0405 10.8552 92.9361 10.6121 93.9887 10.6121C94.9735 10.6121 95.8322 10.8275 96.5647 11.2584C97.2971 11.6893 97.8665 12.3048 98.2728 13.105C98.679 13.899 98.8821 14.8469 98.8821 15.9487V24.9786H94.9489V16.6505C94.9551 15.7826 94.7335 15.1055 94.2841 14.6192C93.8348 14.1268 93.2162 13.8806 92.4283 13.8806C91.8989 13.8806 91.4311 13.9944 91.0249 14.2222C90.6248 14.4499 90.3109 14.7823 90.0831 15.2193C89.8615 15.6502 89.7477 16.1703 89.7415 16.7797ZM107.665 25.2556C106.206 25.2556 104.951 24.9601 103.898 24.3692C102.852 23.7721 102.045 22.9289 101.479 21.8394C100.913 20.7437 100.63 19.448 100.63 17.9523C100.63 16.4935 100.913 15.2132 101.479 14.1114C102.045 13.0096 102.842 12.1509 103.87 11.5354C104.905 10.9199 106.117 10.6121 107.508 10.6121C108.444 10.6121 109.315 10.7629 110.121 11.0645C110.934 11.36 111.641 11.8062 112.245 12.4033C112.854 13.0004 113.328 13.7513 113.667 14.6561C114.005 15.5548 114.174 16.6074 114.174 17.8138V18.8941H102.199V16.4566H110.472C110.472 15.8903 110.349 15.3886 110.103 14.9516C109.856 14.5146 109.515 14.1729 109.078 13.9267C108.647 13.6744 108.145 13.5482 107.573 13.5482C106.976 13.5482 106.446 13.6867 105.985 13.9637C105.529 14.2345 105.172 14.6007 104.914 15.0624C104.655 15.5179 104.523 16.0257 104.517 16.5858V18.9033C104.517 19.605 104.646 20.2113 104.905 20.7222C105.169 21.2331 105.542 21.627 106.022 21.904C106.502 22.181 107.071 22.3195 107.73 22.3195C108.167 22.3195 108.567 22.2579 108.93 22.1348C109.293 22.0117 109.604 21.8271 109.863 21.5808C110.121 21.3346 110.318 21.033 110.454 20.676L114.091 20.9161C113.907 21.7901 113.528 22.5534 112.956 23.2058C112.389 23.8522 111.657 24.3569 110.758 24.7201C109.866 25.0771 108.835 25.2556 107.665 25.2556Z",
                    class: "fill-blue-600 ", //dark:fill-white"
                }
                path {
                    "stroke-width": "2",
                    "d": "M1 28.9786V15.9786C1 9.35116 6.37258 3.97858 13 3.97858C19.6274 3.97858 25 9.35116 25 15.9786C25 22.606 19.6274 27.9786 13 27.9786H12",
                    "stroke": "currentColor",
                    class: "stroke-blue-600 ", //dark:stroke-white"
                }
                path {
                    "stroke-width": "2",
                    "d": "M5 28.9786V16.1386C5 11.6319 8.58172 7.97858 13 7.97858C17.4183 7.97858 21 11.6319 21 16.1386C21 20.6452 17.4183 24.2986 13 24.2986H12",
                    "stroke": "currentColor",
                    class: "stroke-blue-600 ", //dark:stroke-white"
                }
                circle {
                    "fill": "currentColor",
                    "cx": "13",
                    "r": "5",
                    "cy": "16",
                    class: "fill-blue-600 ", //dark:fill-white"
                }
            }
        }
    }
}


// #[derive(Default)]
// enum Size {
//     Small,
//     #[default]
//     Medium,
//     Large
// }

// impl Display for Size{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self{
//             Size::Small => "small".fmt(f),
//             Size::Medium => "medium".fmt(f),
//             Size::Large => "large".fmt(f),
//         }
//     }
// }

// impl FromStr for Size {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         use Size::*;
//         match s.to_lowercase().as_str(){
//             "small"=> Ok(Small),
//             "medium" => Ok(Medium),
//             "large" => Ok(Large),
//             _ => Err(())
//         }
//     }
// }

// #[component]
// pub fn Product(id: u32) -> Element {
//     let quantity = use_signal(|| 1);
//     let size = use_signal(||  Size::default);

//     // let product = use_future!(cx, |()| async move{
//     //     fetch_product(id).await
//     // });
//     let product = use_resource(move || fetch_product(id));

//     // check if the future is resolved
//     match &*product.read_unchecked() {
//         Some(Ok(value)) => {
//             let Product {  title, price, description, category, image, rating,.. }= value;
//             // if it is, render the stories
//             rsx! {
//                 section { class: "py-20",
//                 div { class: "container mx-auto px-4",
//                     div { class: "flex flex-wrap -mx-4 mb-24",
//                         div { class: "w-full md:w-1/2 px-4 mb-8 md:mb-0",
//                             div { class: "relative mb-10",
//                                 style: "height: 564px;",
//                                 a { class: "absolute top-1/2 left-0 ml-8 transform translate-1/2",
//                                     href: "#",
//                                     icons::icon_0 {}
//                                 }
//                                 img { class: "object-cover w-full h-full",
//                                     alt: "",
//                                     src: "{image}",
//                                 }
//                                 a { class: "absolute top-1/2 right-0 mr-8 transform translate-1/2",
//                                     href: "#",
//                                     icons::icon_1 {}
//                                 }
//                             }
//                         }
//                         div { class: "w-full md:w-1/2 px-4",
//                             div { class: "lg:pl-20",
//                                 div { class: "mb-10 pb-10 border-b",
//                                     h2 { class: "mt-2 mb-6 max-w-xl text-5xl md:text-6xl font-bold font-heading",
//                                         "{title}"
//                                     }
//                                     div { class: "mb-8",
//                                         "{rating}"
//                                     }
//                                     p { class: "inline-block mb-8 text-2xl font-bold font-heading text-blue-300",
//                                         span {
//                                             "${price}"
//                                         }
//                                     }
//                                     p { class: "max-w-md text-gray-500",
//                                         "{description}"
//                                     }
//                                 }
//                                 div { class: "flex mb-12",
//                                     div { class: "mr-6",
//                                         span { class: "block mb-4 font-bold font-heading text-gray-400 uppercase",
//                                             "QTY"
//                                         }
//                                         div { class: "inline-flex items-center px-4 font-semibold font-heading text-gray-500 border border-gray-200 focus:ring-blue-300 focus:border-blue-300 rounded-md",
//                                             button { class: "py-2 hover:text-gray-700",
//                                                 // onclick: move |_| quantity.modify(|q| *q + 1),
//                                                 icons::icon_2 {}
//                                             }
//                                             input { class: "w-12 m-0 px-2 py-4 text-center md:text-right border-0 focus:ring-transparent focus:outline-none rounded-md",
//                                                 placeholder: "1",
//                                                 r#type: "number",
//                                                 value: "{quantity}",
//                                                 // oninput: move |evt| if let Ok(as_number) = evt.value.parse() { quantity.set(as_number) },
//                                             }
//                                             button { class: "py-2 hover:text-gray-700",
//                                                 // onclick: move |_| quantity.modify(|q| q - 1),
//                                                 icons::icon_3 {}
//                                             }
//                                         }
//                                     }
//                                     div {
//                                         span { class: "block mb-4 font-bold font-heading text-gray-400 uppercase",
//                                             "Size"
//                                         }
//                                         select { class: "pl-6 pr-10 py-4 font-semibold font-heading text-gray-500 border border-gray-200 focus:ring-blue-300 focus:border-blue-300 rounded-md",
//                                             id: "",
//                                             name: "",
//                                             // onchange: move |evt| {
//                                             //     if let Ok(new_size) = evt.value().parse() {
//                                             //         size.set(new_size);
//                                             //     }
//                                             // },
//                                             option {
//                                                 value: "1",
//                                                 "Medium"
//                                             }
//                                             option {
//                                                 value: "2",
//                                                 "Small"
//                                             }
//                                             option {
//                                                 value: "3",
//                                                 "Large"
//                                             }
//                                         }
//                                     }
//                                 }
//                                 div { class: "flex flex-wrap -mx-4 mb-14 items-center",
//                                     div { class: "w-full xl:w-2/3 px-4 mb-4 xl:mb-0",
//                                         a { class: "block bg-orange-300 hover:bg-orange-400 text-center text-white font-bold font-heading py-5 px-8 rounded-md uppercase transition duration-200",
//                                             href: "#",
//                                             "Add to cart"
//                                         }
//                                     }
//                                 }
//                                 div { class: "flex items-center",
//                                     span { class: "mr-8 text-gray-500 font-bold font-heading uppercase",
//                                         "SHARE IT"
//                                     }
//                                     a { class: "mr-1 w-8 h-8",
//                                         href: "#",
//                                         img {
//                                             alt: "",
//                                             src: "https://shuffle.dev/yofte-assets/buttons/facebook-circle.svg",
//                                         }
//                                     }
//                                     a { class: "mr-1 w-8 h-8",
//                                         href: "#",
//                                         img {
//                                             alt: "",
//                                             src: "https://shuffle.dev/yofte-assets/buttons/instagram-circle.svg",
//                                         }
//                                     }
//                                     a { class: "w-8 h-8",
//                                         href: "#",
//                                         img {
//                                             src: "https://shuffle.dev/yofte-assets/buttons/twitter-circle.svg",
//                                             alt: "",
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                     div {
//                         ul { class: "flex flex-wrap mb-16 border-b-2",
//                             li { class: "w-1/2 md:w-auto",
//                                 a { class: "inline-block py-6 px-10 bg-white text-gray-500 font-bold font-heading shadow-2xl",
//                                     href: "#",
//                                     "Description"
//                                 }
//                             }
//                             li { class: "w-1/2 md:w-auto",
//                                 a { class: "inline-block py-6 px-10 text-gray-500 font-bold font-heading",
//                                     href: "#",
//                                     "Customer reviews"
//                                 }
//                             }
//                             li { class: "w-1/2 md:w-auto",
//                                 a { class: "inline-block py-6 px-10 text-gray-500 font-bold font-heading",
//                                     href: "#",
//                                     "Shipping &amp; returns"
//                                 }
//                             }
//                             li { class: "w-1/2 md:w-auto",
//                                 a { class: "inline-block py-6 px-10 text-gray-500 font-bold font-heading",
//                                     href: "#",
//                                     "Brand"
//                                 }
//                             }
//                         }
//                         h3 { class: "mb-8 text-3xl font-bold font-heading text-blue-300",
//                             "{category}"
//                         }
//                         p { class: "max-w-2xl text-gray-500",
//                             "{description}"
//                         }
//                     }
//                 }
//             }
//             }
//         }
//         Some(Err(err)) => {
//             // if there was an error, render the error
//             rsx! {"An error occurred while fetching product {err}"}
//         }
//         None => {
//             // if the future is not resolved yet, render a loading message
//             rsx! {"Loading items"}
//         }
//     }

// }

// mod icons {
//     use super::*;

//     pub(super) fn icon_0() -> Element {
//         rsx!(
//             svg { class: "w-6 h-6",
//                 view_box: "0 0 24 23",
//                 xmlns: "http://www.w3.org/2000/svg",
//                 height: "23",
//                 fill: "none",
//                 width: "24",
//                 path {
//                     stroke: "black",
//                     fill: "black",
//                     d: "M2.01328 18.9877C2.05682 16.7902 2.71436 12.9275 6.3326 9.87096L6.33277 9.87116L6.33979 9.86454L6.3398 9.86452C6.34682 9.85809 8.64847 7.74859 13.4997 7.74859C13.6702 7.74859 13.8443 7.75111 14.0206 7.757L14.0213 7.75702L14.453 7.76978L14.6331 7.77511V7.59486V3.49068L21.5728 10.5736L14.6331 17.6562V13.6558V13.5186L14.4998 13.4859L14.1812 13.4077C14.1807 13.4075 14.1801 13.4074 14.1792 13.4072M2.01328 18.9877L14.1792 13.4072M2.01328 18.9877C7.16281 11.8391 14.012 13.3662 14.1792 13.4072M2.01328 18.9877L14.1792 13.4072M23.125 10.6961L23.245 10.5736L23.125 10.4512L13.7449 0.877527L13.4449 0.571334V1V6.5473C8.22585 6.54663 5.70981 8.81683 5.54923 8.96832C-0.317573 13.927 0.931279 20.8573 0.946581 20.938L0.946636 20.9383L1.15618 22.0329L1.24364 22.4898L1.47901 22.0885L2.041 21.1305L2.04103 21.1305C4.18034 17.4815 6.71668 15.7763 8.8873 15.0074C10.9246 14.2858 12.6517 14.385 13.4449 14.4935V20.1473V20.576L13.7449 20.2698L23.125 10.6961Z",
//                     stroke_width: "0.35",
//                 }
//             }
// 		)
// 	}

//     pub(super) fn icon_1() -> Element {
//         rsx!(
//             svg { class: "w-6 h-6",
//                 height: "27",
//                 view_box: "0 0 27 27",
//                 fill: "none",
//                 width: "27",
//                 xmlns: "http://www.w3.org/2000/svg",
//                 path {
//                     d: "M13.4993 26.2061L4.70067 16.9253C3.9281 16.1443 3.41815 15.1374 3.24307 14.0471C3.06798 12.9568 3.23664 11.8385 3.72514 10.8505V10.8505C4.09415 10.1046 4.63318 9.45803 5.29779 8.96406C5.96241 8.47008 6.73359 8.14284 7.54782 8.00931C8.36204 7.87578 9.19599 7.93978 9.98095 8.19603C10.7659 8.45228 11.4794 8.89345 12.0627 9.48319L13.4993 10.9358L14.9359 9.48319C15.5192 8.89345 16.2327 8.45228 17.0177 8.19603C17.8026 7.93978 18.6366 7.87578 19.4508 8.00931C20.265 8.14284 21.0362 8.47008 21.7008 8.96406C22.3654 9.45803 22.9045 10.1046 23.2735 10.8505V10.8505C23.762 11.8385 23.9306 12.9568 23.7556 14.0471C23.5805 15.1374 23.0705 16.1443 22.298 16.9253L13.4993 26.2061Z",
//                     stroke: "black",
//                     stroke_width: "1.5",
//                     stroke_linecap: "round",
//                     stroke_linejoin: "round",
//                 }
//             }
// 		)
// 	}

//     pub(super) fn icon_2() -> Element {
//         rsx!(
//             svg {
//                 view_box: "0 0 12 12",
//                 height: "12",
//                 width: "12",
//                 fill: "none",
//                 xmlns: "http://www.w3.org/2000/svg",
//                 g {
//                     opacity: "0.35",
//                     rect {
//                         height: "12",
//                         x: "5",
//                         fill: "currentColor",
//                         width: "2",
//                     }
//                     rect {
//                         fill: "currentColor",
//                         width: "2",
//                         height: "12",
//                         x: "12",
//                         y: "5",
//                         transform: "rotate(90 12 5)",
//                     }
//                 }
//             }
// 		)
// 	}

//     pub(super) fn icon_3() -> Element {
//         rsx!(
//             svg {
//                 width: "12",
//                 fill: "none",
//                 view_box: "0 0 12 2",
//                 height: "2",
//                 xmlns: "http://www.w3.org/2000/svg",
//                 g {
//                     opacity: "0.35",
//                     rect {
//                         transform: "rotate(90 12 0)",
//                         height: "12",
//                         fill: "currentColor",
//                         x: "12",
//                         width: "2",
//                     }
//                 }
//             }
// 		)
// 	}
// }
