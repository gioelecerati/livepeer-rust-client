use crate::errors;
use serde_json;

#[derive(Debug, Clone)]
pub struct TaskApi {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

impl crate::vod::Task for TaskApi {
    fn list_tasks(&self) -> Result<serde_json::Value, errors::Error> {
        self.clone()._list_tasks()
    }

    fn get_task_by_output_asset_id(&self, asset_id: String) -> Result<serde_json::Value, errors::Error> {
        self.clone()._get_task_by_output_asset_id(asset_id)
    }

    fn get_tasks_by_user_id(&self, user_id: String) -> Result<serde_json::Value, errors::Error> {
        self.clone()._get_tasks_by_user_id(user_id)
    }

    fn get_task_by_id(&self, task_id: String) -> Result<serde_json::Value, errors::Error> {
        self.clone()._get_task_by_id(task_id)
    }
}

impl TaskApi {
    pub fn new(client: &crate::LivepeerClient) -> Self {
        TaskApi {
            client: client.clone(),
            urls: crate::api::urls::LivepeerUrls::new(),
        }
    }

    /// List all tasks
    /// <https://docs.livepeer.com/api/vod/tasks.html#list-all-tasks>
    pub fn _list_tasks(self: Self) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!("{}{}", self.client.config.host, self.urls.task.list_tasks),
            self.client,
        );
        res
    }

    /// Get task by id
    /// <https://docs.livepeer.com/api/vod/tasks.html#get-task-by-id>
    pub fn _get_task_by_id(
        self: Self,
        task_id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!(
                "{}{}/{}",
                self.client.config.host, self.urls.task.list_tasks, task_id
            ),
            self.client,
        );
        res
    }

    pub fn _get_task_by_output_asset_id(
        self: Self,
        asset_id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!(
                r#"{}{}?all=true&allUsers=true&filters=[{{"id":"outputAssetId","value":"{}"}}]"#,
                self.client.config.host, self.urls.task.list_tasks, asset_id
            ),
            self.client,
        );
        res
    }

    pub fn _get_tasks_by_user_id(
        self: Self,
        user_id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!(
                r#"{}{}?all=true&allUsers=true&filters=[{{"id":"userId","value":"{}"}}]"#,
                self.client.config.host, self.urls.task.list_tasks, user_id
            ),
            self.client,
        );
        res
    }

    pub fn get_task_status(self: Self, task_id: String) -> Result<String, errors::Error> {
        let task = self._get_task_by_id(task_id)?;
        let task_status = task["status"]["phase"].as_str().unwrap();
        return Ok(task_status.to_string());
    }

    pub fn wait_for_task(self: Self, task_id: String) -> bool {
        let mut task_status = String::from("running");

        while task_status == "running" {
            task_status = self.clone().get_task_status(task_id.to_string()).unwrap();
            // sleep 1s
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        if task_status != "completed" {
            return false;
        } else {
            return true;
        }
    }
}
