use crate::errors;
use serde_json;

#[derive(Debug, Clone)]
pub struct TaskApi {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

impl crate::vod::Task for TaskApi {
    /// List all tasks
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the list of tasks or an error
    fn list_tasks(&self) -> Result<serde_json::Value, errors::Error> {
        self.clone()._list_tasks()
    }

    /// Get task by output asset ID
    ///
    /// # Parameters
    /// * `asset_id` - The ID of the output asset
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the task or an error
    fn get_task_by_output_asset_id(
        &self,
        asset_id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        self.clone()._get_task_by_output_asset_id(asset_id)
    }

    /// Get tasks by user ID
    ///
    /// # Parameters
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the tasks or an error
    fn get_tasks_by_user_id(&self, user_id: String) -> Result<serde_json::Value, errors::Error> {
        self.clone()._get_tasks_by_user_id(user_id)
    }

    /// Get task by ID
    ///
    /// # Parameters
    /// * `task_id` - The ID of the task
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the task or an error
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

    /// Make a request to the given URL
    ///
    /// # Parameters
    /// * `url` - The URL to make the request to
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the response or an error
    fn make_request(&self, url: String) -> Result<serde_json::Value, errors::Error> {
        crate::utils::ReqwestRequest::get(url, self.client.clone())
    }

    /// List all tasks
    /// <https://docs.livepeer.com/api/vod/tasks.html#list-all-tasks>
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the list of tasks or an error
    pub fn _list_tasks(self: Self) -> Result<serde_json::Value, errors::Error> {
        self.make_request(format!("{}{}", self.client.config.host, self.urls.task.list_tasks))
    }

    /// Get task by ID
    /// <https://docs.livepeer.com/api/vod/tasks.html#get-task-by-id>
    ///
    /// # Parameters
    /// * `task_id` - The ID of the task
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the task or an error
    pub fn _get_task_by_id(
        self: Self,
        task_id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        self.make_request(format!(
            "{}{}/{}",
            self.client.config.host, self.urls.task.list_tasks, task_id
        ))
    }

    /// Get task by output asset ID
    ///
    /// # Parameters
    /// * `asset_id` - The ID of the output asset
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the task or an error
    pub fn _get_task_by_output_asset_id(
        self: Self,
        asset_id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        self.make_request(format!(
            r#"{}{}?all=true&allUsers=true&filters=[{{"id":"outputAssetId","value":"{}"}}]"#,
            self.client.config.host, self.urls.task.list_tasks, asset_id
        ))
    }

    /// Get tasks by user ID
    ///
    /// # Parameters
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// * `Result<serde_json::Value, errors::Error>` - A JSON value containing the tasks or an error
    pub fn _get_tasks_by_user_id(
        self: Self,
        user_id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        self.make_request(format!(
            r#"{}{}?all=true&allUsers=true&filters=[{{"id":"userId","value":"{}"}}]"#,
            self.client.config.host, self.urls.task.list_tasks, user_id
        ))
    }

    /// Get the status of a task
    ///
    /// # Parameters
    /// * `task_id` - The ID of the task
    ///
    /// # Returns
    /// * `Result<String, errors::Error>` - A string containing the status of the task or an error
    pub fn get_task_status(self: Self, task_id: String) -> Result<String, errors::Error> {
        let task = self._get_task_by_id(task_id)?;
        let task_status = task["status"]["phase"].as_str().unwrap();
        Ok(task_status.to_string())
    }

    /// Wait for a task to complete
    ///
    /// # Parameters
    /// * `task_id` - The ID of the task
    ///
    /// # Returns
    /// * `bool` - `true` if the task completed successfully, `false` otherwise
    pub fn wait_for_task(self: Self, task_id: String) -> bool {
        let mut task_status = String::from("running");

        while task_status == "running" {
            task_status = self.clone().get_task_status(task_id.to_string()).unwrap();
            // sleep 1s
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        task_status == "completed"
    }
}
