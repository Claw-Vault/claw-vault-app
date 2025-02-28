use dioxus::prelude::*;

use crate::{components::SquarePattern, Route};

const AWK_CLAW_PNG: Asset = asset!("/assets/awk-claw.png");

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    // println!("{:?}", segments);
    rsx! {
        main { class: "relative isolate place-items-center bg-white",

            SquarePattern {  }

            div { class: "justify-center text-center flex flex-col mx-auto max-w-7xl px-6 py-12 sm:py-12 lg:flex lg:items-center lg:gap-x-10 lg:px-8 lg:py-24",
                div { class: "lg:flex-shrink-0 lg:flex-grow justify-center flex",
                    img {
                        alt: "",
                        class: "w-72",
                        src: AWK_CLAW_PNG,
                    }
                }
                p { class: "text-3xl font-semibold text-brand", "404" }
                h1 { class: "mt-4 text-3xl font-bold tracking-tight text-dark-brand sm:text-5xl",
                    "Data not found"
                }
                p { class: "mt-6 text-base leading-7 text-gray-500",
                    "Sorry, there is no claw with for this data ID."
                }
                div { class: "mt-10 flex items-center justify-center gap-x-6",
                    Link {
                        to: Route::Home {  },
                        class: "rounded-md bg-brand px-3.5 py-2.5 text-sm font-semibold text-gray-900 shadow-sm hover:bg-[#9ad4d8]/90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2",

                        "Go back home"
                    }
                }
            }
        }
    }
}
