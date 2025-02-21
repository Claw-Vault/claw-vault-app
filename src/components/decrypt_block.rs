use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn DecryptBlock() -> Element {
    let nav = use_navigator();
    let mut dec_id = use_signal(|| String::new());

    rsx! {
        form {
            // action: "\\#",
            // method: "dialog",
            // onsubmit: move |_| async move {
            //     // nav.push(target)
            // },
            h1 { class: "mt-10 max-w-lg text-4xl font-bold tracking-tight text-gray-800 sm:text-6xl",
                "Get Data"
            }
            div { class: "sm:col-span-4 mt-6 p-1",
                label {
                    class: "block text-sm font-medium leading-6 text-gray-800",
                    r#for: "dec-id",
                    "Data ID"
                }
                div { class: "mt-2 flex",
                    input {
                        autocomplete: "off",
                        class: "flex-grow max-w-96 rounded-md px-2 border-0 bg-gray-900/5 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-900/10 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6",
                        id: "dec-id",
                        name: "id",
                        placeholder: "ID",
                        r#type: "text",
                        required: "false",
                        oninput: move |e| async move {
                            dec_id.set(e.value());
                        },
                    }
                }
            }
            div { class: "mt-10 flex items-center gap-x-6",
                button { class: "rounded-md bg-gray-800 px-3.5 py-2.5 text-sm font-semibold text-[#9AD4D8] shadow-sm hover:bg-gray-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2",
                    onclick: move |_| async move {
                        let id = dec_id();
                        let id = id.trim();
                        if id.len() > 0 {
                            nav.push(Route::Vault { id: id.to_owned() });
                        }
                    },

                    "Get Data"
                }
            }
        }
    }
}
