#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::{home::Home, blog::Blog, not_found::PageNotFound};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}