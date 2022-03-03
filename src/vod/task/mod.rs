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
}
