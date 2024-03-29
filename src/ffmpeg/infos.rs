use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(tag = "codec_type")]
#[allow(dead_code)]
pub(crate) enum FFMpegStream {
    #[serde(rename = "video")]
    Video {
        index: u8,
        codec_name: String,
        codec_long_name: String,
        profile: String,
        codec_tag_string: String,
        codec_tag: String,
        width: u32,
        height: u32,
        coded_width: u16,
        coded_height: u16,
        closed_captions: u16,
        film_grain: u32,
        has_b_frames: u32,
        sample_aspect_ratio: Option<String>, // make some kind of aspect ratio struct?
        display_aspect_ratio: Option<String>,
        pix_fmt: String,
        level: u32,
        color_range: Option<String>,
        color_space: Option<String>,
        chroma_location: String,
        field_order: String,
        refs: u32,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_bool_from_anything")]
        is_avc: bool,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        nal_length_size: u32,
        id: String,
        r_frame_rate: String,
        avg_frame_rate: String,
        time_base: String,
        start_pts: u32,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        start_time: f32,
        duration_ts: u32,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        duration: f32,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        bit_rate: u32,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        bits_per_raw_sample: u16,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        nb_frames: u32,
        extradata_size: u32,
        disposition: HashMap<String, u32>,
        tags: HashMap<String, String>,
    },
    #[serde(rename = "audio")]
    Audio {
        index: u32,
        codec_name: String,
        codec_long_name: String,
        profile: String,
        codec_tag_string: String,
        codec_tag: String,
        sample_fmt: String,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        sample_rate: u32,
        channels: u16,
        channel_layout: String, // stereo/mono I guess ?
        bits_per_sample: u32,
        initial_padding: u32,
        id: String,
        r_frame_rate: String,
        avg_frame_rate: String,
        time_base: String,
        start_pts: u32,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        start_time: f32,
        duration_ts: u32,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        duration: f32,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        bit_rate: u32,
        #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
        nb_frames: u32,
        extradata_size: u32,
        disposition: HashMap<String, u32>,
        tags: HashMap<String, String>,
    },
}

#[derive(serde::Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct FFMpegFormat {
    filename: String,
    nb_streams: u16,
    nb_programs: u16,
    format_name: String,
    format_long_name: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    start_time: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    duration: f32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    size: u64,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    bit_rate: u32,
    probe_score: u32,
    tags: HashMap<String, String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct FFMpegInfos {
    streams: Vec<FFMpegStream>,
    format: FFMpegFormat,
}

impl FFMpegInfos {
    /// Runs ffprobe to get informations about the given file
    pub(crate) fn from_file(path: &PathBuf) -> eyre::Result<Self> {
        if !path.as_path().is_file() {
            eyre::bail!("not a valid file: {:?}", path);
        }

        let output = Command::new("ffprobe")
            .args([
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
                path.to_str()
                    .ok_or(eyre::eyre!("path is not utf8 string"))?,
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()
            .map_err(|err| eyre::eyre!("unable to get output: {:?}", err))?;

        if !output.status.success() {
            eyre::bail!("Call to ffmpeg failed: {:?}", output.status);
        }

        let out = String::from_utf8_lossy(&output.stdout);

        serde_json::from_str(out.as_ref())
            .map_err(|err| eyre::eyre!("unable to parse JSON: {:?}", err))
    }

    /// Returns the dimensions of the video, None if there is no video stream
    pub(crate) fn dimensions(&self) -> Option<(u32, u32)> {
        self.streams.iter().find_map(|stream| match stream {
            FFMpegStream::Video { width, height, .. } => Some((*width, *height)),
            FFMpegStream::Audio { .. } => None,
        })
    }

    /// Returns the right Pixel tuple of name and pixel depth
    // TODO: the depth should be strong typed
    pub(crate) fn pixel(&self) -> Option<(&'static str, u8)> {
        self.streams.iter().find_map(|stream| match stream {
            FFMpegStream::Video { pix_fmt, .. } => match pix_fmt.as_str() {
                "rgb24" => Some(("rgb24", 3)),
                "yuv420p" => Some(("yuv420p", 3)),
                _ => None,
            },
            FFMpegStream::Audio { .. } => None,
        })
    }

    /// Returns the number of frames per second. None if there is no video stream.
    pub(crate) fn fps(&self) -> Option<f32> {
        self.streams.iter().find_map(|stream| match stream {
            FFMpegStream::Video {
                duration,
                nb_frames,
                ..
            } => Some(*nb_frames as f32 / *duration),
            FFMpegStream::Audio { .. } => None,
        })
    }

    /// Returns the number of frames in the video strea. None if there is no video stream.
    pub(crate) fn nb_frames(&self) -> Option<usize> {
        self.streams.iter().find_map(|stream| match stream {
            FFMpegStream::Video { nb_frames, .. } => Some(*nb_frames as usize),
            FFMpegStream::Audio { .. } => None,
        })
    }

    /// Returns the duration in seconds. None if there is no video stream.
    pub(crate) fn duration(&self) -> Option<f32> {
        self.streams.iter().find_map(|stream| match stream {
            FFMpegStream::Video { duration, .. } => Some(*duration),
            FFMpegStream::Audio { .. } => None,
        })
    }
}
