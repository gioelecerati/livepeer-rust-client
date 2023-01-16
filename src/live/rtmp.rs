#[derive(Debug, Clone)]
pub struct Rtmp {
    pub client: crate::LivepeerClient,
}

use std::collections::HashMap;
use std::env;
use std::time::Instant;

use ffmpeg_next::{
    codec, decoder, encoder, format, frame, log, media, picture, Dictionary, Packet, Rational,
};

const DEFAULT_X264_OPTS: &str = "preset=medium";

struct Transcoder {
    ost_index: usize,
    decoder: decoder::Video,
    encoder: encoder::video::Video,
    logging_enabled: bool,
    frame_count: usize,
    last_log_frame_count: usize,
    starting_time: Instant,
    last_log_time: Instant,
}

impl Transcoder {
    fn new(
        ist: &format::stream::Stream,
        octx: &mut format::context::Output,
        ost_index: usize,
        x264_opts: Dictionary,
        enable_logging: bool,
    ) -> Result<Self, ffmpeg_next::Error> {
        let global_header = octx.format().flags().contains(format::Flags::GLOBAL_HEADER);
        let decoder = ffmpeg_next::codec::context::Context::from_parameters(ist.parameters())?
            .decoder()
            .video()?;
        let mut ost = octx.add_stream(encoder::find(codec::Id::H264))?;
        let mut encoder = codec::context::Context::from_parameters(ost.parameters())?
            .encoder()
            .video()?;
        let fps = Some(ffmpeg_next::util::rational::Rational::new(30,30));
        let width = decoder.width();
        println!("FPS {:?}", fps);
        println!("Width {:?}", width);
        encoder.set_height(decoder.height());
        encoder.set_width(width);
        encoder.set_aspect_ratio(decoder.aspect_ratio());
        encoder.set_format(decoder.format());
        encoder.set_frame_rate(fps);
        encoder.set_time_base(fps.unwrap().invert());
        if global_header {
            encoder.set_flags(codec::Flags::GLOBAL_HEADER);
        }

        encoder
            .open_with(x264_opts)
            .expect("error opening libx264 encoder with supplied settings");
        encoder = codec::context::Context::from_parameters(ost.parameters())?
            .encoder()
            .video()?;
        ost.set_parameters(&encoder);
        Ok(Self {
            ost_index,
            decoder,
            encoder: codec::context::Context::from_parameters(ost.parameters())?
                .encoder()
                .video()?,
            logging_enabled: enable_logging,
            frame_count: 0,
            last_log_frame_count: 0,
            starting_time: Instant::now(),
            last_log_time: Instant::now(),
        })
    }

    fn send_packet_to_decoder(&mut self, packet: &Packet) {
        self.decoder.send_packet(packet).unwrap();
    }

    fn send_eof_to_decoder(&mut self) {
        self.decoder.send_eof().unwrap();
    }

    fn receive_and_process_decoded_frames(
        &mut self,
        octx: &mut format::context::Output,
        ost_time_base: Rational,
    ) {
        let mut frame = frame::Video::empty();
        while self.decoder.receive_frame(&mut frame).is_ok() {
            self.frame_count += 1;
            let timestamp = frame.timestamp();
            self.log_progress(f64::from(
                Rational(timestamp.unwrap_or(0) as i32, 1) * self.decoder.time_base(),
            ));
            frame.set_pts(timestamp);
            frame.set_kind(picture::Type::None);
            self.send_frame_to_encoder(&frame);
            self.receive_and_process_encoded_packets(octx, ost_time_base);
        }
    }

    fn send_frame_to_encoder(&mut self, frame: &frame::Video) {
        self.encoder.send_frame(frame).unwrap();
    }

    fn send_eof_to_encoder(&mut self) {
        self.encoder.send_eof().unwrap();
    }

    fn receive_and_process_encoded_packets(
        &mut self,
        octx: &mut format::context::Output,
        ost_time_base: Rational,
    ) {
        let mut encoded = Packet::empty();
        while self.encoder.receive_packet(&mut encoded).is_ok() {
            encoded.set_stream(self.ost_index);
            encoded.rescale_ts(self.decoder.time_base(), ost_time_base);
            encoded.write_interleaved(octx).unwrap();
        }
    }

    fn log_progress(&mut self, timestamp: f64) {
        println!("logging progress");
        if !self.logging_enabled
            || (self.frame_count - self.last_log_frame_count < 100
                && self.last_log_time.elapsed().as_secs_f64() < 1.0)
        {
            return;
        }
        eprintln!(
            "time elpased: \t{:8.2}\tframe count: {:8}\ttimestamp: {:8.2}",
            self.starting_time.elapsed().as_secs_f64(),
            self.frame_count,
            timestamp
        );
        self.last_log_frame_count = self.frame_count;
        self.last_log_time = Instant::now();
    }
}

fn parse_opts<'a>(s: String) -> Option<Dictionary<'a>> {
    let mut dict = Dictionary::new();
    for keyval in s.split_terminator(',') {
        let tokens: Vec<&str> = keyval.split('=').collect();
        match tokens[..] {
            [key, val] => dict.set(key, val),
            _ => return None,
        }
    }
    println!("DICT {:?}", dict);
    Some(dict)
}

