use crate::components::{
    icons::{FacebookIcon, GithubIcon, InstagramIcon, LinkedinIcon},
    logos::FullLogo,
};
use dioxus::prelude::*;
use dioxus_i18n::{prelude::*, tid};
use unic_langid::LanguageIdentifier;

#[component]
pub fn Footer() -> Element {
    let mut i18n = i18n();
    let mut change_language = move |lang: LanguageIdentifier| i18n.set_language(lang);
    let mut mode = use_signal(|| None);

    use_effect(move || match mode() {
        Some(theme_mode) => {
            document::eval(&format!(
                r#"window.localStorage.setItem('theme', '{theme_mode}');
                    document.documentElement.setAttribute('data-theme', '{theme_mode}');"#
            ));
        }
        None => {}
    });

    rsx! {
        div {
            class: "max-w-4xl lg:mx-auto lg:w-4xl p-4 mt-32",
            div {
                class: "flex flex-wrap justify-center my-8",
                FullLogo {}
            },
            div {
                class: "flex flex-wrap justify-center",
                div {
                    onclick: move |_| change_language("en".parse().expect("No 'en' language")),
                    class: "inline-block p-2 cursor-pointer",
                    "en"
                },
                div {
                    onclick: move |_| change_language("fr".parse().expect("No 'fr' language")),
                    class: "inline-block p-2 cursor-pointer",
                    "fr"
                }
            },
            div {
                class: "flex flex-wrap justify-center",
                div {
                    onclick: move |_| mode.set(Some("light")),
                    class: "inline-block p-2 cursor-pointer",
                    {tid!("light")}
                },
                div {
                    onclick: move |_| mode.set(Some("dark")),
                    class: "inline-block p-2 cursor-pointer",
                    {tid!("dark")}
                }
            },
            div {
                class: "flex justify-center items-center my-8",
                div {
                    class: "m-4",
                    FacebookIcon {}
                },
                div {
                    class: "m-4",
                    LinkedinIcon {}
                },
                div {
                    class: "m-4",
                    InstagramIcon {}
                },
                div {
                    class: "m-4",
                    GithubIcon {}
                },
            }
        }
    }
}
