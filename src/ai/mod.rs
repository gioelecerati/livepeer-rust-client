pub mod api;

pub trait Generate {
    fn text_to_image(&self, prompt: &String) -> Result<serde_json::Value, crate::errors::Error>;
    fn image_to_video(&self, image_file_path: &String) -> Result<serde_json::Value, crate::errors::Error>;
}