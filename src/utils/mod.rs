use crate::errors;
use async_std;
use serde_json;
use surf::StatusCode;
use reqwest;
use tokio;
use reqwest::multipart;

pub struct SurfRequest {}

impl SurfRequest {
    async fn make_request(
        method: surf::http::Method,
        url: String,
        body: Option<String>,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        let mut req = surf::Request::builder(method, url.parse().unwrap())
            .header("Authorization", format!("Bearer {}", client.config.api_token));

        if let Some(body) = body {
            req = req.header("Content-Type", "application/json").body(body);
        }

        let mut res: Result<serde_json::Value, errors::Error> = Err(errors::Error::UNKNOWN);

        let response = req.await;

        match response {
            Ok(mut response) => match response.status() {
                StatusCode::Ok | StatusCode::Created | StatusCode::NoContent => {
                    if response.status() == StatusCode::NoContent {
                        res = Ok(serde_json::Value::Null);
                    } else {
                        let body = response.body_json::<serde_json::Value>().await.unwrap();
                        res = Ok(body);
                    }
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

        res
    }

    pub fn get(
        url: String,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        async_std::task::block_on(Self::make_request(surf::http::Method::Get, url, None, client))
    }

    pub fn post(
        url: String,
        body: String,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        async_std::task::block_on(Self::make_request(surf::http::Method::Post, url, Some(body), client))
    }

    pub fn patch(
        url: String,
        body: String,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        async_std::task::block_on(Self::make_request(surf::http::Method::Patch, url, Some(body), client))
    }

    pub fn delete(
        url: String,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        async_std::task::block_on(Self::make_request(surf::http::Method::Delete, url, None, client))
    }
}

pub struct ReqwestRequest {}

impl ReqwestRequest {
    async fn make_request(
        method: reqwest::Method,
        url: String,
        body: Option<String>,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        let req_client = reqwest::Client::new();
        let mut req = req_client.request(method, &url)
            .header("Authorization", format!("Bearer {}", client.config.api_token));


        if let Some(body) = body {
            req = req.header("Content-Type", "application/json").body(body);
        }

        let response = req.send().await.map_err(|_| errors::Error::UNKNOWN)?;

        match response.status() {
            reqwest::StatusCode::OK | reqwest::StatusCode::CREATED | reqwest::StatusCode::NO_CONTENT => {
                if response.status() == reqwest::StatusCode::NO_CONTENT {
                    Ok(serde_json::Value::Null)
                } else {
                    let text = response.text().await.map_err(|_| errors::Error::UNKNOWN)?;
                    serde_json::from_str(&text).map_err(|_| errors::Error::UNKNOWN)
                }
            }
            _ => {
                let err = errors::Error::from_reqwest_response(&response);
                Err(err)
            }
        }
    }

    pub fn get(
        url: String,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        tokio::runtime::Runtime::new().unwrap().block_on(Self::make_request(reqwest::Method::GET, url, None, client))
    }

    pub fn post(
        url: String,
        body: String,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        tokio::runtime::Runtime::new().unwrap().block_on(Self::make_request(reqwest::Method::POST, url, Some(body), client))
    }

    pub fn patch(
        url: String,
        body: String,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        tokio::runtime::Runtime::new().unwrap().block_on(Self::make_request(reqwest::Method::PATCH, url, Some(body), client))
    }

    pub fn delete(
        url: String,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        tokio::runtime::Runtime::new().unwrap().block_on(Self::make_request(reqwest::Method::DELETE, url, None, client))
    }

    pub fn post_multipart(
        url: String,
        form: multipart::Form,
        client: crate::LivepeerClient,
    ) -> Result<serde_json::Value, errors::Error> {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let req_client = reqwest::Client::new();
            let req = req_client.post(&url)
                .header("Authorization", format!("Bearer {}", client.config.api_token))
                .multipart(form);

            let response = req.send().await.map_err(|_| errors::Error::UNKNOWN)?;

            match response.status() {
                reqwest::StatusCode::OK | reqwest::StatusCode::CREATED | reqwest::StatusCode::NO_CONTENT => {
                    if response.status() == reqwest::StatusCode::NO_CONTENT {
                        Ok(serde_json::Value::Null)
                    } else {
                        let text = response.text().await.map_err(|_| errors::Error::UNKNOWN)?;
                        serde_json::from_str(&text).map_err(|_| errors::Error::UNKNOWN)
                    }
                }
                _ => {
                    let err = errors::Error::from_reqwest_response(&response);
                    Err(err)
                }
            }
        })
    }
}