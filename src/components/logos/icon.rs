use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/logo.svg");

#[derive(PartialEq, Props, Clone)]
pub struct IconLogoProps {
    width: Option<String>,
    class: Option<String>,
}

#[component]
pub fn IconLogo(props: IconLogoProps) -> Element {
    let widthClass = props.width.unwrap_or("w-240".to_string());
    let class = props.class.unwrap_or("".to_string());

    rsx! {
        img {
            src: LOGO,
            alt: "Logo",
            class: "{widthClass} {class}"
        }
    }
}
