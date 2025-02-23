use dioxus::prelude::*;

use crate::{
    api::{self, EncryptResponse, ErrorResponse},
    components::{encrypt_dialog::EDialogData, EncryptDialog},
    ticks::use_ticks,
};

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
    let mut dialog_data: Signal<EDialogData> = use_signal(|| Ok(None));
    let mut dialog_counter = use_ticks();

    let mut text_data = use_signal(|| String::new());

    use_effect(move || {
        if dialog_counter().completed() {
            show_dialog.set(false);
        }
    });

    let expiration_radio = vec![
        Expiration::OneMin,
        Expiration::FifteenMin,
        Expiration::ThirtyMin,
    ];
    let mut selected_expiry = use_signal::<Option<Expiration>>(|| None);

    rsx! {
        EncryptDialog { show: show_dialog, data: dialog_data, dialog_counter }

        div {
            h1 { class: "mt-10 max-w-lg text-4xl font-bold tracking-tight text-dark-brand sm:text-6xl",
                "Encrypt your private data"
            }
            div { class: "mt-12 col-span-full",
                textarea {
                    class: "block w-full px-2 rounded-md min-h-32 border-brand border bg-white py-1.5 text-dark-brand shadow-sm sm:text-sm sm:leading-6",
                    id: "enc-data",
                    name: "data",
                    required: "false",
                    rows: "3",
                    placeholder: "Enter your private text data",
                    value: text_data,
                    oninput: move |e| async move {
                        text_data.set(e.value());
                    },
                }
            }
            div { class: "space-y-12",
                div { class: "border-b border-dark-brand/10 pb-12",
                    div { class: "mt-2 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6" }
                }
                div { class: "sm:col-span-6",
                    label { class: "text-lg font-semibold text-dark-brand",
                        "Expiration"
                    }
                    p { class: "text-sm text-gray-500",
                        "How long do you want this data to be accessible to the recipient ?"
                    }
                    fieldset { class: "mt-4",
                        div { class: "space-y-4 sm:flex sm:items-center sm:space-x-10 sm:space-y-0",
                            for er in expiration_radio.into_iter() {
                                div { class: "flex items-center",
                                    input {
                                        checked: selected_expiry().map(|e| e.get_secs() == er.get_secs()).unwrap_or(false),
                                        class: "h-4 w-4 bg-dark-brand border-gray-500 text-dark-brand focus:ring-dark-brand",
                                        name: "expiration",
                                        r#type: "radio",
                                        oninput: move |e| async move {
                                            selected_expiry.set(if e.value() == "on" { Some(er) } else { None });
                                        },
                                    }
                                    label {
                                        class: "ml-3 block text-sm font-medium leading-6 text-dark-brand",
                                        {er.get_text()}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "mt-10 flex items-center gap-x-6",
                button { class: "hover:cursor-pointer inline-flex items-center gap-x-2 rounded-md bg-brand px-3.5 py-2.5 text-sm font-semibold text-dark-brand shadow-sm hover:bg-brand/70",
                    disabled: selected_expiry().is_none() || text_data().is_empty(),
                    onclick: move |_| async move {
                        on_encrypt(
                            &mut selected_expiry,
                            &mut text_data,
                            move |show| {
                               show_dialog.set(show);
                               dialog_data.set(Ok(None));
                            },
                            move |res| {
                                dialog_data.set(Ok(Some(res)));
                                dialog_counter.write().reset(10);
                            },
                            move |err| {
                                dialog_data.set(Err(err));
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
    mut on_success: impl FnMut(EncryptResponse),
    mut on_err: impl FnMut(ServerFnError<ErrorResponse>),
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

    match api::encrypt_data(data, duration).await {
        Ok(res) => on_success(res),
        Err(err) => on_err(err),
    };
}
