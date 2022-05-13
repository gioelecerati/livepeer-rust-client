// Tests


#[cfg(test)]
mod tests {
    use crate::*;
    use crate::vod::Vod;
    use crate::vod::Task;

    use std::any::type_name;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
    
    #[test]
    fn test_client() {
        // Create a new Livepeer Client
        let _api_token = std::env::var("LIVEPEER_API_TOKEN").unwrap_or_default();
        let lp = Livepeer::new(None, Some(LivepeerEnv::Stg));
        assert_eq!(lp._client.config.api_token, _api_token);

        // List assets
        let assets = lp.asset.list_assets().unwrap();
        assert_eq!(type_of(assets.clone()), "serde_json::value::Value");

        // Get asset by id
        if assets.clone().as_array().unwrap().len() > 0 {
            let assets_array = assets.as_array().unwrap();
            let asset = &assets_array[0];
            let asset_id = asset["id"].as_str().unwrap();
            let retrieved_asset = lp.asset.get_asset_by_id(asset_id.to_string()).unwrap();
            println!("RETRIEVED ASSET : {}",retrieved_asset);
            assert_eq!(asset["id"], retrieved_asset["id"]);
        }

        // List tasks
        let tasks = lp.task.list_tasks().unwrap();
        assert_eq!(type_of(tasks.clone()), "serde_json::value::Value");
        
        // Get task by id
        if tasks.clone().as_array().unwrap().len() > 0 {
            let tasks_array = tasks.as_array().unwrap();
            let task = &tasks_array[0];
            let task_id = task["id"].as_str().unwrap();
            let retrieved_task = lp.task._get_task_by_id(task_id.to_string()).unwrap();
            println!(" RETRIEVED TASK : {}",retrieved_task);
            assert_eq!(task["id"], retrieved_task["id"]);
        }

        // List streams
        let lp_stream = lp.stream.clone();
        let streams = lp.stream.list_streams().unwrap();
        println!("STREAMS : {}",streams);
        assert_eq!(type_of(streams.clone()), "serde_json::value::Value");
        

        // Get stream by id
        if streams.clone().as_array().unwrap().len() > 0 {
            let streams_array = streams.as_array().unwrap();
            let stream = &streams_array[0];
            let stream_id = stream["id"].as_str().unwrap();
            let retrieved_stream = lp_stream.get_stream_by_id(stream_id.to_string()).unwrap();
            println!("RETRIEVED STREAM : {}",retrieved_stream);
            assert_eq!(stream["id"], retrieved_stream["id"]);
        }

        // Get presigned url
        let presigned_url = lp.asset.get_presigned_url(String::from("test_asset_name")).unwrap();
        println!("PRESIGNED URL : {}",presigned_url);
        assert_eq!(type_of(presigned_url.clone()), "serde_json::value::Value");

        // Upload asset
        let upload_url = presigned_url["url"].to_string().replace("\"", "");
        let upload_result = lp.asset.upload_asset(upload_url, String::from("/home/gioele/Downloads/jellyfish.mp4")).unwrap();
        assert_eq!(type_of(upload_result.clone()), "()");

        // Export to IPFS
        let export_result = lp.asset.export_to_ipfs(String::from("test_asset_name"), String::from("{}")).unwrap();
        println!("{}",export_result);
        assert_eq!(type_of(export_result.clone()), "serde_json::value::Value");
        
    }
}