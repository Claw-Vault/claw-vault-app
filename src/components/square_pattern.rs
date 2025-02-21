use dioxus::prelude::*;

#[component]
pub fn SquarePattern() -> Element {
    rsx! {
        svg {
            class: "absolute inset-0 -z-10 h-full w-full stroke-brand",
            svg {
                class: "overflow-visible fill-brand/30",
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
    }
}
