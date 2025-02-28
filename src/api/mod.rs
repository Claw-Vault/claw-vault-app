use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmptyResponse {
    pub status: u16,
    pub message: String,
}
unsafe impl Send for EmptyResponse {}
impl Display for EmptyResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status, self.message)
    }
}
impl FromStr for EmptyResponse {
    type Err = String;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(EmptyResponse {
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

#[derive(Serialize)]
struct DecryptRequest {
    id: String,
    key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecryptResponse {
    pub data: String,
}

#[server(EncryptServer)]
pub async fn encrypt_data(
    data: String,
    validity: usize,
) -> Result<EncryptResponse, ServerFnError<EmptyResponse>> {
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
        return Err(match res.json::<EmptyResponse>().await {
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

#[server(DecryptServer)]
pub async fn decrypt_data(
    id: String,
    key: String,
) -> Result<DecryptResponse, ServerFnError<EmptyResponse>> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/v1/decrypt", get_api_url()))
        .json(&DecryptRequest { id, key })
        .send()
        .await;

    let res = match res {
        Ok(r) => r,
        Err(err) => return Err(ServerFnError::Response(format!("{:?}", err))),
    };
    if !res.status().is_success() {
        return Err(match res.json::<EmptyResponse>().await {
            Ok(err) => ServerFnError::WrappedServerError(err),
            Err(e) => {
                ServerFnError::Deserialization(format!("Failed to deserialize respones: {:?}", e))
            }
        });
    }

    res.json::<DecryptResponse>().await.map_err(|e| {
        ServerFnError::Deserialization(format!("Failed to deserialize respones: {:?}", e))
    })
}

#[server(ClawExistsServer)]
pub async fn claw_exists(id: String) -> Result<Option<()>, ServerFnError> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/api/v1/claw/{}", get_api_url(), id))
        .send()
        .await;

    let res = match res {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };
    if !res.status().is_success() {
        return Ok(None);
    }

    Ok(Some(()))
}
fn get_api_url() -> String {
    env!("BACKEND_URL").to_owned()
}
