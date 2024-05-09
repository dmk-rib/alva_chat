mod pages;
mod server;
mod app;

const _STYLE: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    use tracing::Level;

    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(app::App);
}
