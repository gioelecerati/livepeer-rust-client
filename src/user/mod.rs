#[derive(Debug, Clone)]
pub struct User {
    pub client: crate::LivepeerClient,
    pub user_id: String,
}

impl User {
    pub fn new(client: &crate::LivepeerClient) -> Self {
        let user_id = get_user_id(&client);
        User {
            client: client.clone(),
            user_id: user_id,
        }
    }
}

/// Get the user id from the API
pub fn get_user_id(client: &crate::LivepeerClient) -> String {
    let mut _user_id = String::new();
    let response: Result<serde_json::Value, crate::errors::Error> =
        crate::utils::SurfRequest::get(
            format!("{}{}", client.config.host, "/api/user/me"),
            client.clone(),
        );

    if let Ok(_r) = response {
        _user_id = _r["id"].as_str().unwrap().to_string();
    } else {
        panic!("Error getting user info");
    }
    _user_id
}
