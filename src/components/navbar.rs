use dioxus::prelude::*;

use crate::Route;

const FAVICON_PNG: Asset = asset!("/assets/favicon.png");

#[component]
pub fn Navbar() -> Element {
    let mut show_mobile_nav = use_signal(|| false);

    rsx! {
        nav { class: "sticky absolute inset-x-0 top-0 z-30 bg-gray-900 text-gray-300",
            div { class: "flex items-center justify-between p-6 lg:px-8",
                div { class: "flex md:flex-1 flex-row items-center",
                    Link { class: "flex items-center", to: Route::Home { },
                        img { class: "size-12 mr-4", src: FAVICON_PNG }
                        p { class: "text-lg font-semibold", "ClawVault" }
                    }
                }
                div { class: "flex md:hidden",
                    button { r#type: "button", class: "-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-zinc-700",
                        onclick: move |_| async move {
                            show_mobile_nav.set(true);
                        },

                        svg { class: "size-6", fill: "none", view_box: "0 0 24 24", stroke_width: "1.5", stroke: "#9AD4D8",
                            path { stroke_linecap: "round", stroke_linejoin: "round", d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" },
                        }
                    }
                }
                div { class: "hidden md:flex md:gap-x-12 md:justify-end",
                    a { href: "https://github.com/Claw-Vault/claw-vault", target: "_blank", class: "text-md font-semibold leading-6",
                        "Github"
                    }
                    Link { to: Route::Privacy {}, class: "text-md font-semibold leading-6",
                        "Privacy"
                    }
                }
            },

            if show_mobile_nav() {
                div {
                    div { class: "fixed inset-0 z-30" }
                    div { class: "fixed inset-y-0 right-0 z-30 w-full overflow-y-auto bg-zinc-800 text-zinc-300 px-6 py-6 sm:max-w-sm",
                        div { class: "flex items-center justify-between",
                            div { class: "flex lg:flex-1 flex-row items-center",
                                a { class: "flex items-center", href: "/",
                                    img { class: "size-12 mr-4", src: FAVICON_PNG }
                                    p { class: "text-lg font-semibold", "ClawVault" }
                                }
                            }

                            button { r#type:"button", class:"-m-2.5 rounded-md p-2.5 text-zinc-700",
                                onclick: move |_| async move {
                                    show_mobile_nav.set(false);
                                },

                                svg { class:"h-6 w-6", fill:"none", view_box:"0 0 24 24", stroke_width:"1.5", stroke:"#9AD4D8",
                                    path { stroke_linecap:"round", stroke_linejoin:"round", d:"M6 18L18 6M6 6l12 12" }
                                }
                            }
                        }

                        div { class:"mt-6",
                            div { class:"-my-6",
                                div { class:"space-y-2 py-6",
                                    a { href:"https://github.com/Claw-Vault/claw-vault", target:"_blank",
                                        class:"-mx-3 block rounded-lg px-3 py-2 text-base font-semibold leading-7 hover:bg-zinc-700",
                                        "Github"
                                    }
                                    Link { to: Route::Privacy {  }, class:"-mx-3 block rounded-lg px-3 py-2 text-base font-semibold leading-7 hover:bg-zinc-700",
                                        "Privacy"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
