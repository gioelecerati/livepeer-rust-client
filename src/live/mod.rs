pub mod rtmp;
pub mod stream;

pub trait Stream {
    fn list_streams(&self) -> Result<crate::data::stream::Streams, crate::errors::Error>;
    fn get_stream_by_id(
        &self,
        stream_id: String,
    ) -> Result<serde_json::Value, crate::errors::Error>;
    fn get_streams_by_user_id(
        &self,
        user_id: String,
    ) -> Result<crate::data::stream::Streams, crate::errors::Error>;
    fn create_stream(
        &self,
        name: &String,
        profiles: &Vec<crate::data::stream::Profile>,
    ) -> Result<String, crate::errors::Error>;
}
