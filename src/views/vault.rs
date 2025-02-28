use dioxus::prelude::*;
use dioxus_fullstack::prelude::use_server_future;

use crate::{
    api::{self, DecryptResponse, EmptyResponse},
    components::{DDialogData, DecryptDialog, SquarePattern},
    ticks::use_ticks,
    Route,
};

#[component]
pub fn Vault(id: String) -> Element {
    let mut dec_key = use_signal(|| String::new());
    let claw_id = use_signal(|| id.clone());
    let nav = navigator();

    let data = use_server_future(move || api::claw_exists(claw_id()))?;

    let mut dialog_closed = use_signal::<Option<bool>>(|| None);
    let mut show_dialog = use_signal(|| false);
    let mut dialog_data: Signal<DDialogData> = use_signal(|| Ok(None));
    let mut dialog_counter = use_ticks();

    use_effect(move || {
        if dialog_counter().completed() {
            show_dialog.set(false);
            dialog_closed.set(Some(true));
        }
    });
    use_effect(move || match dialog_closed() {
        Some(closed) => {
            if closed {
                nav.replace(Route::Home {});
            }
        }
        None => {}
    });

    match data.value().unwrap().unwrap() {
        200 => {
            rsx! {
                DecryptDialog { show: show_dialog, data: dialog_data, dialog_counter, closed: dialog_closed }

                section {
                    class: "relative isolate overflow-hidden",

                    SquarePattern { }

                    div { class: "mx-auto max-w-7xl pb-24 pt-4 sm:pb-32 lg:grid lg:grid-cols-2 lg:gap-x-8 lg:px-8 lg:py-40",
                        div { class: "mt-20 sm:mt-24 md:mx-auto md:max-w-2xl lg:mx-0 lg:mt-0 lg:w-screen",
                            div { class: "shadow-lg md:rounded-3xl",
                                div { class: "bg-brand [clip-path:inset(0)] md:[clip-path:inset(0_round_theme(borderRadius.3xl))]",
                                    div { class: "mx-auto max-w-2xl lg:mx-0 p-16 lg:flex-auto",
                                        h1 { class: "mt-10 max-w-lg text-4xl font-bold tracking-tight text-dark-brand sm:text-6xl",
                                            "Decrypt"
                                        }
                                        div { class: "sm:col-span-4 mt-6 p-1",
                                            label {
                                                class: "block text-sm font-medium leading-6 text-dark-brand",
                                                r#for: "dec-key",
                                                "Enter the Key for claw: {id}"
                                            }
                                            div { class: "mt-2 flex",
                                                input {
                                                    autocomplete: "off",
                                                    class: "flex-grow rounded-md px-2 border-0 bg-gray-900/5 py-1.5 text-gray-900 shadow-sm sm:text-sm sm:leading-6",
                                                    id: "dec-key",
                                                    name: "key",
                                                    placeholder: "Key",
                                                    r#type: "text",
                                                    required: "false",
                                                    value: dec_key,
                                                    oninput: move |e| async move {
                                                        let tokens = e.value().split('.').map(|s| s.to_owned()).collect::<Vec<_>>();
                                                        let key = if tokens.len() > 1 {
                                                            tokens.into_iter().nth(1).unwrap()
                                                        } else {
                                                            e.value()
                                                        };
                                                        dec_key.set(key);
                                                    },
                                                }
                                            }
                                        }
                                        div { class: "mt-10 flex items-center gap-x-6",
                                            button { class: "hover:cursor-pointer inline-flex items-center gap-x-2 rounded-md bg-dark-brand px-3.5 py-2.5 text-sm font-semibold text-brand shadow-sm hover:bg-dark-brand/70",
                                                onclick: move |_| async move {
                                                    show_dialog.set(true);
                                                    dialog_data.set(Ok(None));

                                                    on_decrypt(
                                                        claw_id(),
                                                        &mut dec_key,
                                                        move |data| {
                                                            dialog_data.set(Ok(Some(data)));
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
                                                    path { d: "M144 144c0-44.2 35.8-80 80-80c31.9 0 59.4 18.6 72.3 45.7c7.6 16 26.7 22.8 42.6 15.2s22.8-26.7 15.2-42.6C331 33.7 281.5 0 224 0C144.5 0 80 64.5 80 144v48H64c-35.3 0-64 28.7-64 64V448c0 35.3 28.7 64 64 64H384c35.3 0 64-28.7 64-64V256c0-35.3-28.7-64-64-64H144V144z" }
                                                }
                                                "Decrypt"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            nav.replace(format!("/{id}"));
            rsx! {}
        }
    }
}

async fn on_decrypt(
    id: String,
    dec_key: &mut Signal<String>,
    mut on_success: impl FnMut(DecryptResponse),
    mut on_err: impl FnMut(ServerFnError<EmptyResponse>),
) {
    let key = dec_key();

    dec_key.set(String::new());

    match api::decrypt_data(id, key).await {
        Ok(res) => on_success(res),
        Err(err) => on_err(err),
    };
}
