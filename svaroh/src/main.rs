 

use dioxus::prelude::*;
use components::{item_detailed::Product, home::Home, not_found::PageNotFound, nav_bar::NavBar};

mod components { 
    pub mod home;
    pub mod error;
    pub mod nav_bar;
    pub mod not_found;
    pub mod item_detailed;
}
mod api;
mod server;

const _STYLE: &str = manganis::mg!(file("assets/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
pub enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/details/:id")] //details/1
        Product { id: u32 },
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}


fn main() {
    use tracing::Level;

    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(app);
}


fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}