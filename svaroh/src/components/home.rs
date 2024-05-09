use crate::{ api::*, server::{ get_server_data, post_server_data }, Route };
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    // Fetch the top 10 stories on Hackernews
    let products = use_resource(move || fetch_products(10, Sort::Ascending));

    // check if the future is resolved
    match &*products.read_unchecked() {
        Some(Ok(list)) => {
            // if it is, render the stories
            rsx! {
               body {
                   section { class: "p-10",
                       for product in list {
                          // p { "Server data: {product.title}" }

                           Item {
                               product: product.clone()
                           }
                       }
                   }
               }
            }
        }
        Some(Err(err)) => {
            // if there was an error, render the error
            rsx! {
                "An error occurred while fetching stories {err}"
            }
        }
        None => {
            // if the future is not resolved yet, render a loading message
            rsx! {
                "Loading items"
            }
        }
    }
}

#[component]
fn Item(product: Product) -> Element {
    let Product { id, title, price, category, image, rating, .. } = product;

    rsx! {
        Link { to: Route::Product { id } ,
        section { class: "h-40 p-2 m-2 shadow-lg ring-1 rounded-lg flex flex-row place-items-center hover:ring-4 hover:shadow-2xl transition-all duration-200",
            img {
                class: "object-scale-down w-1/6 h-full",
                src: "{image}",
            }
            div { class: "pl-4 text-left text-ellipsis",
                a {
                    class: "w-full text-center",
                    "{title}"
                }
                p {
                    class: "w-full",
                    "{rating}"
                }
                p {
                    class: "w-full",
                    "{category}"
                }
                p {
                    class: "w-1/4",
                    "${price}"
                }
            }
        }
    }
    }
}

// #[component]
// pub fn Home() -> Element {
//     let mut count = use_signal(|| 0);
//     let mut text = use_signal(|| String::from("..."));

//     // rsx! {
//     //     Link {
//     //         to: Route::Blog {
//     //             id: count()
//     //         },
//     //         "Go to blog"
//     //     }
//     //     div {
//     //         h1 { "High-Five counter: {count}" }
//     //         button { onclick: move |_| count += 1, "Up high!" }
//     //         button { onclick: move |_| count -= 1, "Down low!" }
//     //         button {
//     //             onclick: move |_| async move {
//     //                 if let Ok(data) = get_server_data().await {
//     //                     tracing::info!("Client received: {}", data);
//     //                     text.set(data.clone());
//     //                     post_server_data(data).await.unwrap();
//     //                 }
//     //             },
//     //             "Get Server Data"
//     //         }
//     //         p { "Server data: {text}"}
//     //     }
//     // }
//     let grey_background = true;
//     rsx!(
//         div {
//             header {
//                 class: "text-gray-400 body-font",
//                 // you can use optional attributes to optionally apply a tailwind class
//                 class: if grey_background {
//                     "bg-gray-900"
//                 },
//                 div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
//                     a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
//                         StacksIcon {}
//                         span { class: "ml-3 text-xl", "Hello Dioxus!" }
//                     }
//                     nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
//                         a { class: "mr-5 hover:text-white", "First Link" }
//                         a { class: "mr-5 hover:text-white", "Second Link" }
//                         a { class: "mr-5 hover:text-white", "Third Link" }
//                         a { class: "mr-5 hover:text-white", "Fourth Link" }
//                     }
//                     button { class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
//                         "Button"
//                         RightArrowIcon {}
//                     }
//                 }
//             }

//             section { class: "text-gray-400 bg-gray-900 body-font",
//                 div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
//                     div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
//                         h1 { class: "title-font sm:text-4xl text-3xl mb-4 font-medium text-white",
//                             br { class: "hidden lg:inline-block" }
//                             "Dioxus Sneak Peek"
//                         }
//                         p { class: "mb-8 leading-relaxed",

//                             "Dioxus is a new UI framework that makes it easy and simple to write cross-platform apps using web
//                             technologies! It is functional, fast, and portable. Dioxus can run on the web, on the desktop, and
//                             on mobile and embedded platforms."
//                         }
//                         div { class: "flex justify-center",
//                             button { class: "inline-flex text-white bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg",
//                                 "Learn more"
//                             }
//                             button { class: "ml-4 inline-flex text-gray-400 bg-gray-800 border-0 py-2 px-6 focus:outline-none hover:bg-gray-700 hover:text-white rounded text-lg",
//                                 "Build an app"
//                             }
//                         }
//                     }
//                     div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
//                         img {
//                             class: "object-cover object-center rounded",
//                             src: "https://i.imgur.com/oK6BLtw.png",
//                             referrerpolicy: "no-referrer",
//                             alt: "hero"
//                         }
//                     }
//                     Link {
//                         to: Route::Blog {
//                             id: count()
//                         },
//                         "Go to blog"
//                     }
//                     div {
//                         h1 { "High-Five counter: {count}" }
//                         button { onclick: move |_| count += 1, "Up high!" }
//                         button { onclick: move |_| count -= 1, "Down low!" }
//                         button {
//                             onclick: move |_| async move {
//                                 if let Ok(data) = get_server_data().await {
//                                     tracing::info!("Client received: {}", data);
//                                     text.set(data.clone());
//                                     post_server_data(data).await.unwrap();
//                                 }
//                             },
//                             "Get Server Data"
//                         }
//                         p { "Server data: {text}"}
//                     }
//                 }
//             }
//         }
//     )
// }

// fn StacksIcon() -> Element {
//     rsx!(
//         svg {
//             fill: "none",
//             stroke: "currentColor",
//             stroke_linecap: "round",
//             stroke_linejoin: "round",
//             stroke_width: "2",
//             class: "w-10 h-10 text-white p-2 bg-indigo-500 rounded-full",
//             view_box: "0 0 24 24",
//             path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" }
//         }
//     )
// }

// fn RightArrowIcon() -> Element {
//     rsx!(
//         svg {
//             fill: "none",
//             stroke: "currentColor",
//             stroke_linecap: "round",
//             stroke_linejoin: "round",
//             stroke_width: "2",
//             class: "w-4 h-4 ml-1",
//             view_box: "0 0 24 24",
//             path { d: "M5 12h14M12 5l7 7-7 7" }
//         }
//     )
// }
