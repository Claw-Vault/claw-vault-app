use chrono::{Datelike, Utc};
use dioxus::prelude::*;

use crate::Route;

const FAVICON_PNG: Asset = asset!("/assets/favicon.png");

#[component]
pub fn Footer() -> Element {
    let current_date = Utc::now();

    rsx! {
        Outlet::<Route> {}

        footer { class: "relative isolate overflow-hidden p-4 md:p-8 lg:p-10 bg-accent text-gray-900",
            div { class: "mx-auto max-w-screen-xl text-center",
                Link { class: "flex justify-center items-center text-2xl font-semibold text-gray-900", to: Route::Home { },
                    img {
                        alt: "Claw Vault",
                        class: "w-12 mr-4",
                        src: FAVICON_PNG,
                    }
                    "Claw Vault"
                }
                p { class: "my-6",
                    "Open-source platform for securely sharing private information."
                }
                ul { class: "flex flex-wrap justify-center items-center mb-6",
                    li {
                        a {
                            class: "mr-4 hover:underline md:mr-6",
                            href: "https://github.com/Claw-Vault/claw-vault",
                            target: "_blank",
                            "Github"
                        }
                    }
                    li {
                        Link {
                            class: "mr-4 hover:underline md:mr-6",
                            to: Route::Privacy { },
                            "Privacy"
                        }
                    }
                }
                span { class: "text-sm sm:text-center",
                    {format!("© {} ", current_date.year())}
                    a { class: "hover:underline", href: "/", "Claw Vault ™" }
                    ". All Rights Reserved."
                }
            }
        }
    }
}
