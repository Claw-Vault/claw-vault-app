use dioxus::prelude::*;

use crate::components::SquarePattern;

#[component]
pub fn Vault(id: String) -> Element {
    let mut dec_key = use_signal(|| String::new());

    rsx! {
        section {
            class: "relative isolate overflow-hidden bg-gray-900",
            id: "claw",

            SquarePattern { }

            div { class: "mx-auto max-w-7xl pb-24 pt-4 sm:pb-32 lg:grid lg:grid-cols-2 lg:gap-x-8 lg:px-8 lg:py-40",
                div { class: "mt-20 sm:mt-24 md:mx-auto md:max-w-2xl lg:mx-0 lg:mt-0 lg:w-screen",
                    div { class: "shadow-lg md:rounded-3xl",
                        div { class: "bg-accent [clip-path:inset(0)] md:[clip-path:inset(0_round_theme(borderRadius.3xl))]",
                            div { class: "mx-auto max-w-2xl lg:mx-0 p-16 lg:flex-auto",
                                form {
                                    action: "\\#",
                                    method: "dialog",
                                    "onsubmit": "decryptData()",
                                    h1 { class: "mt-10 max-w-lg text-4xl font-bold tracking-tight text-gray-800 sm:text-6xl",
                                        "Decrypt"
                                    }
                                    div { class: "sm:col-span-4 mt-6 p-1",
                                        label {
                                            class: "block text-sm font-medium leading-6 text-gray-800",
                                            r#for: "dec-key",
                                            "Enter the Key for claw: {id}"
                                        }
                                        div { class: "mt-2 flex",
                                            input {
                                                autocomplete: "off",
                                                class: "flex-grow rounded-md px-2 border-0 bg-gray-900/5 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-900/10 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6",
                                                id: "dec-key",
                                                name: "key",
                                                placeholder: "Key",
                                                r#type: "text",
                                                required: "false",
                                                oninput: move |e| async move {
                                                    dec_key.set(e.value());
                                                },
                                            }
                                        }
                                    }
                                    div { class: "mt-10 flex items-center gap-x-6",
                                        button { class: "hover:cursor-pointer inline-flex items-center gap-x-2 rounded-md bg-gray-900 px-3.5 py-2.5 text-sm font-semibold text-[#9AD4D8] shadow-sm hover:bg-accent focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2",
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

        div {
            aria_labelledby: "modal-title",
            aria_modal: "true",
            class: "relative hidden",
            id: "modal",
            role: "dialog",
            div {
                class: "fixed inset-0 bg-gray-800 bg-opacity-75 transition-opacity opacity-100",
                id: "backdrop",
            }
            div { class: "fixed inset-0 z-40 w-screen overflow-y-auto",
                div { class: "flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0",
                    div {
                        class: "relative transform overflow-hidden rounded-lg flex flex-col flex-grow bg-accent px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6",
                        id: "dialog",
                        div { class: "hidden", id: "modal-data",
                            div { class: "mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-green-100",
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
                            div { class: "mt-3 text-center sm:mt-5",
                                h3 {
                                    class: "text-base font-semibold leading-6 text-gray-900",
                                    id: "modal-title",
                                    "\n                                Decrypted Data\n                            "
                                }
                                div { class: "mt-2",
                                    div { class: "mt-10 col-span-full",
                                        label {
                                            class: "block text-sm font-medium leading-6 text-gray-900",
                                            r#for: "dec-data",
                                            "\n                                        Data\n                                    "
                                        }
                                        div { class: "mt-2 bg-gray-900 rounded-md",
                                            textarea {
                                                class: "block w-full px-2 rounded-md min-h-32 border-0 bg-[#9AD4D8]/10 py-1.5 text-white shadow-sm ring-1 ring-inset ring-[#9AD4D8]/10 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6",
                                                disabled: "false",
                                                id: "dec-data",
                                                name: "data",
                                                rows: "3",
                                            }
                                        }
                                    }
                                    div { class: "sm:col-span-4 mt-2 p-1",
                                        label {
                                            class: "block text-sm font-medium leading-6 text-gray-800",
                                            id: "closing",
                                        }
                                    }
                                }
                            }
                            div { class: "mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3",
                                button {
                                    class: "mt-3 inline-flex w-full justify-center rounded-md bg-gray-900 px-3 py-2 text-sm font-semibold text-accent shadow-sm ring-1 ring-inset ring-gray-900 hover:bg-gray-800 sm:col-span-2 sm:mt-0",
                                    "onclick": "closeDialog()",
                                    r#type: "button",
                                    svg {
                                        class: "mr-2 h-5 w-5",
                                        fill: "currentColor",
                                        view_box: "0 0 448 512",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        path { d: "M208 0H332.1c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9V336c0 26.5-21.5 48-48 48H208c-26.5 0-48-21.5-48-48V48c0-26.5 21.5-48 48-48zM48 128h80v64H64V448H256V416h64v48c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V176c0-26.5 21.5-48 48-48z" }
                                    }
                                    "\n                                Copy Data\n                            "
                                }
                            }
                        }
                        div {
                            class: "mx-auto flex rounded-full",
                            id: "progress",
                            svg {
                                class: "w-8 h-8 text-gray-200 animate-spin fill-gray-900",
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
                        div { class: "hidden", id: "modal-err",
                            div { class: "mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-red-100",
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
                                    "\n                                Error occured\n                            "
                                }
                            }
                            div { class: "mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3",
                                button {
                                    class: "mt-3 inline-flex w-full justify-center rounded-md bg-gray-900 px-3 py-2 text-sm font-semibold text-accent shadow-sm ring-1 ring-inset ring-gray-900 hover:bg-gray-800 sm:col-span-2 sm:mt-0",
                                    "onclick": "closeDialog(true)",
                                    r#type: "button",
                                    "\n                                Close\n                            "
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
