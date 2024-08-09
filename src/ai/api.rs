use crate::errors;
use async_std;
use serde_json;

#[derive(Debug, Clone)]
pub struct GenerateApi {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

impl crate::ai::Generate for GenerateApi {
    /// Generate an image from text
    ///
    /// # Parameters
    /// * `prompt` - Prompt for the image
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the generate information or an error
    fn text_to_image(&self, prompt: &String) -> Result<serde_json::Value, errors::Error> {
        self.clone()._text_to_image(prompt)
    }
}

impl GenerateApi {
    /// Create a new instance of GenerateApi
    ///
    /// # Parameters
    /// * `client` - A reference to the LivepeerClient
    ///
    /// # Returns
    /// * `Self` - A new instance of GenerateApi
    pub fn new(client: &crate::LivepeerClient) -> Self {
        GenerateApi {
            client: client.clone(),
            urls: crate::api::urls::LivepeerUrls::new(),
        }
    }

    /// Text to image
    ///
    /// # Parameters
    /// * `prompt` - Prompt for the image
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the generate information or an error
    pub fn _text_to_image(
        self: Self,
        prompt: &String,
    ) -> Result<serde_json::Value, errors::Error> {
        let mut result: Result<serde_json::Value, errors::Error> = Err(errors::Error::CREATESTREAM);
        let mut data = serde_json::json!({
            "prompt": prompt,
        });
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::post(
            format!("{}{}", self.client.config.host, self.urls.generate.text_to_image),
            serde_json::to_string(&data).unwrap(),
            self.client,
        );
        if res.is_ok() {
            let output: serde_json::Value = serde_json::from_value(res.unwrap()).unwrap();
            result = Ok(output)
        }
        result
    }
}