use crate::errors;
use serde_json;

#[derive(Debug, Clone)]
pub struct Stream {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

impl crate::live::Stream for Stream {
    /// List all streams
    ///
    /// # Returns
    /// * `Result<crate::data::stream::Streams, errors::Error>` - A list of streams or an error
    fn list_streams(&self) -> Result<crate::data::stream::Streams, errors::Error> {
        self.clone().list_streams()
    }

    /// Get stream by ID
    ///
    /// # Parameters
    /// * `stream_id` - The ID of the stream
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the stream or an error
    fn get_stream_by_id(&self, stream_id: String) -> Result<serde_json::Value, errors::Error> {
        self.clone().get_stream_by_id(stream_id)
    }

    /// Get streams by user ID
    ///
    /// # Parameters
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// * `Result<crate::data::stream::Streams, errors::Error>` - A list of streams or an error
    fn get_streams_by_user_id(
        &self,
        user_id: String,
    ) -> Result<crate::data::stream::Streams, errors::Error> {
        self.clone().get_streams_by_user_id(user_id)
    }

    /// Get stream by playback ID
    ///
    /// # Parameters
    /// * `playback_id` - The ID of the playback
    /// * `admin` - A boolean indicating if the request is made by an admin
    ///
    /// # Returns
    /// * `Result<serde_json::Value, crate::errors::Error>` - A JSON value containing the stream or an error
    fn get_stream_by_playback_id(
            &self,
            playback_id: String,
            admin: bool,
        ) -> Result<serde_json::Value, crate::errors::Error> {
            self.clone().get_stream_by_playback_id(playback_id, admin)
    }

    /// Create a stream
    ///
    /// # Parameters
    /// * `name` - The name of the stream
    /// * `profiles` - A list of profiles for the stream
    /// * `playback_policy` - An optional playback policy
    ///
    /// # Returns
    /// * `Result<String, errors::Error>` - The ID of the created stream or an error
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
    /// Create a new Stream instance
    ///
    /// # Parameters
    /// * `client` - A reference to the LivepeerClient
    ///
    /// # Returns
    /// * `Self` - A new instance of Stream
    pub fn new(client: &crate::LivepeerClient) -> Self {
        Stream {
            client: client.clone(),
            urls: crate::api::urls::LivepeerUrls::new(),
        }
    }

    /// List all streams
    ///
    /// # Returns
    /// * `Result<crate::data::stream::Streams, errors::Error>` - A list of streams or an error
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

    /// Get stream by ID
    ///
    /// # Parameters
    /// * `stream_id` - The ID of the stream
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the stream or an error
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

    /// Get stream by playback ID
    ///
    /// # Parameters
    /// * `playback_id` - The ID of the playback
    /// * `admin` - A boolean indicating if the request is made by an admin
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the stream or an error
    pub fn get_stream_by_playback_id(
        self: Self,
        playback_id: String,
        admin: bool,
    ) -> Result<serde_json::Value, errors::Error> {
        let mut admin_string = String::new();
        if admin {
            admin_string = String::from("&allUsers=true&all=true");
        }
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!(
                r#"{}{}?filters=[{{"id":"playbackId","value":"{}"}}]{}"#,
                self.client.config.host, "/api/stream", playback_id, admin_string
            ),
            self.client,
        );
        res
    }

    /// Get streams by user ID
    ///
    /// # Parameters
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// * `Result<crate::data::stream::Streams, errors::Error>` - A list of streams or an error
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
    ///
    /// # Parameters
    /// * `name` - The name of the stream
    /// * `profiles` - A list of profiles for the stream
    /// * `playback_policy` - An optional playback policy
    ///
    /// # Returns
    /// * `Result<String, errors::Error>` - The ID of the created stream or an error
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
