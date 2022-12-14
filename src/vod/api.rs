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
        self.clone()._get_assets()
    }

    fn get_presigned_url(&self, video_name: String) -> Result<serde_json::Value, errors::Error> {
        self.clone()._get_presigned_url(video_name)
    }

    fn upload_asset(
        &self,
        presigned_url: String,
        video_file_path: String,
    ) -> Result<(), errors::Error> {
        self.clone()._upload_file(presigned_url, video_file_path)
    }

    fn get_asset_by_id(&self, asset_id: String) -> Result<serde_json::Value, errors::Error> {
        self.clone()._get_asset_by_id(asset_id)
    }

    fn get_assets_by_user_id(&self, user_id: String) -> Result<serde_json::Value, errors::Error> {
        self.clone()._get_assets_by_user_id(user_id)
    }

    fn import_asset(
        &self,
        video_file_path: String,
        video_name: String,
    ) -> Result<serde_json::Value, errors::Error> {
        self.clone()._import_asset(video_file_path, video_name)
    }

    fn update_asset(
        &self,
        asset_id: String,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, errors::Error> {
        self.clone()._update_asset(asset_id, payload)
    }

    fn export_to_ipfs(
        &self,
        asset_id: String,
        nft_metadata: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let json_nft_metadata = serde_json::from_str(&nft_metadata).unwrap();
        self.clone()._export_to_ipfs(asset_id, json_nft_metadata)
    }
}

impl VodApi {
    pub fn new(client: &crate::LivepeerClient) -> Self {
        VodApi {
            client: client.clone(),
            urls: crate::api::urls::LivepeerUrls::new(),
        }
    }

    /// List all assets
    /// <https://livepeer.com/docs/api-reference/vod/list>
    ///
    pub fn _get_assets(self: Self) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!("{}{}", self.client.config.host, self.urls.vod.assets),
            self.client,
        );
        res
    }

    /// Get asset by id
    /// <https://livepeer.com/docs/api-reference/vod/list#retrieve-an-asset>
    ///
    pub fn _get_asset_by_id(
        self: Self,
        asset_id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!(
                "{}{}/{}",
                self.client.config.host, self.urls.vod.assets, asset_id
            ),
            self.client,
        );
        res
    }

    /// Get assets by user id
    pub fn _get_assets_by_user_id(
        self: Self,
        user_id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!(
                r#"{}{}?all=true&allUsers=true&filters=[{{"id":"userId","value":"{}"}}]"#,
                self.client.config.host, self.urls.vod.assets, user_id
            ),
            self.client,
        );
        res
    }

    /// Import asset
    /// <https://livepeer.com/docs/api-reference/vod/list#import-an-asset>
    ///
    pub fn _import_asset(
        self: Self,
        url: String,
        name: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::post(
            format!("{}{}", self.client.config.host, self.urls.vod.import_asset),
            serde_json::json!({
                "url": url,
                "name": name
            })
            .to_string(),
            self.client,
        );
        res
    }

    /// Get presigned url
    /// <https://livepeer.com/docs/api-reference/vod/upload>
    ///
    pub fn _get_presigned_url(
        self: Self,
        video_name: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let body = serde_json::to_string(&serde_json::json!({ "name": video_name })).unwrap();
        let response: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::post(
            format!(
                "{}{}",
                self.client.config.host, self.urls.vod.get_presigned_url
            ),
            body,
            self.client,
        );

        response
    }

    /// Upload asset
    /// <https://livepeer.com/docs/api-reference/vod/upload>
    ///
    pub fn _upload_file(
        self: Self,
        presigned_url: String,
        video_file_path: String,
    ) -> Result<(), errors::Error> {
        let mut res: Result<(), errors::Error> = Err(errors::Error::UNKNOWN);
        let video_buffer = std::fs::read(video_file_path).unwrap();

        async_std::task::block_on(async {
            let response = surf::put(&presigned_url)
                .header("Content-Type", "video/mp4")
                .body(video_buffer)
                .await;

            match response {
                Ok(response) => match response.status() {
                    surf::StatusCode::Ok => {
                        res = Ok(());
                    }
                    _ => {
                        let err = errors::Error::from_response(&response);
                        res = Err(err);
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        });
        return res;
    }

    /// Update Asset
    /// <https://livepeer.com/docs/api-reference/vod/list#update-an-asset>
    ///
    pub fn _update_asset(
        self: Self,
        asset_id: String,
        payload: serde_json::Value
    ) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::patch(
            format!(
                "{}{}/{}",
                self.client.config.host, self.urls.vod.assets, asset_id
            ),
            serde_json::json!(payload)
            .to_string(),
            self.client,
        );
        res
    }

    /// Export asset to IPFS
    /// <https://livepeer.com/docs/api-reference/vod/export>
    ///
    pub fn _export_to_ipfs(
        self: Self,
        asset_id: String,
        nft_metadata: serde_json::Value,
    ) -> Result<serde_json::Value, errors::Error> {
        let body = serde_json::to_string(&serde_json::json!({ "ipfs": nft_metadata })).unwrap();
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::post(
            format!(
                "{}/api/asset/{}/{}",
                self.client.config.host, asset_id, "export"
            ),
            body,
            self.client,
        );

        res
    }
}
