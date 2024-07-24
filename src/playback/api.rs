use crate::errors;
use async_std;
use serde_json;

#[derive(Debug, Clone)]
pub struct PlaybackApi {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

impl crate::playback::Playback for PlaybackApi {
    /// Get playback information
    ///
    /// # Parameters
    /// * `id` - The ID of the playback
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the playback information or an error
    fn get_playback_info(&self, id: &String) -> Result<serde_json::Value, errors::Error> {
        self._get_playback_info(id)
    }
}

impl PlaybackApi {
    /// Create a new instance of PlaybackApi
    ///
    /// # Parameters
    /// * `client` - A reference to the LivepeerClient
    ///
    /// # Returns
    /// * `Self` - A new instance of PlaybackApi
    pub fn new(client: &crate::LivepeerClient) -> Self {
        PlaybackApi {
            client: client.clone(),
            urls: crate::api::urls::LivepeerUrls::new(),
        }
    }

    /// Get playback information
    ///
    /// # Parameters
    /// * `id` - The ID of the playback
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the playback information or an error
    pub fn _get_playback_info(&self, id: &String) -> Result<serde_json::Value, errors::Error> {
        crate::utils::SurfRequest::get(
            format!(
                "{}{}/{}",
                self.client.config.host, self.urls.playback.get_playback_info, id
            ),
            self.client.clone(),
        )
    }
}
