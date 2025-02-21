use dioxus::prelude::*;

use crate::{api, components::EncryptDialog};

#[derive(Clone, Copy)]
enum Expiration {
    OneMin,
    FifteenMin,
    ThirtyMin,
}

impl Expiration {
    fn get_secs(&self) -> usize {
        match self {
            Expiration::OneMin => 60,
            Expiration::FifteenMin => 900,
            Expiration::ThirtyMin => 1800,
        }
    }

    fn get_text(&self) -> &str {
        match self {
            Expiration::OneMin => "1 minute",
            Expiration::FifteenMin => "15 minutes",
            Expiration::ThirtyMin => "30 minutes",
        }
    }
}

#[component]
pub fn EncryptBlock() -> Element {
    let mut show_dialog = use_signal(|| false);
    let mut show_modal_data = use_signal(|| false);
    let mut show_progress = use_signal(|| false);
    let mut show_modal_err = use_signal(|| false);

    let mut enc_data = use_signal(|| String::new());

    let expiration_radio = vec![
        Expiration::OneMin,
        Expiration::FifteenMin,
        Expiration::ThirtyMin,
    ];
    let mut selected_expiry = use_signal::<Option<Expiration>>(|| None);

    rsx! {
        EncryptDialog { show: show_dialog, show_data: show_modal_data, show_progress: show_progress, show_err: show_modal_err }

        div {
            h1 { class: "mt-10 max-w-lg text-4xl font-bold tracking-tight text-gray-100 sm:text-6xl",
                "Encrypt your private data"
            }
            div { class: "mt-10 col-span-full",
                label {
                    class: "block text-sm font-medium leading-6 text-[#9AD4D8]",
                    r#for: "enc-data",
                    "Data"
                }
                div { class: "mt-2 bg-gray-900",
                    textarea {
                        class: "block w-full px-2 rounded-md min-h-32 border-0 bg-[#9AD4D8]/10 py-1.5 text-white shadow-sm ring-1 ring-inset ring-[#9AD4D8]/10 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6",
                        id: "enc-data",
                        name: "data",
                        required: "false",
                        rows: "3",
                        oninput: move |e| async move {
                            enc_data.set(e.value());
                        },
                    }
                }
                p { class: "mt-3 text-sm leading-6 text-gray-400",
                    "Enter your private text data"
                }
            }
            div { class: "space-y-12",
                div { class: "border-b border-white/10 pb-12",
                    div { class: "mt-2 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6" }
                }
                div { class: "sm:col-span-6",
                    label { class: "text-lg font-semibold text-accent",
                        "Expiration"
                    }
                    p { class: "text-sm text-gray-300",
                        "How long do you want this data to be accessible to the recipient ?"
                    }
                    fieldset { class: "mt-4",
                        div { class: "space-y-4 sm:flex sm:items-center sm:space-x-10 sm:space-y-0",
                            for er in expiration_radio {
                                div { class: "flex items-center",
                                    input {
                                        checked: "false",
                                        class: "h-4 w-4 bg-gray-900 border-gray-300 text-[#9AD4D8] focus:ring-[#9AD4D8]",
                                        name: "expiration",
                                        r#type: "radio",
                                        oninput: move |e| async move {
                                            selected_expiry.set(if e.value() == "on" { Some(er) } else { None });
                                        },
                                    }
                                    label {
                                        class: "ml-3 block text-sm font-medium leading-6 text-gray-100",
                                        {er.get_text()}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "mt-10 flex items-center gap-x-6",
                button { class: "hover:cursor-pointer inline-flex items-center gap-x-2 rounded-md bg-[#9AD4D8] px-3.5 py-2.5 text-sm font-semibold text-gray-900 shadow-sm hover:bg-accent focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2",
                    disabled: selected_expiry().is_none() || enc_data().is_empty(),
                    onclick: move |_| async move {
                        on_encrypt(
                            &mut selected_expiry,
                            &mut enc_data,
                            move |show| {
                                toggle_dialog(show, &mut show_dialog, &mut show_modal_data, &mut show_progress, &mut show_modal_err);
                            },
                            move || {},
                            move || {
                                show_modal_err.set(true);
                            }
                        ).await;
                    },

                    svg {
                        class: "-ml-0.5 h-5 w-5",
                        fill: "currentColor",
                        view_box: "0 0 448 512",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M144 144v48H304V144c0-44.2-35.8-80-80-80s-80 35.8-80 80zM80 192V144C80 64.5 144.5 0 224 0s144 64.5 144 144v48h16c35.3 0 64 28.7 64 64V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V256c0-35.3 28.7-64 64-64H80z" }
                    }
                    "Encrypt"
                }
            }
        }
    }
}

async fn on_encrypt(
    selected_expiry: &mut Signal<Option<Expiration>>,
    enc_data: &mut Signal<String>,
    mut on_encrypt_click: impl FnMut(bool),
    mut on_success: impl FnMut(),
    mut on_err: impl FnMut(),
) {
    let expiry = match selected_expiry() {
        Some(e) => e,
        None => return,
    };

    let duration = expiry.get_secs();
    let data = enc_data();

    selected_expiry.set(None);
    enc_data.set(String::new());

    on_encrypt_click(true);

    println!("{:?}", api::encrypt_data(data, duration).await);
}

fn toggle_dialog(
    show: bool,
    show_modal: &mut Signal<bool>,
    show_modal_data: &mut Signal<bool>,
    show_progress: &mut Signal<bool>,
    show_modal_err: &mut Signal<bool>,
) {
    show_modal_data.set(false);
    show_progress.set(true);
    show_modal.set(show);
    show_modal_err.set(false);
}
