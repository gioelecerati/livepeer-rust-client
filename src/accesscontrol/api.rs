use crate::errors;
use async_std;
use serde_json;

#[derive(Debug, Clone)]
pub struct AccessControlApi {
    pub client: crate::LivepeerClient,
    pub urls: crate::api::urls::LivepeerUrls,
}

impl crate::accesscontrol::AccessControl for AccessControlApi {
    fn list_signing_keys(&self) -> Result<serde_json::Value, errors::Error> {
        self.clone()._get_signing_keys()
    }
    fn create_signing_key(
        &self,
        name: String,
    ) -> Result<serde_json::Value, errors::Error> {
        self.clone()._create_signing_key(name)
    }   
    fn delete_signing_key(
        &self,
        id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        self.clone()._delete_signing_key(id)
    }
}

impl AccessControlApi {
    pub fn new(client: &crate::LivepeerClient) -> Self {
        AccessControlApi {
            client: client.clone(),
            urls: crate::api::urls::LivepeerUrls::new(),
        }
    }

    /// List all Signing keys
    ///
    pub fn _get_signing_keys(self: Self) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::get(
            format!("{}{}", self.client.config.host, self.urls.access_control.signing_key),
            self.client,
        );
        res
    }

    pub fn _create_signing_key(
        self: Self,
        name: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::post(
            format!("{}{}", self.client.config.host, self.urls.access_control.signing_key),
            serde_json::json!({
                "name": name
            })
            .to_string(),
            self.client,
        );
        res
    }

    pub fn _delete_signing_key(
        self: Self,
        id: String,
    ) -> Result<serde_json::Value, errors::Error> {
        let res: Result<serde_json::Value, errors::Error> = crate::utils::SurfRequest::delete(
            format!(
                "{}{}/{}",
                self.client.config.host, self.urls.access_control.signing_key, id
            ),
            self.client,
        );
        res
    }

    
}
