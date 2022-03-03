pub mod rtmp;
pub mod stream;

pub trait Stream {
    fn list_streams(&self) -> Result<serde_json::Value, crate::errors::Error>;
}
