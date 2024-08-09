#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

pub mod accesscontrol;
pub mod api;
pub mod data;
pub mod errors;
pub mod live;
pub mod playback;
pub mod tests;
pub mod user;
pub mod utils;
pub mod vod;
pub mod ai;

#[derive(Debug, Clone)]
pub enum LivepeerEnv {
    Box,
    Dev,
    Stg,
    Prod,
    Origin,
    Test,
}

#[derive(Debug, Clone)]
pub struct LivepeerClient {
    pub config: LivepeerConfig,
}

#[derive(Debug, Clone)]
pub struct LivepeerConfig {
    /// The host of the Livepeer API.
    host: &'static str,
    /// User API token
    api_token: String,
    rtmp_endpoint: &'static str,
}

/// Livepeer Client to interact with the livepeer.com API
#[derive(Debug, Clone)]
pub struct Livepeer {
    /// Livepeer Client
    _client: LivepeerClient,
    /// Livepeer Environment
    _env: LivepeerEnv,
    /// Vod API set
    pub asset: vod::api::VodApi,
    /// Access Control API set
    pub access_control: accesscontrol::api::AccessControlApi,
    /// Task API set
    pub task: vod::task::TaskApi,
    /// User API set
    pub user_api: user::UserApi,
    /// Rtmp push utils
    pub rtmp: live::rtmp::Rtmp,
    /// Stream API set
    pub stream: live::stream::Stream,
    /// User Infos
    pub user: user::User,
    /// Playback Info
    pub playback: playback::api::PlaybackApi,
    /// AI API set
    pub generate: ai::api::GenerateApi,
}

impl LivepeerClient {
    /// Create a new Livepeer Client
    /// # Arguments
    /// * `api_token` - User API token
    /// * `env` - Livepeer Environment
    fn new(api_token: String, env: Option<LivepeerEnv>) -> Self {
        let (host, rtmp_endpoint) = match env {
            Some(LivepeerEnv::Box) => ("http://localhost:8888", "rtmp://localhost/live"),
            Some(LivepeerEnv::Dev) => ("http://localhost:3004", "rtmp://127.0.0.1:1935/live"),
            Some(LivepeerEnv::Stg) => ("https://livepeer.monster", "rtmp://rtmp.livepeer.monster:11935/live"),
            Some(LivepeerEnv::Prod) => ("https://livepeer.com", "rtmp://rtmp.livepeer.com/live"),
            Some(LivepeerEnv::Origin) | Some(LivepeerEnv::Test) => ("https://origin.livepeer.com", "rtmp://prg-playback.lp-playback.studio/live"),
            None => ("https://livepeer.monster", "rtmp://rtmp.livepeer.monster:11935/live"),
        };

        let config = LivepeerConfig {
            host,
            api_token,
            rtmp_endpoint,
        };
        LivepeerClient { config }
    }
}

impl Livepeer {
    /// Create a new Livepeer
    /// # Arguments
    /// * `api_token` - User API token
    /// * `env` - Livepeer Environment
    /// # Example
    pub fn new(api_token: Option<String>, env: Option<LivepeerEnv>) -> Result<Livepeer, String> {
        let _api_token = api_token.unwrap_or_else(|| std::env::var("LIVEPEER_API_TOKEN").unwrap_or_default());
        let client = LivepeerClient::new(_api_token.clone(), env.clone());

        let user_info = user::User::new(&client).map_err(|e| e.to_string())?;

        Ok(Livepeer {
            _client: client.clone(),
            _env: env.unwrap_or(LivepeerEnv::Dev),
            asset: vod::api::VodApi::new(&client),
            task: vod::task::TaskApi::new(&client),
            user_api: user::UserApi::new(&client),
            access_control: accesscontrol::api::AccessControlApi::new(&client),
            stream: live::stream::Stream::new(&client),
            rtmp: live::rtmp::Rtmp { client: client.clone() },
            user: user_info,
            playback: playback::api::PlaybackApi::new(&client),
            generate: ai::api::GenerateApi::new(&client),
        })
    }
}
