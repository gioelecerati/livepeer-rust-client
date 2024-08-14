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

    fn image_to_video(&self, image_file_path: &String) -> Result<serde_json::Value, errors::Error> {
        self.clone()._image_to_video(image_file_path)
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
        let mut result: Result<serde_json::Value, errors::Error> = Err(errors::Error::GENERATE);
        let mut data = serde_json::json!({
            "prompt": prompt,
        });
        let res: Result<serde_json::Value, errors::Error> = crate::utils::ReqwestRequest::post(
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

    pub fn _image_to_video(
        self: Self,
        image_file_path: &String,
    ) -> Result<serde_json::Value, errors::Error> {
        let file = std::fs::read(image_file_path).unwrap();
        let file_part = reqwest::multipart::Part::bytes(file)
            .file_name("bg.jpg")
            .mime_str("image/jpg")
            .unwrap();
        let form = reqwest::multipart::Form::new()
            .part("image", file_part);

        let res: Result<serde_json::Value, errors::Error> = crate::utils::ReqwestRequest::post_multipart(
            format!("{}{}", self.client.config.host, self.urls.generate.image_to_video),
            form,
            self.client,
        );

        res.and_then(|value| serde_json::from_value(value).map_err(|_| errors::Error::GENERATE))
    }
}