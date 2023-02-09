use crate::errors;
use async_std;
use serde_json;

#[derive(Debug, Clone)]
pub struct PlaybackApi {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

impl crate::playback::Playback for PlaybackApi {
    fn get_playback_info(&self, id: &String) -> Result<serde_json::Value, errors::Error> {
        self.clone()._get_playback_info(id)
    }
}

impl PlaybackApi {
    pub fn new(client: &crate::LivepeerClient) -> Self {
        PlaybackApi {
            client: client.clone(),
            urls: crate::api::urls::LivepeerUrls::new(),
        }
    }

    pub fn _get_playback_info(self, id: &String) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!(
                "{}{}/{}",
                self.client.config.host, self.urls.playback.get_playback_info, id
            ),
            self.client,
        );
        res
    }
}
