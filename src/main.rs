use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use unic_langid::langid;

use views::Authentication;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[redirect("/", || Route::Authentication { mode: "login".to_string() })]
    #[route("/:mode")]
    Authentication { mode: String },
}

const FAVICON: Asset = asset!("/assets/logo.svg");
const MAIN_CSS: Asset = asset!("/assets/style.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

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
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
