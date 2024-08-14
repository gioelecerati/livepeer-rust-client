use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct User {
    pub client: crate::LivepeerClient,
    pub user_id: String,
    pub info: UserInfo,
}

#[derive(Debug, Clone)]
pub struct UserApi {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

pub trait UserTrait {
    /// Get user information by user ID
    ///
    /// # Parameters
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// * `Result<serde_json::Value, String>` - A JSON value containing the user information or an error message
    fn get_user_info_by_id(&self, user_id: String) -> Result<serde_json::Value, String>;
}

impl UserTrait for UserApi {
    /// Get user information by user ID
    ///
    /// # Parameters
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// * `Result<serde_json::Value, String>` - A JSON value containing the user information or an error message
    fn get_user_info_by_id(&self, user_id: String) -> Result<serde_json::Value, String> {
        self.clone()._get_user_info_by_id(user_id)
    }
}

impl UserApi {
    /// Create a new UserApi instance
    ///
    /// # Parameters
    /// * `client` - A reference to the LivepeerClient
    ///
    /// # Returns
    /// * `Self` - A new instance of UserApi
    pub fn new(client: &crate::LivepeerClient) -> Self {
        UserApi {
            client: client.clone(),
            urls: crate::api::urls::LivepeerUrls::new(),
        }
    }

    /// Internal method to get user information by user ID
    ///
    /// # Parameters
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// * `Result<serde_json::Value, String>` - A JSON value containing the user information or an error message
    pub fn _get_user_info_by_id(&self, user_id: String) -> Result<serde_json::Value, String> {
        match crate::utils::ReqwestRequest::get(
            format!(
                "{}{}",
                self.client.config.host,
                format!("/api/user/{}", user_id)
            ),
            self.client.clone(),
        ) {
            Ok(response) => Ok(response),
            Err(_) => Err("Error getting user info".to_string()),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub kind: String,
    pub admin: bool,
    pub email: String,
    pub last_name: Option<String>,
    pub last_seen: i64,
    pub created_at: i64,
    pub first_name: String,
    pub email_valid: bool,
    pub last_streamed_at: Option<i64>,
    pub stripe_product_id: String,
    pub stripe_customer_id: String,
    pub stripe_customer_subscription_id: String,
}

impl User {
    /// Create a new User instance
    ///
    /// # Parameters
    /// * `client` - A reference to the LivepeerClient
    ///
    /// # Returns
    /// * `Result<Self, String>` - A new instance of User or an error message
    pub fn new(client: &crate::LivepeerClient) -> Result<Self, String> {
        match get_user_info(&client) {
            Ok(user_info) => {
                let user_id = user_info.clone().id;
                Ok(User {
                    client: client.clone(),
                    user_id,
                    info: user_info,
                })
            }
            Err(err) => Err(err),
        }
    }
}

/// Get the user information from the API
///
/// # Parameters
/// * `client` - A reference to the LivepeerClient
///
/// # Returns
/// * `Result<UserInfo, String>` - A UserInfo struct containing the user information or an error message
pub fn get_user_info(client: &crate::LivepeerClient) -> Result<UserInfo, String> {
    match crate::utils::ReqwestRequest::get(
        format!("{}{}", client.config.host, "/api/user/me"),
        client.clone(),
    ) {
        Ok(response) => Ok(serde_json::from_value(response).unwrap()),
        Err(_) => Err("Error getting user id".to_string()),
    }
}

/// Get user information by user ID
///
/// # Parameters
/// * `client` - A reference to the LivepeerClient
/// * `user_id` - The ID of the user
///
/// # Returns
/// * `Result<UserInfo, String>` - A UserInfo struct containing the user information or an error message
pub fn get_user_info_by_id(
    client: &crate::LivepeerClient,
    user_id: String,
) -> Result<UserInfo, String> {
    match crate::utils::ReqwestRequest::get(
        format!("{}{}", client.config.host, format!("/api/user/{}", user_id)),
        client.clone(),
    ) {
        Ok(response) => Ok(serde_json::from_value(response).unwrap()),
        Err(_) => Err("Error getting user id".to_string()),
    }
}
