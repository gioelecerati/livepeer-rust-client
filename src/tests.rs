// Tests


#[cfg(test)]
mod tests {
    use crate::*;
    use crate::vod::Vod;
    use crate::vod::Task;

    use std::any::type_name;
    use colored::*;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
    
    #[test]
    fn test_client() {
        // Create a new Livepeer Client
        println!("{}", "Creating a new Livepeer Client".blue());
        let _api_token = std::env::var("LIVEPEER_API_TOKEN").unwrap_or_default();
        let lp = Livepeer::new(None, Some(LivepeerEnv::Stg));
        assert_eq!(lp._client.config.api_token, _api_token);
        println!("{}", "Success".green());

        // List assets
        println!("{}", "Listing assets".blue());
        let assets = lp.asset.list_assets().unwrap();
        assert_eq!(type_of(assets.clone()), "serde_json::value::Value");
        println!("{}", "Success".green());

        // Get asset by id
        println!("{}", "Getting asset by id".blue());
        if assets.clone().as_array().unwrap().len() > 0 {
            let assets_array = assets.as_array().unwrap();
            let asset = &assets_array[0];
            let asset_id = asset["id"].as_str().unwrap();
            let retrieved_asset = lp.asset.get_asset_by_id(asset_id.to_string()).unwrap();
            assert_eq!(asset["id"], retrieved_asset["id"]);
        }
        println!("{}", "Success".green());

        // List tasks
        println!("{}", "Listing tasks".blue());
        let tasks = lp.task.list_tasks().unwrap();
        assert_eq!(type_of(tasks.clone()), "serde_json::value::Value");
        println!("{}", "Success".green());
        
        

        // List streams
        println!("{}", "Listing streams".blue());
        let lp_stream = lp.stream.clone();
        let streams = lp.stream.list_streams().unwrap();
        assert_eq!(type_of(streams.clone()), "serde_json::value::Value");
        println!("{}", "Success".green());
        

        // Get stream by id
        println!("{}", "Getting stream by id".blue());
        if streams.clone().as_array().unwrap().len() > 0 {
            let streams_array = streams.as_array().unwrap();
            let stream = &streams_array[0];
            let stream_id = stream["id"].as_str().unwrap();
            let retrieved_stream = lp_stream.get_stream_by_id(stream_id.to_string()).unwrap();
            assert_eq!(stream["id"], retrieved_stream["id"]);
        }
        println!("{}", "Success".green());

        // Import asset
        println!("{}", "Importing asset".blue());
        let imported_asset = lp.asset.import_asset(String::from("https://freetestdata.com/wp-content/uploads/2022/02/Free_Test_Data_15MB_MP4.mp4"), String::from("test_asset_name")).unwrap();
        println!("{}", "Success".green());

        // Get task by id
        println!("{}", "Getting task by id".blue());
        let task_id = imported_asset["task"]["id"].as_str().unwrap();
        let retrieved_task = lp.task.clone()._get_task_by_id(task_id.to_string()).unwrap();
        assert_eq!(imported_asset["task"]["id"], retrieved_task["id"]);
        println!("{}", "Success".green());
        
        println!("{}", "Waiting for Import task to finish...".blue());
        let task_completed = lp.task.clone().wait_for_task(task_id.to_string());

        if !task_completed {
            panic!("The import task has failed");
        }else{
            println!("{}", "Success".green());
        }

        // Export to IPFS
        println!("{}", "Exporting to IPFS".blue());
        let export_result = lp.asset.export_to_ipfs(imported_asset["asset"]["id"].to_string().replace("\"", ""), String::from("{}")).unwrap();
        println!("{}",export_result);
        assert_eq!(type_of(export_result.clone()), "serde_json::value::Value");
        println!("{}", "Success".green());

        let task_id = export_result["task"]["id"].as_str().unwrap();

        // Wait for task to finish
        println!("{}", "Waiting for Export task to finish...".blue());
        let task_completed = lp.task.clone().wait_for_task(task_id.to_string());

        if !task_completed {
            panic!("The export task has failed");
        }else{
            println!("{}", "Success".green());
        }
        
    }
}

