use core::panic;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

#[derive(Serialize)]
struct EncryptRequest {
    data: String,
    valid: usize,
}

#[derive(Debug, Deserialize)]
pub struct EncryptResponse {
    data_id: String,
    key_id: String,
    valid_for: String,
}

pub async fn encrypt_data(data: String, valid: usize) -> Result<EncryptResponse, ErrorResponse> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/v1/encrypt", get_api_url()))
        .json(&EncryptRequest { data, valid })
        .send()
        .await;

    let res = match res {
        Ok(r) => r,
        Err(err) => panic!("{:?}", err),
    };
    if !res.status().is_success() {
        match res.json::<ErrorResponse>().await {
            Ok(err) => return Err(err),
            Err(e) => panic!("Failed to parse err: {:?}", e),
        };
    }

    match res.json::<EncryptResponse>().await {
        Ok(r) => return Ok(r),
        Err(e) => panic!("Failed to parse response: {:?}", e),
    }
}

fn get_api_url() -> String {
    env!("BACKEND_URL").to_owned()
}
