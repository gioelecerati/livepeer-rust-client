use crate::errors;
use serde_json;

#[derive(Debug, Clone)]
pub struct Stream {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

impl crate::live::Stream for Stream {
    fn list_streams(&self) -> Result<serde_json::Value, errors::Error> {
        self.clone().list_streams()
    }
}

impl Stream {
    pub fn new(client: &crate::LivepeerClient) -> Self {
        Stream {
            client: client.clone(),
            urls: crate::api::urls::LivepeerUrls::new(),
        }
    }

    /// List all streams
    /// <https://docs.livepeer.com/api/live/streams.html#list-all-streams>
    pub fn list_streams(self: Self) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!("{}{}", self.client.config.host, "/api/stream"),
            self.client,
        );
        res
    }
}
