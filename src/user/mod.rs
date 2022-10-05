use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct User {
    pub client: crate::LivepeerClient,
    pub user_id: String,
    pub info: UserInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub kind: String,
    pub admin: bool,
    pub email: String,
    pub last_name: String,
    pub last_seen: i64,
    pub created_at: i64,
    pub first_name: String,
    pub email_valid: bool,
    pub last_streamed_at: i64,
    pub stripe_product_id: String,
    pub stripe_customer_id: String,
    pub stripe_customer_subscription_id: String,
}


impl User {
    pub fn new(client: &crate::LivepeerClient) -> Result<Self, String> {
        let user_info = get_user_info(&client);
        if user_info.is_err() {
            return Err(user_info.err().unwrap());
        }

        let usi = user_info.unwrap();

        let user_id = usi.clone().id;
        Ok(User {
            client: client.clone(),
            user_id: user_id,
            info: usi,
        })
    }
}

/// Get the user id from the API
pub fn get_user_info(client: &crate::LivepeerClient) -> Result<UserInfo, String> {

    let response: Result<serde_json::Value, crate::errors::Error> = crate::utils::SurfRequest::get(
        format!("{}{}", client.config.host, "/api/user/me"),
        client.clone(),
    );

    if let Ok(_r) = response {
        return Ok(serde_json::from_value(_r).unwrap());
    } else {
        return Err("Error getting user id".to_string());
    }
}
