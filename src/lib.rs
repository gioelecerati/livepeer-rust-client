pub mod api;
pub mod data;
pub mod errors;
pub mod live;
pub mod tests;
pub mod user;
pub mod utils;
pub mod vod;
pub mod accesscontrol;

#[derive(Debug, Clone)]
pub enum LivepeerEnv {
    Dev,
    Stg,
    Prod,
    Origin,
    Sin,
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
    /// Rtmp push utils
    pub rtmp: live::rtmp::Rtmp,
    /// Stream API set
    pub stream: live::stream::Stream,
    /// User API set
    pub user: user::User,
}

impl LivepeerClient {
    /// Create a new Livepeer Client
    /// # Arguments
    /// * `api_token` - User API token
    /// * `env` - Livepeer Environment
    fn new(api_token: String, env: Option<LivepeerEnv>) -> Self {
        let host = match env {
            Some(LivepeerEnv::Dev) => "http://localhost:3004",
            Some(LivepeerEnv::Stg) => "https://livepeer.monster",
            Some(LivepeerEnv::Prod) => "https://livepeer.com",
            Some(LivepeerEnv::Origin) => "https://origin.livepeer.com",
            Some(LivepeerEnv::Sin) => "https://origin.livepeer.com",
            None => "https://livepeer.monster",
        };

        let rtmp_endpoint = match env {
            Some(LivepeerEnv::Dev) => "rtmp://127.0.0.1:1935/live",
            Some(LivepeerEnv::Stg) => "rtmp://rtmp.livepeer.monster:11935/live",
            Some(LivepeerEnv::Prod) => "rtmp://rtmp.livepeer.com/live",
            Some(LivepeerEnv::Origin) => "rtmp://rtmp.livepeer.com/live",
            Some(LivepeerEnv::Sin) => "rtmp://sin-playback.lp-playback.studio/live",
            None => "rtmp://rtmp.livepeer.monster:11935/live",
        };

        let config = LivepeerConfig {
            host: host,
            api_token: api_token,
            rtmp_endpoint: rtmp_endpoint,
        };
        LivepeerClient { config: config }
    }
}

impl Livepeer {
    /// Create a new Livepeer
    /// # Arguments
    /// * `api_token` - User API token
    /// * `env` - Livepeer Environment
    /// # Example
    pub fn new(api_token: Option<String>, env: Option<LivepeerEnv>) -> Result<Livepeer, String> {
        let mut _api_token = String::new();
        if api_token.is_some() {
            _api_token = api_token.unwrap();
        } else {
            // Get API token from environment
            _api_token = std::env::var("LIVEPEER_API_TOKEN").unwrap_or_default();
        }
        let client = LivepeerClient::new(_api_token.clone(), env.clone());

        let user_info = user::User::new(&client);

        if user_info.is_err() {
            return Err(user_info.err().unwrap());
        }

        Ok(Livepeer {
            _client: client.clone(),
            _env: env.clone().unwrap_or(LivepeerEnv::Dev),
            asset: vod::api::VodApi::new(&client),
            task: vod::task::TaskApi::new(&client),
            access_control: accesscontrol::api::AccessControlApi::new(&client),
            stream: live::stream::Stream::new(&client),
            rtmp: live::rtmp::Rtmp {
                client: client.clone(),
            },
            user: user_info.unwrap(),
        })
    }
}
