use dioxus::prelude::*;

use components::{Echo, Footer, Navbar};
use views::{Blog, Home, NotFound, Privacy, Vault};

mod api;
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[layout(Footer)]
    #[route("/")]
    Home {},

    #[route("/privacy")]
    Privacy {},

    #[route("/:id")]
    Vault { id: String },

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.png");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta { charset: "UTF-8" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Meta {
            name:"description",
            content:"Claw Vault is a platform to share sensitive information with desired recipients. Claw Vault encrypts your data, providing a unique ID and key for secure transmission.",
        }

        document::Meta { name:"og:title", content:"Claw Vault" }
        document::Meta {
            name:"og:description",
            content:"Claw Vault is a platform to share sensitive information with desired recipients. Claw Vault encrypts your data, providing a unique ID and key for secure transmission."
        }
        document::Meta { name:"og:image", content:"https://claw-vault.up.railway.app/assets/claw-vault.png"}
        document::Meta { name:"og:url", content:"https://claw-vault.up.railway.app/"}
        document::Meta { name:"og:site_name", content:"Claw Vault"}
        document::Meta { name:"og:type", content:"website"}

        document::Meta { name:"twitter:title", content:"Claw Vault"}
        document::Meta {
            name:"twitter:description",
            content:"Claw Vault is a platform to share sensitive information with desired recipients. Claw Vault encrypts your data, providing a unique ID and key for secure transmission."
        }
        document::Meta { name:"twitter:image", content:"https://claw-vault.up.railway.app/assets/claw-vault.png"}
        document::Meta { name:"twitter:card", content:"summary_large_image"}

        document::Meta { name:"robots", content:"index,follow"}

        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }

        main {
            Router::<Route> {}
        }
    }
}
