#[derive(Debug, Clone)]
pub struct User {
    pub client: crate::LivepeerClient,
    pub user_id: String,
}

impl User {
    pub fn new(client: &crate::LivepeerClient) -> Result<Self, String> {
        let user_id = get_user_id(&client);
        if user_id.is_err() {
            return Err(user_id.err().unwrap());
        }
        Ok(User {
            client: client.clone(),
            user_id: user_id.unwrap(),
        })
    }
}

/// Get the user id from the API
pub fn get_user_id(client: &crate::LivepeerClient) -> Result<String, String> {
    let mut _user_id = String::new();
    let response: Result<serde_json::Value, crate::errors::Error> = crate::utils::SurfRequest::get(
        format!("{}{}", client.config.host, "/api/user/me"),
        client.clone(),
    );

    if let Ok(_r) = response {
        _user_id = _r["id"].as_str().unwrap().to_string();
    } else {
        return Err("Error getting user id".to_string());
    }
    Ok(_user_id)
}
