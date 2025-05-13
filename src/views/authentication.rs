use crate::components::layouts::Footer;
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::tid;
use std::collections::HashMap;
use verbali_design_system::components::{
    cards::SwitchPanel,
    forms::{Button, Input},
};

#[cfg(feature = "server")]
use crate::database::models::{InsertableUser, User};

#[component]
pub fn Authentication(mut mode: String) -> Element {
    if mode.is_empty() {
        mode = "login".to_string();
    }
    let mut panel_show = use_signal(|| mode.clone());
    let mut signup_values = use_signal(HashMap::new);
    let mut login_values = use_signal(HashMap::new);

    let panel_classes = "flex flex-col justify-center p-16";
    let hidden_classes = "flex-1 bg-blue-600";
    let show_classes = "flex-1";

    rsx! {
        div {
            class: "flex h-screen justify-center items-center p-4",

            div {
                class: "flex items-stretch w-full max-w-4xl h-[550px] bg-white dark:bg-slate-800 shadow-xl rounded-lg overflow-hidden",

                SwitchPanel {
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

                        form {
                            oninput: move |event| {
                                signup_values.set(event.data().values());
                            },

                            onsubmit: move |_| {
                                async move {
                                    let email = signup_values.read().get("email").unwrap().as_value();
                                    let password = signup_values.read().get("password").unwrap().as_value();
                                    let confirm = signup_values.read().get("confirm").unwrap().as_value();
                                    match signup(email, password, confirm).await
                                    {
                                        Ok(user) => {
                                            document::eval(&format!("console.log('{user}')"));
                                        },
                                        Err(err) => {
                                            document::eval(&format!("console.log('{err}')"));
                                        }
                                    }
                                }
                            },

                            Input {
                                class: "my-4",
                                name: "email",
                                label: tid!("email"),
                                placeholder: tid!("email.placeholder"),
                                type: "email",
                                required: true,
                            },

                            Input {
                                class: "my-4",
                                name: "password",
                                label: tid!("password"),
                                placeholder: "······",
                                type: "password",
                                required: true,
                            },

                            Input {
                                class: "my-4",
                                name: "confirm",
                                label: tid!("password.confirm"),
                                placeholder: "······",
                                type: "password",
                                required: true,
                            },

                            div {
                                class: "text-center",
                                Button<Route> {
                                    label: tid!("signup"),
                                    type: "submit"
                                }
                            }
                        }
                    ),
                },

                SwitchPanel {
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

                        form {
                            oninput: move |event| {
                                login_values.set(event.data().values());
                            },

                            onsubmit: move |_| {
                                async move {
                                    let email = login_values.read().get("email").unwrap().as_value();
                                    let password = login_values.read().get("password").unwrap().as_value();
                                    if let Ok(user) = login(email, password).await
                                    {
                                        document::eval(&format!("console.log('{user}')"));
                                    }
                                }
                            },

                            Input {
                                class: "my-4",
                                name: "email",
                                label: tid!("email"),
                                placeholder: tid!("email.placeholder"),
                                type: "email",
                                required: true,
                            },

                            Input {
                                class: "my-4",
                                name: "password",
                                label: tid!("password"),
                                placeholder: "······",
                                type: "password",
                                required: true,
                            }

                            div {
                                class: "text-center",
                                Button<Route> {
                                    label: tid!("login"),
                                    type: "submit",
                                }
                            }
                        }

                    ),
                }
            }
        }
        Footer {}
    }
}

#[server(prefix = "/api", endpoint = "login")]
async fn login(email: String, password: String) -> Result<String, ServerFnError> {
    match User::find(email, password) {
        Ok(user) => Ok(serde_json::to_string(&user).unwrap()),
        Err(_) => Err(ServerFnError::new("User not found")),
    }
}

#[server(prefix = "/api", endpoint = "signup")]
async fn signup(email: String, password: String, confirm: String) -> Result<String, ServerFnError> {
    if (password != confirm) {
        return Err(ServerFnError::new(
            "Both password and confirmation must be same",
        ));
    }

    match User::create(InsertableUser { email, password }) {
        Ok(user) => Ok(serde_json::to_string(&user).unwrap()),
        Err(err) => Err(err.into()),
    }
}
