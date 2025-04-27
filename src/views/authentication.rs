use crate::components::layouts::Footer;
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::tid;
use verbali_design_system::components::{
    cards::Panel,
    forms::{Button, Input},
};

#[component]
pub fn Authentication(mut mode: String) -> Element {
    if mode.is_empty() {
        mode = "login".to_string();
    }
    let mut panel_show = use_signal(|| mode.clone());

    let panel_classes = "flex flex-col justify-center p-16";
    let hidden_classes = "flex-1 bg-blue-600";
    let show_classes = "flex-1";

    rsx! {
        div {
            class: "flex h-screen justify-center items-center p-4",

            div {
                class: "flex items-stretch w-full max-w-4xl h-[550px] bg-white dark:bg-slate-800 shadow-xl rounded-lg overflow-hidden",

                Panel {
                    show: *panel_show.read() == "signup".to_string(),

                    hidden_classes: "{panel_classes} {hidden_classes}",
                    hidden_content: rsx!(
                        div {
                            class: "text-center",
                            Button<Route> {
                                onclick: move |_| {
                                    *panel_show.write() = "signup".to_string();
                                },
                                label: {tid!("signup")},
                            }
                        }
                    ),

                    show_classes: "{panel_classes} {show_classes}",
                    show_content: rsx!(
                        h1 {
                            class: "text-2xl text-center font-bold mb-4",
                            {tid!("signup")}
                        },

                        Input {
                            class: "my-4",
                            label: tid!("email"),
                            placeholder: tid!("email.placeholder"),
                            type: "email",
                            required: true,
                        },

                        Input {
                            class: "my-4",
                            label: tid!("password"),
                            placeholder: "······",
                            type: "password",
                            required: true,
                        },

                        Input {
                            class: "my-4",
                            label: tid!("password.confirm"),
                            placeholder: "······",
                            type: "password",
                            required: true,
                        },

                        div {
                            class: "text-center",
                            Button<Route> {
                                label: tid!("signup")
                            }
                        }
                    ),
                },

                Panel {
                    show: *panel_show.read() == "login".to_string(),

                    hidden_classes: "{panel_classes} {hidden_classes}",
                    hidden_content: rsx!(
                        div {
                            class: "text-center",
                            Button<Route> {
                                onclick: move |_| {
                                    *panel_show.write() = "login".to_string();
                                },
                                label: tid!("login"),
                            }
                        }
                    ),

                    show_classes: "{panel_classes} {show_classes}",
                    show_content: rsx!(
                        h1 {
                            class: "text-2xl text-center font-bold mb-4",
                            {tid!("login")}
                        },

                        Input {
                            class: "my-4",
                            label: tid!("email"),
                            placeholder: tid!("email.placeholder"),
                            type: "email",
                            required: true,
                        },

                        Input {
                            class: "my-4",
                            label: tid!("password"),
                            placeholder: "······",
                            type: "password",
                            required: true,
                        }

                        div {
                            class: "text-center",
                            Button<Route> {
                                label: tid!("login")
                            }
                        }
                    ),
                }
            }
        }
        Footer {}
    }
}
