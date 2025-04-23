use crate::components::logos::IconLogo;
use dioxus::prelude::*;

#[component]
pub fn FullLogo() -> Element {
    rsx! {
        div {
            class: "flex flex-row items-center font-bold text-xl font-fredoka",
            IconLogo {
                width: "w-8",
                class: "mr-2",
            }
            "verbali"
        }
    }
}
