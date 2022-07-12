# livepeer-rs

`0.0.1`

Crate to interact with the `Livepeer Studio` API

## Documentation

```
$ cargo doc
$ cd ./target/doc
$ python3 -m http.server 8080
```   
   
http://localhost:8080/livepeer_client

## Example usage

```rust
use livepeer_rs::{vod::Vod, Livepeer, LivepeerEnv};

fn main() {
    let _env = LivepeerEnv::Prod;

    // Set up your Client
    let lp_client = Livepeer::new(String::from("$YOUR_API_TOKEN"), Some(_env));

    // Retrieve JSON of VOD assets
    match lp_client.asset.list_assets() {
        Ok(assets) => {
            println!("Assets: {}", serde_json::to_string(&assets).unwrap());
        }
        Err(err) => {
            println!("Error retrieving VOD assets: {:?}", err);
        }
    };

    // Retrieve JSON of Livepeer Streams
    match lp_client.stream.list_streams() {
        Ok(streams) => {
            println!("Streams: {}", serde_json::to_string(&streams).unwrap());
        }
        Err(err) => {
            println!("Error retrieving streams: {:?}", err);
        }
    };
}
```

## Available methods

### On Demand

- [X] List VOD assets   
- [X] Get VOD asset   
- [X] Update VOD asset
- [X] List VOD tasks   
- [X] Direct Upload   
- [X] Import from external URL   
- [X] Generate presigned URL   

### Live

- [X] List streams   
- [X] Get stream
- [X] Push RTMP (linux only - ffmpeg dep)   


