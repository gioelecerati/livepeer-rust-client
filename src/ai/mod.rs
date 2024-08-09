pub mod api;

pub trait Generate {
    fn text_to_image(&self, prompt: &String) -> Result<serde_json::Value, crate::errors::Error>;
}