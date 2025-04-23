use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct GithubIconProps {
    size: Option<String>,
    color: Option<String>,
}

#[component]
pub fn GithubIcon(props: GithubIconProps) -> Element {
    let size = props.size.unwrap_or("36px".to_string());
    let color = props.color.unwrap_or("currentColor".to_string());

    rsx! {
        svg {
            view_box: "0 0 20.025242 19.51556",
            width: "{size}",
            height: "{size}",
            fill: "{color}",
            path {
                d: "M 10.037717,7.1127397e-5 A 10,10 0 0 0 6.877717,19.500071 c 0.5,0.08 0.66,-0.23 0.66,-0.5 v -1.69 c -2.77,0.6 -3.36,-1.31 -3.36,-1.31 a 2.69,2.69 0 0 0 -1.14,-1.5 c -0.91,-0.62 0.07,-0.6 0.07,-0.6 a 2.1,2.1 0 0 1 1.53,1 2.15,2.15 0 0 0 2.91,0.83 2.16,2.16 0 0 1 0.63,-1.34 c -2.14,-0.22 -4.52,-1.08 -4.52,-4.8899999 a 3.87,3.87 0 0 1 1,-2.71 3.58,3.58 0 0 1 0.1,-2.64 c 0,0 0.84,-0.27 2.75,1 a 9.63,9.63 0 0 1 5,0 c 1.91,-1.29 2.75,-1 2.75,-1 a 3.58,3.58 0 0 1 0.1,2.64 3.87,3.87 0 0 1 1,2.71 c 0,3.8199999 -2.34,4.6599999 -4.57,4.9099999 a 2.39,2.39 0 0 1 0.69,1.85 v 2.74 c 0,0.27 0.16,0.59 0.67,0.5 a 10,10 0 0 0 -3.11,-19.499999872603 z"
            }
        }
    }
}
