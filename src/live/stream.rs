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

    fn get_stream_by_id(&self, stream_id: String) -> Result<serde_json::Value, errors::Error> {
        self.clone().get_stream_by_id(stream_id)
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

    /// Get stream by id
    /// <https://docs.livepeer.com/api/live/streams.html#get-stream-by-id>
    pub fn get_stream_by_id(self: Self, stream_id: String) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!("{}{}/{}", self.client.config.host, "/api/stream", stream_id),
            self.client,
        );
        res
    }
}
