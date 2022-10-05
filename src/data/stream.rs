use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub type Streams = Vec<Stream>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub created_at: i64,
    pub id: String,
    pub ingest_rate: Option<f64>,
    pub is_active: bool,
    pub kind: String,
    pub last_seen: i64,
    pub name: String,
    pub outgoing_rate: Option<f64>,
    pub parent_id: Option<String>,
    pub profiles: Vec<Profile>,
    pub record: bool,
    pub region: Option<String>,
    pub renditions: Renditions,
    pub source_bytes: i64,
    pub source_segments: i64,
    pub source_segments_duration: f64,
    pub suspended: bool,
    pub transcoded_bytes: i64,
    pub transcoded_segments: i64,
    pub transcoded_segments_duration: f64,
    pub user_id: String,
    pub multistream: Option<Multistream>,
    pub playback_id: Option<String>,
    pub stream_key: Option<String>,
    pub created_by_token_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub bitrate: i64,
    pub fps: i64,
    pub height: i64,
    pub name: String,
    pub width: i64,
    pub gop: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Renditions {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Multistream {
    pub targets: Vec<Value>,
}
