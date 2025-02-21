use dioxus::prelude::*;

use crate::components::{DecryptBlock, EncryptBlock, SquarePattern};

const CLAW_VAULT_PNG: Asset = asset!("/assets/claw-vault.png");

#[component]
pub fn Home() -> Element {
    let dec_data = use_signal(|| String::new());

    rsx! {
        section { class: "relative isolate pt-14 bg-gray-900",

            SquarePattern {}

            div { class: "mx-auto max-w-7xl px-6 py-12 sm:py-12 lg:flex lg:items-center lg:gap-x-10 lg:px-8 lg:py-24",
                div { class: "lg:flex-shrink-0 lg:flex-grow justify-center flex",
                    img {
                        alt: "",
                        class: "rounded-3xl shadow-xl",
                        src: CLAW_VAULT_PNG,
                    }
                }
                div { class: "mx-auto max-w-2xl lg:mx-0 lg:flex-auto justify-center",
                    h1 { class: "mt-10 max-w-lg text-4xl font-bold tracking-tight text-gray-100 sm:text-6xl",
                        "A better way to share private data"
                    }
                    p { class: "mt-6 text-lg leading-8 font-semibold text-gray-400",
                        "Claw Vault encrypts your data, providing a unique ID and key for secure transmission. Your recipient can easily access the encrypted content using the ID and key, ensuring confidentiality. Data is automatically deleted upon access or expiration, prioritizing your privacy."
                    }
                    div { class: "mt-10 flex items-center gap-x-6",
                        a {
                            class: "rounded-md bg-accent px-3.5 py-2.5 text-sm font-semibold text-gray-900 shadow-sm hover:bg-accent focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                            href: "#claw",
                            "Get started"
                        }
                        a {
                            class: "text-sm font-semibold leading-6 text-accent",
                            href: "https://github.com/Claw-Vault/claw-vault",
                            target: "_blank",

                            "View on GitHub →"
                        }
                    }
                }
            }
        }

        section {
            class: "relative isolate overflow-hidden bg-gray-900",
            id: "claw",

            SquarePattern {}

            div { class: "mx-auto max-w-7xl pb-24 pt-4 sm:pb-32 lg:grid lg:grid-cols-2 lg:gap-x-8 lg:px-8 lg:py-40",
                div { class: "px-6 lg:px-0 lg:pt-4",
                    div { class: "mx-auto max-w-2xl",
                        div { class: "max-w-lg",
                            EncryptBlock { }
                        }
                    }
                }
                div { class: "mt-20 sm:mt-24 md:mx-auto md:max-w-2xl lg:mx-0 lg:mt-0 lg:w-screen",
                    div { class: "shadow-lg md:rounded-3xl",
                        div { class: "bg-accent [clip-path:inset(0)] md:[clip-path:inset(0_round_theme(borderRadius.3xl))]",
                            div { class: "mx-auto max-w-2xl lg:mx-0 p-16 lg:flex-auto",
                                DecryptBlock {}
                            }
                        }
                    }
                }
            }
        }
        // script {
        //     "function getData() {\n            var id = document.getElementById('dec-id').value;\n            if (!id) {\n                return;\n            }\n            document.getElementById('dec-id').value = \"\";\n            window.location.href = '/' + id;\n        }\n\n        function encryptData() {\n            var valid = 60;\n            if (document.getElementById('15-min').checked) {\n                valid = 900;\n            } else if (document.getElementById('30-min').checked) {\n                valid = 1800;\n            }\n\n            var data = document.getElementById('enc-data').value;\n            if (!data) {\n                return;\n            }\n\n            document.getElementById('enc-data').value = \"\";\n            document.getElementById('1-min').checked = true;\n\n            toggleDialog(true);\n\n            fetch('/api/v1/encrypt', {\n                method: 'POST',\n                headers: { 'Content-Type': 'application/json' },\n                body: JSON.stringify({\n                    value: data,\n                    validity: valid,\n                })\n            }).then((res) => {\n                if (res.status == 200) {\n                    res.json().then((data) => {\n                        document.getElementById('enc-url').value = data.data_id;\n                        document.getElementById('enc-key').value = data.key_id;\n                        document.getElementById('enc-valid').innerHTML = 'Valid for ' + data.valid_for;\n                        document.getElementById('progress').classList.add('hidden');\n                        document.getElementById('modal-data').classList.remove('hidden');\n                        document.getElementById('modal-err').classList.add('hidden');\n\n                        timer();\n                    });\n                } else {\n                    res.json().then((data) => {\n                        document.getElementById('modal-err').classList.remove('hidden');\n                        document.getElementById('modal-err-title').innerHTML = data.message;\n                        document.getElementById('progress').classList.add('hidden');\n                    });\n                }\n            });\n        }\n\n        function timer() {\n            let timer = 9;\n            let interval = setInterval(() => {\n                if (document.getElementById('modal').classList.contains('hidden')) {\n                    clearInterval(interval);\n                    return;\n                }\n                document.getElementById('closing').innerHTML = \"Dialog will close in \" + timer + \" seconds\";\n                timer--;\n                if (timer < 0) {\n                    clearInterval(interval);\n                }\n            }, 1000);\n            setTimeout(() => {\n                if (document.getElementById('modal').classList.contains('hidden')) {\n                    return;\n                }\n                toggleDialog(false);\n            }, 10000);\n        }\n\n        function closeDialog(failed = false) {\n            if (failed) {\n                toggleDialog(false);\n                return;\n            }\n\n            var clipboard = \"ID: \" + document.getElementById('enc-url').value + \"\\nKey: \" + document.getElementById('enc-key').value;\n            navigator.clipboard.writeText(clipboard).then(() => toggleDialog(false), () => toggleDialog(false));\n        }\n\n        function toggleDialog(show) {\n            document.getElementById('modal-data').classList.add('hidden');\n            document.getElementById('enc-url').value = \"\";\n            document.getElementById('enc-key').value = \"\";\n            document.getElementById('enc-valid').innerHTML = \"\";\n            document.getElementById('progress').classList.remove('hidden');\n            if (show) {\n                document.getElementById('modal').classList.remove('hidden');\n            } else {\n                document.getElementById('modal').classList.add('hidden');\n            }\n            document.getElementById('modal-err').classList.add('hidden');\n        }"
        // }
    }
}
