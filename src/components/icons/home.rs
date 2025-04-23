use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct HomeIconProps {
    size: Option<String>,
    color: Option<String>,
}

#[component]
pub fn HomeIcon(props: HomeIconProps) -> Element {
    let size = props.size.unwrap_or("36px".to_string());
    let color = props.color.unwrap_or("currentColor".to_string());

    rsx! {
        svg {
            view_box: "0 0 16 16",
            width: "{size}",
            height: "{size}",
            fill: "none",
            path {
                fill: "{color}",
                d: "M1 6V15H6V11C6 9.89543 6.89543 9 8 9C9.10457 9 10 9.89543 10 11V15H15V6L8 0L1 6Z"
            }
        }
    }
}
