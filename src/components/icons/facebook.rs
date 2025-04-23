use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct FacebookIconProps {
    size: Option<String>,
    color: Option<String>,
}

#[component]
pub fn FacebookIcon(props: FacebookIconProps) -> Element {
    let size = props.size.unwrap_or("36px".to_string());
    let color = props.color.unwrap_or("currentColor".to_string());

    rsx! {
        svg {
            view_box: "0 0 20.000026 19.92002",
            width: "{size}",
            height: "{size}",
            fill: "{color}",
            path {
                d: "M 10,0 C 4.5,0 0,4.49 0,10.02002 c 0,5 3.66,9.15 8.44,9.9 v -7 H 5.9 v -2.9 H 8.44 V 7.81 c 0,-2.51 1.49,-3.89 3.78,-3.89 1.09,0 2.23,0.19 2.23,0.19 v 2.47 h -1.26 c -1.24,0 -1.63,0.77 -1.63,1.56002 v 1.88 h 2.78 l -0.45,2.9 h -2.33 v 7 c 2.3564,-0.3722 4.5022,-1.5745 6.0499,-3.39 1.5477,-1.8154 2.3955,-4.1244 2.3901,-6.51 C 20,4.49 15.5,0 10,0 Z"
            }
        }
    }
}
