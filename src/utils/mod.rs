use crate::errors;
use async_std;
use serde_json;
use surf::StatusCode;

pub struct SurfRequest {}

impl SurfRequest {
    pub fn get(
        url: String,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        let mut res: Result<serde_json::Value, errors::Error> = Err(errors::Error::UNKNOWN);

        async_std::task::block_on(async {
            let response = surf::get(&format!("{}", url))
                .header(
                    "Authorization",
                    format!("Bearer {}", client.config.api_token),
                )
                .await;

            match response {
                Ok(mut response) => match response.status() {
                    StatusCode::Ok => {
                        let body = response.body_json::<serde_json::Value>().await.unwrap();
                        res = Ok(body);
                    }
                    _ => {
                        let err = errors::Error::from_response(&response);
                        res = Err(err);
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        });
        return res;
    }

    pub fn post(
        url: String,
        body: String,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        let mut res: Result<serde_json::Value, errors::Error> = Err(errors::Error::UNKNOWN);

        async_std::task::block_on(async {
            let response = surf::post(&format!("{}", url))
                .header(
                    "Authorization",
                    format!("Bearer {}", client.config.api_token),
                )
                .header("Content-Type", "application/json")
                .body(body)
                .await;

            match response {
                Ok(mut response) => match response.status() {
                    StatusCode::Ok => {
                        let body = response.body_json::<serde_json::Value>().await.unwrap();
                        res = Ok(body);
                    }
                    _ => {
                        let err = errors::Error::from_response(&response);
                        res = Err(err);
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        });
        return res;
    }
}
