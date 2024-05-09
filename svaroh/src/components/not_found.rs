use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        main { class: "grid min-h-full px-6 py-24 bg-white place-items-center sm:py-32 lg:px-8",
            div { class: "text-center",
                p { class: "text-base font-semibold text-indigo-600", "404" }
                h1 { class: "mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl", "Page not found" }
                p { class: "mt-6 text-base leading-7 text-gray-600", "Sorry, we couldn`t find the page you`re looking for." }
                div { class : "flex items-center justify-center mt-10 gap-x-6", 
                    a { class: "rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600", "Go back home" }
                    a { class: "text-sm font-semibold text-gray-900", "Contact support" }
                }
            }
            // h1 { "Page not found" }
            // p { "We are terribly sorry, but the page you requested doesn't exist." }
            // pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
        }
    }
}

// <!--
//   This example requires updating your template:

//   ```
//   <html class="h-full">
//   <body class="h-full">
//   ```
// -->
// <main class="grid min-h-full px-6 py-24 bg-white place-items-center sm:py-32 lg:px-8">
//   <div class="text-center">
//     <p class="text-base font-semibold text-indigo-600">404</p>
//     <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">Page not found</h1>
//     <p class="mt-6 text-base leading-7 text-gray-600">Sorry, we couldn’t find the page you’re looking for.</p>
//     <div class="flex items-center justify-center mt-10 gap-x-6">
//       <a href="#" class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Go back home</a>
//       <a href="#" class="text-sm font-semibold text-gray-900">Contact support <span aria-hidden="true">&rarr;</span></a>
//     </div>
//   </div>
// </main>
