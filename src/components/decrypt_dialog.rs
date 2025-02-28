use dioxus::prelude::*;

use crate::{
    api::{DecryptResponse, EmptyResponse},
    ticks::Ticker,
};

pub type DDialogData = Result<Option<DecryptResponse>, ServerFnError<EmptyResponse>>;

#[component]
pub fn DecryptDialog(
    mut show: Signal<bool>,
    mut data: Signal<DDialogData>,
    mut dialog_counter: Signal<Ticker>,
    mut closed: Signal<Option<bool>>,
) -> Element {
    rsx! {
        if show() {
            div {
                aria_labelledby: "modal-title",
                aria_modal: "true",
                class: "relative z-50",
                id: "modal",
                role: "dialog",

                div {
                    class: "z-50 fixed inset-0 bg-white/70 bg-opacity-75 transition-opacity opacity-100",
                    id: "backdrop",
                }

                div { class: "fixed inset-0 z-50 w-screen overflow-y-auto",
                    div { class: "flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0",
                        div {
                            class: "relative transform overflow-hidden flex flex-col flex-grow rounded-lg bg-brand px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6",
                            id: "dialog",

                            match data() {
                                Ok(op_data) => {
                                    match op_data {
                                        Some(DecryptResponse { data: dec_data }) => rsx !{
                                            div {
                                                class: "flex flex-col",
                                                id: "modal-data",

                                                div { class: "mx-auto flex-grow flex h-12 w-12 items-center justify-center rounded-full bg-green-100",
                                                    svg {
                                                        class: "h-6 w-6 text-green-600",
                                                        fill: "none",
                                                        stroke: "currentColor",
                                                        stroke_width: "1.5",
                                                        view_box: "0 0 24 24",
                                                        path {
                                                            d: "M4.5 12.75l6 6 9-13.5",
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                        }
                                                    }
                                                }
                                                div { class: "mt-3 sm:mt-5 flex-grow",
                                                    h3 {
                                                        class: "text-base text-center font-semibold leading-6 text-gray-900",
                                                        id: "modal-title",
                                                        "Decrypted Data"
                                                    }
                                                    div { class: "mt-2",
                                                        div { class: "sm:col-span-4 mt-2 p-1",
                                                            label {
                                                                class: "block text-sm font-medium leading-6 text-gray-800",
                                                                "Data"
                                                            }
                                                            div { class: "mt-2",
                                                                input {
                                                                    autocomplete: "none",
                                                                    class: "block w-full rounded-md px-2 border-0 bg-gray-900/5 py-1.5 text-gray-900 shadow-sm sm:text-sm sm:leading-6",
                                                                    disabled: true,
                                                                    value: dec_data,
                                                                    placeholder: "Data",
                                                                }
                                                            }
                                                        }
                                                        div { class: "sm:col-span-4 mt-2 p-1 text-center",
                                                            label {
                                                                class: "block text-sm font-medium leading-6 text-gray-800",
                                                                id: "closing",

                                                                {format!("Dialog will close in {} seconds", dialog_counter().remaining())}
                                                            }
                                                        }
                                                    }
                                                }
                                                div { class: "mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3",
                                                    button {
                                                        class: "mt-3 inline-flex w-full justify-center rounded-md bg-dark-brand px-3 py-2 text-sm font-semibold text-brand shadow-sm hover:cursor-pointer hover:bg-dark-brand/70 sm:col-span-2 sm:mt-0",
                                                        onclick: move |_| async move {
                                                            let data = data().unwrap().unwrap();
                                                            let _ = web_sys::window().unwrap().navigator().clipboard().write_text(data.data.as_str());
                                                            show.set(false);
                                                            closed.set(Some(true));
                                                        },
                                                        r#type: "button",
                                                        svg {
                                                            class: "mr-2 h-5 w-5",
                                                            fill: "currentColor",
                                                            view_box: "0 0 448 512",
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            path { d: "M208 0H332.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V336c0 26.5-21.5 48-48 48H208c-26.5 0-48-21.5-48-48V48c0-26.5 21.5-48 48-48zM48 128h80v64H64V448H256V416h64v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V176c0-26.5 21.5-48 48-48z" }
                                                        }
                                                        "Copy Data"
                                                    }
                                                }
                                            }
                                        },
                                        None => rsx! {
                                            div {
                                                class: "mx-auto flex-grow rounded-full bg-brand",
                                                id: "progress",
                                                svg {
                                                    class: "w-8 h-8 text-white animate-spin fill-dark-brand",
                                                    fill: "none",
                                                    view_box: "0 0 100 101",
                                                    xmlns: "http://www.w3.org/2000/svg",
                                                    path {
                                                        d: "M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z",
                                                        fill: "currentColor",
                                                    }
                                                    path {
                                                        d: "M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z",
                                                        fill: "currentFill",
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                Err(err) => rsx! {
                                    div {
                                        class: "hidden flex flex-col",
                                        id: "modal-err",
                                        div { class: "mx-auto flex h-12 w-12 flex-grow items-center justify-center rounded-full bg-red-100",
                                            svg {
                                                class: "h-6 w-6 text-red-600",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "1.5",
                                                view_box: "0 0 24 24",
                                                path {
                                                    d: "M 5 4 L 17 18 M 5 18 l 12 -14",
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                }
                                            }
                                        }
                                        div { class: "mt-3 text-center sm:mt-5",
                                            h3 {
                                                class: "text-base font-semibold leading-6 text-gray-900",
                                                id: "modal-err-title",

                                                {format!("Error occured {}", match err {
                                                    ServerFnError::WrappedServerError(e) => format!("{:?}", e),
                                                    ServerFnError::Response(s) => s,
                                                    ServerFnError::Deserialization(s) => s,
                                                    _ => "".to_string(),
                                                })}
                                            }
                                        }
                                        div { class: "mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3",
                                            button {
                                                class: "mt-3 inline-flex w-full justify-center rounded-md bg-gray-900 px-3 py-2 text-sm font-semibold text-accent shadow-sm ring-1 ring-inset ring-gray-900 hover:bg-gray-800 sm:col-span-2 sm:mt-0",
                                                onclick: move |_| async move {
                                                    show.set(false);
                                                    closed.set(Some(true));
                                                },
                                                r#type: "button",
                                                "Close"
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
    }
}
