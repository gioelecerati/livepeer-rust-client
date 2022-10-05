pub mod api;

pub trait AccessControl {
    fn list_signing_keys(&self) -> Result<serde_json::Value, crate::errors::Error>;
    fn create_signing_key(
        &self,
        name: String,
    ) -> Result<serde_json::Value, crate::errors::Error>;
}
