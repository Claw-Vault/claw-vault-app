use dioxus::prelude::*;

const AWK_CLAW_PNG: Asset = asset!("/assets/awk-claw.png");

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    println!("{:?}", segments);
    rsx! {
        main { class: "relative isolate place-items-center bg-gray-900",
            svg {
                class: "absolute inset-0 -z-10 h-full w-full stroke-accent",
                svg {
                    class: "overflow-visible fill-accent-30",
                    x: "50%",
                    y: "-1",
                    path {
                        d: "M-100.5 0h201v201h-201Z M699.5 0h201v201h-201Z M499.5 400h201v201h-201Z M-300.5 600h201v201h-201Z",
                        stroke_width: "0",
                    }
                }
                rect {
                    fill: "url(#83fd4e5a-9d52-42fc-97b6-718e5d7ee527)",
                    height: "100%",
                    stroke_width: "0",
                    width: "100%",
                }
            }
            div { class: "justify-center text-center flex flex-col mx-auto max-w-7xl px-6 py-12 sm:py-12 lg:flex lg:items-center lg:gap-x-10 lg:px-8 lg:py-24",
                div { class: "lg:flex-shrink-0 lg:flex-grow justify-center flex",
                    img {
                        alt: "",
                        class: "w-72",
                        src: AWK_CLAW_PNG,
                    }
                }
                p { class: "text-3xl font-semibold text-accent", "404" }
                h1 { class: "mt-4 text-3xl font-bold tracking-tight text-gray-100 sm:text-5xl",
                    "Data not found"
                }
                p { class: "mt-6 text-base leading-7 text-gray-400",
                    "Sorry, there is no claw with for this data ID."
                }
                div { class: "mt-10 flex items-center justify-center gap-x-6",
                    a {
                        class: "rounded-md bg-accent px-3.5 py-2.5 text-sm font-semibold text-gray-900 shadow-sm hover:bg-[#9ad4d8]/90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2",
                        href: "/",
                        "Go back home"
                    }
                }
            }
        }
    }
}
