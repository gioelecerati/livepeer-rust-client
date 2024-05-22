use crate::errors;
use serde_json;

#[derive(Debug, Clone)]
pub struct Stream {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

impl crate::live::Stream for Stream {
    fn list_streams(&self) -> Result<crate::data::stream::Streams, errors::Error> {
        self.clone().list_streams()
    }

    fn get_stream_by_id(&self, stream_id: String) -> Result<serde_json::Value, errors::Error> {
        self.clone().get_stream_by_id(stream_id)
    }

    fn get_streams_by_user_id(
        &self,
        user_id: String,
    ) -> Result<crate::data::stream::Streams, errors::Error> {
        self.clone().get_streams_by_user_id(user_id)
    }

    fn create_stream(
        &self,
        name: &String,
        profiles: &Vec<crate::data::stream::Profile>,
        playback_policy: Option<serde_json::Value>,
    ) -> Result<String, errors::Error> {
        self.clone().create_stream(name, profiles, playback_policy)
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
    pub fn list_streams(self: Self) -> Result<crate::data::stream::Streams, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!("{}{}", self.client.config.host, "/api/stream?streamsonly=1"),
            self.client,
        );
        let mut r: Result<crate::data::stream::Streams, errors::Error> =
            Err(errors::Error::LISTSTREAMS);
        if res.is_ok() {
            let streams = serde_json::from_value(res.unwrap()).unwrap();
            r = Ok(streams)
        }
        r
    }

    /// Get stream by id
    /// <https://docs.livepeer.com/api/live/streams.html#get-stream-by-id>
    pub fn get_stream_by_id(
        self: Self,
        stream_id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!("{}{}/{}", self.client.config.host, "/api/stream", stream_id),
            self.client,
        );
        res
    }

    /// Get streams by user id
    /// <https://docs.livepeer.com/api/live/streams.html#get-streams-by-user-id>
    pub fn get_streams_by_user_id(
        self: Self,
        user_id: String,
    ) -> Result<crate::data::stream::Streams, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!(
                r#"{}{}?allUsers=true&streamsonly=1&order=createdAt-true&limit=1000&filters=[{{"id":"userId","value":"{}"}}]"#,
                self.client.config.host, "/api/stream", user_id
            ),
            self.client,
        );
        let mut r: Result<crate::data::stream::Streams, errors::Error> =
            Err(errors::Error::LISTSTREAMS);
        if res.is_ok() {
            let streams = serde_json::from_value(res.unwrap()).unwrap();
            r = Ok(streams)
        }
        r
    }

    /// Create a stream
    /// <https://docs.livepeer.com/api/live/streams.html#create-a-stream>
    pub fn create_stream(
        self: Self,
        name: &String,
        profiles: &Vec<crate::data::stream::Profile>,
        playback_policy: Option<serde_json::Value>,
    ) -> Result<String, errors::Error> {
        let mut result: Result<String, errors::Error> = Err(errors::Error::CREATESTREAM);
        let mut stream_id: String = "".to_string();
        let mut data = serde_json::json!({
            "name": name,
            "playbackPolicy": playback_policy.unwrap(),
            //"profiles": profiles,
        });
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::post(
            format!("{}{}", self.client.config.host, "/api/stream"),
            serde_json::to_string(&data).unwrap(),
            self.client,
        );
        if res.is_ok() {
            let stream: serde_json::Value = serde_json::from_value(res.unwrap()).unwrap();
            stream_id = stream["id"].to_string();
            result = Ok(stream_id)
        }
        result
    }
}
