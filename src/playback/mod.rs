pub mod api;

pub trait Playback {
    fn get_playback_info(&self, id: &String) -> Result<serde_json::Value, crate::errors::Error>;
}
