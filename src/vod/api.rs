use crate::errors;
use async_std;
use serde_json;

#[derive(Debug, Clone)]
pub struct VodApi {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

impl crate::vod::Vod for VodApi {
    fn list_assets(&self) -> Result<serde_json::Value, errors::Error> {
        self._get_assets()
    }

    fn list_paginated_assets(&self, limit: usize, start: usize, details: bool) -> Result<serde_json::Value, errors::Error> {
        self._get_paginated_assets(limit, start, details)
    }

    fn get_presigned_url(&self, video_name: String, playback_policy: Option<serde_json::Value>) -> Result<serde_json::Value, errors::Error> {
        self._get_presigned_url(video_name, playback_policy)
    }

    fn upload_asset(&self, presigned_url: String, video_file_path: String) -> Result<(), errors::Error> {
        self._upload_file(presigned_url, video_file_path)
    }

    fn get_asset_by_id(&self, asset_id: String) -> Result<serde_json::Value, errors::Error> {
        self._get_asset_by_id(asset_id)
    }

    fn get_asset_by_playback_id(&self, playback_id: String, admin: bool) -> Result<serde_json::Value, errors::Error> {
        self._get_asset_by_playback_id(playback_id, admin)
    }

    fn get_assets_by_cid(&self, cid: String, admin: bool) -> Result<serde_json::Value, errors::Error> {
        self._get_assets_by_cid(cid, admin)
    }

    fn get_assets_by_user_id(&self, user_id: String) -> Result<serde_json::Value, errors::Error> {
        self._get_assets_by_user_id(user_id)
    }

    fn import_asset(&self, video_file_path: String, video_name: String, playback_policy: Option<serde_json::Value>) -> Result<serde_json::Value, errors::Error> {
        self._import_asset(video_file_path, video_name, playback_policy)
    }

    fn update_asset(&self, asset_id: String, payload: serde_json::Value) -> Result<serde_json::Value, errors::Error> {
        self._update_asset(asset_id, payload)
    }

    fn export_to_ipfs(&self, asset_id: String, nft_metadata: String) -> Result<serde_json::Value, errors::Error> {
        let json_nft_metadata = serde_json::from_str(&nft_metadata).unwrap();
        self._export_to_ipfs(asset_id, json_nft_metadata)
    }

    fn list_webhooks(&self) -> Result<serde_json::Value, crate::errors::Error> {
        self._get_webhooks()
    }
}

impl VodApi {
    pub fn new(client: &crate::LivepeerClient) -> Self {
        VodApi {
            client: client.clone(),
            urls: crate::api::urls::LivepeerUrls::new(),
        }
    }

    pub fn _get_assets(&self) -> Result<serde_json::Value, errors::Error> {
        crate::utils::SurfRequest::get(
            format!("{}{}", self.client.config.host, self.urls.vod.assets),
            self.client.clone(),
        )
    }

    pub fn _get_paginated_assets(&self, limit: usize, start: usize, details: bool) -> Result<serde_json::Value, errors::Error> {
        let dtls = if details { 1 } else { 0 };
        crate::utils::SurfRequest::get(
            format!("{}{}?limit={}&order=createdAt-true&cursor=skip{}&details={}", self.client.config.host, self.urls.vod.assets, limit, start, dtls),
            self.client.clone(),
        )
    }

    pub fn _get_asset_by_id(&self, asset_id: String) -> Result<serde_json::Value, errors::Error> {
        crate::utils::SurfRequest::get(
            format!("{}{}/{}", self.client.config.host, self.urls.vod.assets, asset_id),
            self.client.clone(),
        )
    }

    pub fn _get_asset_by_playback_id(&self, playback_id: String, admin: bool) -> Result<serde_json::Value, errors::Error> {
        let admin_string = if admin { "&allUsers=true&all=true" } else { "" };
        crate::utils::SurfRequest::get(
            format!("{}{}?playbackId={}{}", self.client.config.host, self.urls.vod.assets, playback_id, admin_string),
            self.client.clone(),
        )
    }

    pub fn _get_assets_by_cid(&self, cid: String, admin: bool) -> Result<serde_json::Value, errors::Error> {
        let admin_string = if admin { "&allUsers=true&all=true" } else { "" };
        crate::utils::SurfRequest::get(
            format!("{}{}?cid={}{}", self.client.config.host, self.urls.vod.assets, cid, admin_string),
            self.client.clone(),
        )
    }

    pub fn _get_assets_by_user_id(&self, user_id: String) -> Result<serde_json::Value, errors::Error> {
        crate::utils::SurfRequest::get(
            format!(r#"{}{}?all=true&allUsers=true&filters=[{{"id":"userId","value":"{}"}}]"#, self.client.config.host, self.urls.vod.assets, user_id),
            self.client.clone(),
        )
    }

    pub fn _import_asset(&self, url: String, name: String, playback_policy: Option<serde_json::Value>) -> Result<serde_json::Value, errors::Error> {
        let body = if let Some(policy) = playback_policy {
            serde_json::json!({ "url": url, "name": name, "playbackPolicy": policy }).to_string()
        } else {
            serde_json::json!({ "url": url, "name": name }).to_string()
        };
        crate::utils::SurfRequest::post(
            format!("{}{}", self.client.config.host, self.urls.vod.import_asset),
            body,
            self.client.clone(),
        )
    }

    pub fn _get_presigned_url(&self, video_name: String, playback_policy: Option<serde_json::Value>) -> Result<serde_json::Value, errors::Error> {
        let body = serde_json::to_string(&serde_json::json!({ "name": video_name, "playbackPolicy": playback_policy })).unwrap();
        crate::utils::SurfRequest::post(
            format!("{}{}", self.client.config.host, self.urls.vod.get_presigned_url),
            body,
            self.client.clone(),
        )
    }

    pub fn _upload_file(&self, presigned_url: String, video_file_path: String) -> Result<(), errors::Error> {
        let video_buffer = std::fs::read(video_file_path).unwrap();
        let mut res: Result<(), errors::Error> = Err(errors::Error::UNKNOWN);

        async_std::task::block_on(async {
            let response = surf::put(&presigned_url)
                .header("Content-Type", "video/mp4")
                .body(video_buffer)
                .await;

            match response {
                Ok(response) => match response.status() {
                    surf::StatusCode::Ok => res = Ok(()),
                    _ => res = Err(errors::Error::from_response(&response)),
                },
                Err(e) => println!("{:?}", e),
            }
        });

        res
    }

    pub fn _update_asset(&self, asset_id: String, payload: serde_json::Value) -> Result<serde_json::Value, errors::Error> {
        crate::utils::SurfRequest::patch(
            format!("{}{}/{}", self.client.config.host, self.urls.vod.assets, asset_id),
            serde_json::json!(payload).to_string(),
            self.client.clone(),
        )
    }

    pub fn _export_to_ipfs(&self, asset_id: String, nft_metadata: serde_json::Value) -> Result<serde_json::Value, errors::Error> {
        let body = serde_json::to_string(&serde_json::json!({ "ipfs": nft_metadata })).unwrap();
        crate::utils::SurfRequest::post(
            format!("{}/api/asset/{}/{}", self.client.config.host, asset_id, "export"),
            body,
            self.client.clone(),
        )
    }

    pub fn _get_webhooks(&self) -> Result<serde_json::Value, errors::Error> {
        crate::utils::SurfRequest::get(
            format!("{}{}", self.client.config.host, self.urls.vod.list_webhooks),
            self.client.clone(),
        )
    }
}
