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
/// Right now there's not a direct way to retrieve the user_id using your api_token
/// so this is a workaround to get the user_id
/// getting the stream list and parsing the first stream's user_id
/// is a hacky way to get the user_id
/// it's not perfect but it works for now
/// if no streams are present, a new one is created
/// 
/// TODO: find a better way to get the user_id
pub fn get_user_id(client: &crate::LivepeerClient) -> String {
    let mut _user_id = String::new();
    let streams = crate::live::stream::Stream::new(&client)
        .list_streams()
        .unwrap();
    let stream_list = streams.as_array().unwrap();

    if stream_list.len() == 0 {
        let body =
            serde_json::to_string(&serde_json::json!({ "name": "user_id_temp_fix" })).unwrap();
        let response: Result<serde_json::Value, crate::errors::Error> =
            crate::utils::SurfRequest::post(
                format!("{}{}", client.config.host, "/api/stream"),
                body,
                client.clone(),
            );

        if let Ok(_r) = response {
            _user_id = _r["userId"].as_str().unwrap().to_string();
        } else {
            panic!("Error creating stream");
        }
    } else {
        let stream = stream_list[0].clone();
        _user_id = stream["userId"].as_str().unwrap().to_string();
    }

    _user_id
}
