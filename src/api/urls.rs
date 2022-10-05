#[derive(Debug, Clone, Copy)]
pub struct LivepeerUrls {
    pub vod: VodUrls,
    pub task: TaskUrls,
    pub auth: AuthUrls,
    pub access_control: AccessControlUrls,
}

#[derive(Debug, Clone, Copy)]
pub struct VodUrls {
    pub assets: &'static str,
    pub import_asset: &'static str,
    pub get_presigned_url: &'static str,
    pub export: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct AccessControlUrls {
    pub signing_key: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct TaskUrls {
    pub list_tasks: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct AuthUrls {
    pub login: &'static str,
    pub info: &'static str,
}

impl LivepeerUrls {
    pub fn new() -> Self {
        let vod = VodUrls {
            assets: "/api/asset",
            import_asset: "/api/asset/import",
            get_presigned_url: "/api/asset/request-upload",
            export: "/api/asset/{{ASSET_ID}}/export",
        };

        let task = TaskUrls {
            list_tasks: "/api/task",
        };

        let auth = AuthUrls {
            login: "/api/user/token",
            info: "api/user/me",
        };

        let access_control = AccessControlUrls {
            signing_key: "/api/access-control/signing-key",
        };

        let urls = LivepeerUrls { vod, task, auth, access_control };
        urls
    }
}
