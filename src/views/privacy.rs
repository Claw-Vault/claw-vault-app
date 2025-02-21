use dioxus::prelude::*;

#[component]
pub fn Privacy() -> Element {
    rsx! {
        document::Title { "Claw Vault - Privacy Policies" }

        section { class: "md:mt-16 mt-16 mb-16 relative overflow-hidden",
            div { class: "px-4 sm:px-6 lg:px-8",
                div { class: "relative mx-auto max-w-[37.5rem] pt-20 text-center pb-24",
                    h1 { class: "text-4xl font-extrabold tracking-tight text-slate-900 sm:text-5xl",
                        "Privacy policy"
                    }
                    p { class: "mt-4 text-base leading-7 text-slate-600",
                        "Last updated on January 24, 2024"
                    }
                }
            }
            div { class: "relative px-4 sm:px-6 lg:px-8",
                div { class: "mx-auto max-w-[40rem] text-slate-600",
                    p {
                        "ClawVault, created by Shashank Verma, is committed to maintaining the privacy and security of your personal information. This Privacy Policy outlines our practices concerning the collection, use, and disclosure of personal information when you use our service."
                    }
                    p {
                        "This page is used to inform visitors regarding my policies with the collection, use, and disclosure of Personal Information if anyone decided to use my Service."
                    }
                    br {}
                    p {
                        "The terms used in this Privacy Policy have the same meanings as in our Terms and Conditions, which are accessible at ClawVault unless otherwise defined in this Privacy Policy."
                    }
                    br {}
                    p {
                        strong { "Information Collection and Use" }
                    }
                    br {}
                    p {
                        "ClawVault is designed to prioritize the security and privacy of your data. We do not collect any analytical information or utilize third-party services. The information collected is solely what you choose to provide to the platform. This data is securely stored in an encrypted form until then specified expiration date."
                    }
                    br {}
                    p {
                        "Your trust is our priority, and we are committed to maintaining the highest standards of security for your data."
                    }
                    br {}
                    p {
                        strong { "Log Data" }
                    }
                    br {}
                    p {
                        "Whenever you use ClawVault, the app may collect Log Data, including device Internet Protocol (“IP”) address, device name, operating system version, app configuration, time and date of usage, and other statistics. This data is collected for error tracking and app improvement."
                    }
                    br {}
                    p {
                        strong { "Cookies" }
                    }
                    br {}
                    p {
                        "Cookies are files with a small amount of data that are commonly used as anonymous unique identifiers. These are sent to your browser from the websites that you visit and are stored on your device's internal memory."
                    }
                    br {}
                    p {
                        "ClawVault does not use cookies explicitly. However, third-party code and libraries used by the app may employ cookies to enhance services. Users have the option to accept or refuse these cookies, though refusal may limit access to certain features."
                    }
                    br {}
                    p {
                        strong { "Service Providers" }
                    }
                    br {}
                    p {
                        "ClawVault is hosted on the "
                        a {
                            class: "underline",
                            href: "https://railway.app",
                            target: "_blank",
                            "Railway Corporation"
                        }
                        " platform to facilitate our service. While Railway Corporation is our hosting provider, it does not involve third-party companies or individuals accessing your Personal Information beyond the necessary hosting services. Please review "
                        a {
                            class: "underline",
                            href: "https://railway.app/legal/privacy",
                            target: "_blank",
                            "Railway Corporation's Privacy Policy"
                        }
                        " for more information on their practices."
                    }
                    br {}
                    p {
                        strong { "Security" }
                    }
                    br {}
                    p {
                        "While ClawVault strives to use commercially acceptable means to protect personal information, it cannot guarantee absolute security. Users are advised to be mindful of the limitations inherent in internet transmission and electronic storage."
                    }
                    br {}
                    p {
                        "It's crucial to highlight that the decryption key required to access your data is not stored in our database; it is provided to you. In the event the key is lost, the data becomes unrecoverable and is automatically deleted upon reaching the specified expiration date. Importantly, we do not have the capability to decrypt or access your data, ensuring your information remains confidential throughout its storage period."
                    }
                    br {}
                    p {
                        strong { "Links to Other Sites" }
                    }
                    br {}
                    p {
                        "ClawVault may contain links to external sites. Users are strongly advised to review the Privacy Policies of these sites, as they are not operated by Shashank Verma, and ClawVault has no control over their content, privacy policies, or practices."
                    }
                    br {}
                    p {
                        strong { "Children’s Privacy" }
                    }
                    br {}
                    p {
                        "ClawVault does not address individuals under the age of 13 and does not knowingly collect personal information from children. If such information is discovered, it will be promptly deleted from our servers."
                    }
                    br {}
                    p {
                        strong { "Changes to This Privacy Policy" }
                    }
                    br {}
                    p {
                        "We may update our Privacy Policy from time to time. Thus, you are advised to review this page periodically for any changes. We will notify you of any changes by posting the new Privacy Policy on this page."
                    }
                    br {}
                    p { "This policy is effective as of 2024-1-24 (January 24, 2024)." }
                    br {}
                    p {
                        strong { "Contact Us" }
                    }
                    br {}
                    p {
                        "If you have any questions or suggestions about our Privacy Policy, do not hesitate to contact shashank.verma2002@gmail.com"
                    }
                }
            }
        }
    }
}
