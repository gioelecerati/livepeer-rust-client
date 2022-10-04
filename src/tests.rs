// Tests

#[cfg(test)]
mod tests {
    use crate::vod::Task;
    use crate::vod::Vod;
    use crate::*;

    use colored::*;
    use std::any::type_name;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn test_client() {
        // Create a new Livepeer Client
        println!("{}", "Creating a new Livepeer Client".blue());
        let _api_token = std::env::var("LIVEPEER_API_TOKEN").unwrap_or_default();
        let _lp = Livepeer::new(None, Some(LivepeerEnv::Stg));

        if _lp.is_err() {
            println!("{}", "Error creating Livepeer Client".red());
            assert!(false);
        }

        let lp = _lp.unwrap();
        
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

        // Update asset
        println!("{}", "Updating asset".blue());
        if assets.clone().as_array().unwrap().len() > 0 {
            let assets_array = assets.as_array().unwrap();
            let asset = &assets_array[0];
            let asset_id = asset["id"].as_str().unwrap();
            let retrieved_asset = lp.asset.get_asset_by_id(asset_id.to_string()).unwrap();
            let updated_asset = lp
                .asset
                .update_asset(
                    retrieved_asset["id"].as_str().unwrap().to_string(),
                    "updated_name".to_string(),
                    Some(serde_json::json!({})),
                    Some(serde_json::json!({
                        "ipfs":{}
                    })),
                )
                .unwrap();
            assert_eq!(updated_asset["name"], "updated_name");
        }
        println!("{}", "Success".green());

        // List tasks
        println!("{}", "Listing tasks".blue());
        let tasks = lp.task.list_tasks().unwrap();
        assert_eq!(type_of(tasks.clone()), "serde_json::value::Value");
        println!("{}", "Success".green());

        // Create stream
        println!("{}", "Creating stream".blue());

        // List streams
        println!("{}", "Listing streams".blue());
        let lp_stream = lp.stream.clone();
        let streams = lp.stream.list_streams().unwrap();
        let st = streams.clone();
        assert_eq!(
            type_of(streams.clone()),
            "alloc::vec::Vec<livepeer_rs::data::stream::Stream>"
        );
        println!("{}", "Success".green());

        // Get stream by id
        println!("{}", "Getting stream by id".blue());
        if streams.clone().len() > 0 {
            let streams_array = streams;
            let stream = &streams_array[0];
            let stream_id = &stream.id;
            let retrieved_stream = lp_stream.get_stream_by_id(stream_id.to_string()).unwrap();
            assert_eq!(stream.id, retrieved_stream["id"]);
        }
        println!("{}", "Success".green());

        // Import asset
        println!("{}", "Importing asset".blue());
        let imported_asset = lp.asset.import_asset(String::from("https://freetestdata.com/wp-content/uploads/2022/02/Free_Test_Data_15MB_MP4.mp4"), String::from("test_asset_name")).unwrap();
        println!("{}", "Success".green());

        // Get task by id
        println!("{}", "Getting task by id".blue());
        let task_id = imported_asset["task"]["id"].as_str().unwrap();
        let retrieved_task = lp
            .task
            .clone()
            ._get_task_by_id(task_id.to_string())
            .unwrap();
        assert_eq!(imported_asset["task"]["id"], retrieved_task["id"]);
        println!("{}", "Success".green());

        //println!("{}", "Waiting for Import task to finish...".blue());
        //let task_completed = lp.task.clone().wait_for_task(task_id.to_string());

        /*if !task_completed {
            panic!("The import task has failed");
        } else {
            println!("{}", "Success".green());
        }*/

        let stream_key = st[1].clone().stream_key.unwrap();
        // push to rtmp endpoint
        lp.rtmp.push(
            &stream_key.to_string(),
            &"/home/gioele/Downloads/bbbx3_720_2s.mp4".to_string(),
        )
    }
}
