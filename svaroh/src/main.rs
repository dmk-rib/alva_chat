use dioxus::prelude::*;
use components::{chat::Chat, home::Home, not_found::PageNotFound, nav_bar::NavBar};

mod components { 
    pub mod home;
    pub mod nav_bar;
    pub mod not_found;
    pub mod chat;
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
    #[end_layout]
    #[route("/chat/:..term")] //details/1
    Chat { term: Vec<String> },
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DarkMode(bool);

impl DarkMode {
    pub fn prefix(&self) -> &str {
       if self.0 { "drak:" } else { "" } 
    }
}


fn main() {
    use tracing::Level;

    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));
    
    rsx! {
        Router::<Route> {}
    }
}