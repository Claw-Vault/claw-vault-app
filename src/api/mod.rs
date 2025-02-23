use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
}
unsafe impl Send for ErrorResponse {}
impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}
impl FromStr for ErrorResponse {
    type Err = String;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(ErrorResponse {
            status: 109,
            message: String::from("FromStr NOT implemented for ErrorResponse"),
        })
    }
}

#[derive(Serialize)]
struct EncryptRequest {
    data: String,
    validity: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptResponse {
    pub id: String,
    pub key: String,
    pub valid_for: String,
}

#[server(EncryptServer)]
pub async fn encrypt_data(
    data: String,
    validity: usize,
) -> Result<EncryptResponse, ServerFnError<ErrorResponse>> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/v1/encrypt", get_api_url()))
        .json(&EncryptRequest { data, validity })
        .send()
        .await;

    let res = match res {
        Ok(r) => r,
        Err(err) => return Err(ServerFnError::Response(format!("{:?}", err))),
    };
    if !res.status().is_success() {
        return Err(match res.json::<ErrorResponse>().await {
            Ok(err) => ServerFnError::WrappedServerError(err),
            Err(e) => {
                ServerFnError::Deserialization(format!("Failed to deserialize respones: {:?}", e))
            }
        });
    }

    res.json::<EncryptResponse>().await.map_err(|e| {
        ServerFnError::Deserialization(format!("Failed to deserialize respones: {:?}", e))
    })
}

fn get_api_url() -> String {
    env!("BACKEND_URL").to_owned()
}
