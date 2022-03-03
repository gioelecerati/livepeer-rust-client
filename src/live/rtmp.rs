#[derive(Debug, Clone)]
pub struct Rtmp {
    pub client: crate::LivepeerClient,
}

/// Live RTMP Methods
/// 
impl Rtmp {

    /// Temp ffmpeg command spawn. TODO: Replace with a proper library.
    pub fn push(self: Self, stream_key: &String, file_path: &String) {
        // spawn ffmpeg command
        let mut cmd = std::process::Command::new("ffmpeg");
        cmd.arg("-re")
            .arg("-i")
            .arg(file_path)
            .arg("-c:v")
            .arg("copy")
            .arg("-c:a")
            .arg("copy")
            .arg("-f")
            .arg("flv")
            .arg(format!(
                "{}/{}",
                self.client.config.rtmp_endpoint, stream_key
            ));
        println!("{:?}", cmd);
        let output = cmd.output().expect("failed to execute process");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    }

    /// Temp ffmpeg command spawn. TODO: Replace with a proper library.
    pub fn push_to_region(self: &Self, stream_key: &String, file_path: &String, region: &String) {
        let mut _region_url = String::new();

        if region != "none" {
            _region_url = self
                .client
                .config
                .rtmp_endpoint
                .replace("rtmp://", &format!("rtmp://{}-", region));
        } else {
            _region_url = self.client.config.rtmp_endpoint.to_string();
        }

        // spawn ffmpeg command
        let mut cmd = std::process::Command::new("ffmpeg");
        cmd.arg("-re")
            .arg("-i")
            .arg(file_path)
            .arg("-c:v")
            .arg("copy")
            .arg("-c:a")
            .arg("copy")
            .arg("-f")
            .arg("flv")
            .arg(format!("{}/{}", _region_url, stream_key));
        println!("{:?}", cmd);
        let output = cmd.output().expect("failed to execute process");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    }
}
