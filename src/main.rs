use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use unic_langid::langid;
use verbali_design_system::assets::{DS_CSS, LOGO_SVG};

use views::Authentication;

mod components;
mod helpers;
mod views;

#[cfg(feature = "server")]
mod models;
#[cfg(feature = "server")]
mod schema;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[redirect("/", || Route::Authentication { mode: "login".to_string() })]
    #[route("/:mode")]
    Authentication { mode: String },
}

fn main() {
    dioxus::launch(App);
}

const MAIN_CSS: Asset = asset!("/assets/style.css");

#[component]
fn App() -> Element {
    // Init dioxus-i18n
    use_init_i18n(|| {
        I18nConfig::new(langid!("fr"))
            .with_locale((langid!("fr"), include_str!("./locales/fr.ftl")))
            .with_locale((langid!("en"), include_str!("./locales/en.ftl")))
    });

    // Init dark/light mode
    document::eval(
        r#"let theme = (localStorage.theme ==='dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) ? 'dark' : 'light';
        document.documentElement.setAttribute('data-theme', theme);"#,
    );

    rsx! {
        document::Link { rel: "icon", href: LOGO_SVG }
        document::Link { rel: "stylesheet", href: DS_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
