pub mod api;
pub mod task;

pub trait Vod {
    fn list_assets(&self) -> Result<serde_json::Value, crate::errors::Error>;
    fn get_presigned_url(
        &self,
        video_name: String,
    ) -> Result<serde_json::Value, crate::errors::Error>;
    fn upload_asset(
        &self,
        video_name: String,
        file_path: String,
    ) -> Result<(), crate::errors::Error>;
    fn get_asset_by_id(&self, asset_id: String) -> Result<serde_json::Value, crate::errors::Error>;
    fn get_assets_by_user_id(&self, user_id: String) -> Result<serde_json::Value, crate::errors::Error>;
    fn update_asset(
        &self,
        asset_id: String,
        payload: serde_json::Value
    ) -> Result<serde_json::Value, crate::errors::Error>;
    fn import_asset(
        &self,
        url: String,
        name: String,
    ) -> Result<serde_json::Value, crate::errors::Error>;
    fn export_to_ipfs(
        &self,
        asset_id: String,
        nft_metadata: String,
    ) -> Result<serde_json::Value, crate::errors::Error>;
}

pub trait Task {
    fn list_tasks(&self) -> Result<serde_json::Value, crate::errors::Error>;
    fn get_task_by_output_asset_id(
        &self,
        output_asset_id: String,
    ) -> Result<serde_json::Value, crate::errors::Error>;
    fn get_tasks_by_user_id(
        &self,
        user_id: String,
    ) -> Result<serde_json::Value, crate::errors::Error>;
    fn get_task_by_id(&self, task_id: String) -> Result<serde_json::Value, crate::errors::Error>;
}