/// Live RTMP Methods
///
impl Rtmp {
    pub fn push(self: Self, stream_key: &String, file_path: &String) {
        let output_file = format!("{}/{}", self.client.config.rtmp_endpoint, stream_key);
        ffmpeg_next::init().unwrap();
        let mut ictx = format::input(&file_path).unwrap();
        let mut octx = format::output(&"output.mp4").unwrap();

        let best_video_stream_index = ictx
            .streams()
            .best(media::Type::Video)
            .map(|stream| stream.index());
        let mut stream_mapping: Vec<isize> = vec![0; ictx.nb_streams() as _];
        let mut ist_time_bases = vec![Rational(0, 0); ictx.nb_streams() as _];
        let mut ost_time_bases = vec![Rational(0, 0); ictx.nb_streams() as _];
        let mut transcoders = HashMap::new();
        let mut ost_index = 0;
        for (ist_index, ist) in ictx.streams().enumerate() {
            let ist_medium = ist.parameters().medium();
            if ist_medium != media::Type::Audio
                && ist_medium != media::Type::Video
                && ist_medium != media::Type::Subtitle
            {
                stream_mapping[ist_index] = -1;
                continue;
            }
            stream_mapping[ist_index] = ost_index;
            ist_time_bases[ist_index] = ist.time_base();
            if ist_medium == media::Type::Video {
                // Initialize transcoder for video stream.
                transcoders.insert(
                    ist_index,
                    Transcoder::new(
                        &ist,
                        &mut octx,
                        ost_index as _,
                        parse_opts(DEFAULT_X264_OPTS.to_string()).to_owned().unwrap(),
                        true,
                    )
                    .unwrap(),
                );
            } else {
                // Set up for stream copy for non-video stream.
                let mut ost = octx.add_stream(encoder::find(codec::Id::None)).unwrap();
                ost.set_parameters(ist.parameters());
                // We need to set codec_tag to 0 lest we run into incompatible codec tag
                // issues when muxing into a different container format. Unfortunately
                // there's no high level API to do this (yet).
                unsafe {
                    (*ost.parameters().as_mut_ptr()).codec_tag = 0;
                }
            }
            ost_index += 1;
        }

        octx.set_metadata(ictx.metadata().to_owned());
        format::context::output::dump(&octx, 0, Some(&output_file));
        octx.write_header().unwrap();

        for (ost_index, _) in octx.streams().enumerate() {
            ost_time_bases[ost_index] = octx.stream(ost_index as _).unwrap().time_base();
        }

        for (stream, mut packet) in ictx.packets() {
            let ist_index = stream.index();
            let ost_index = stream_mapping[ist_index];
            if ost_index < 0 {
                continue;
            }
            let ost_time_base = ost_time_bases[ost_index as usize];
            match transcoders.get_mut(&ist_index) {
                Some(transcoder) => {
                    packet.rescale_ts(stream.time_base(), transcoder.decoder.time_base());
                    transcoder.send_packet_to_decoder(&packet);
                    transcoder.receive_and_process_decoded_frames(&mut octx, ost_time_base);
                }
                None => {
                    // Do stream copy on non-video streams.
                    packet.rescale_ts(ist_time_bases[ist_index], ost_time_base);
                    packet.set_position(-1);
                    packet.set_stream(ost_index as _);
                    packet.write_interleaved(&mut octx).unwrap();
                }
            }
        }

        // Flush encoders and decoders.
        for (ost_index, transcoder) in transcoders.iter_mut() {
            let ost_time_base = ost_time_bases[*ost_index];
            transcoder.send_eof_to_decoder();
            transcoder.receive_and_process_decoded_frames(&mut octx, ost_time_base);
            transcoder.send_eof_to_encoder();
            transcoder.receive_and_process_encoded_packets(&mut octx, ost_time_base);
        }

        octx.write_trailer().unwrap();
        // spawn ffmpeg command
        /*format::context::input::dump(&ictx, 0, Some(&file_path));
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
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));*/
    }

    /// Temp ffmpeg command spawn. TODO: Replace with a proper library.
    pub fn push_to_region(self: &Self, stream_key: &String, file_path: &String, region: &String, ffmpeg_path: &String, proc_id: &mut Option<String>) -> Result<String,String> {
        let mut _region_url = String::new();

        let mut pid = String::new();

        if proc_id.is_none() {
            pid = format!("{:x}", rand::random::<u32>())
        }else{
            pid = proc_id.clone().unwrap();
        }

        let title = pid;

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
        let mut cmd = std::process::Command::new(ffmpeg_path);
        cmd.arg("-re")
            .arg("-i")
            .arg(file_path)
            .arg("-metadata")
            .arg(format!("title={}", title))
            .arg("-c:v")
            .arg("copy")
            .arg("-c:a")
            .arg("copy")
            .arg("-f")
            .arg("flv")
            .arg(format!("{}/{}", _region_url, stream_key))
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());
        let output = cmd.output().expect("failed to execute process");
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.to_string() != "exit status: 0" {
            return Err(output.status.to_string());
        }else{
            return Ok(output.status.to_string());
        }
    }
}
