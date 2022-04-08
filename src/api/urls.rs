#[derive(Debug, Clone, Copy)]
pub struct LivepeerUrls {
    pub vod: VodUrls,
    pub task: TaskUrls,
    pub auth: AuthUrls,
}

#[derive(Debug, Clone, Copy)]
pub struct VodUrls {
    pub list_assets: &'static str,
    pub get_presigned_url: &'static str,
    pub export: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct TaskUrls {
    pub list_tasks: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct AuthUrls {
    pub login: &'static str,
    pub info: &'static str
}

impl LivepeerUrls {
    pub fn new() -> Self {
        let vod = VodUrls {
            list_assets: "/api/asset",
            get_presigned_url: "/api/asset/request-upload",
            export: "/export",
        };

        let task = TaskUrls {
            list_tasks: "/api/task",
        };

        let auth = AuthUrls {
            login: "/api/user/token",
            info: "api/user/me",
        };

        let urls = LivepeerUrls { vod, task, auth };
        urls
    }
}
