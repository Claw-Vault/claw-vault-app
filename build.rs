use core::panic;

fn main() {
    dotenv::dotenv().ok();

    if let Ok(api_url) = std::env::var("BACKEND_URL") {
        println!("cargo:rustc-env=BACKEND_URL={api_url}");
    } else {
        panic!("BACKEND_URL must be present at compile time");
    }
}
