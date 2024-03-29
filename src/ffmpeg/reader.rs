use eyre::{bail, eyre, Result};
use std::io::Read;
use std::process::ChildStdout;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub(crate) struct FFMpegVideoReader {
    width: u32,
    height: u32,
    stdout: ChildStdout,
    pixel_depth: u8,
    max_nb_frames: u32, // maximum number of frames to read
    current_frame: u32, // when reading, the current frame number handled
}

impl FFMpegVideoReader {
    /// Reads a video from a given file.
    /// This methods does not get the video informations from FFMpeg, it uses
    /// what is given as parameters
    pub fn from_file(
        path: &PathBuf,
        (width, height): (u32, u32),
        pixel_depth: u8,
        start: String,
        max_nb_frames: u32,
    ) -> Result<Self> {
        if !path.as_path().is_file() {
            bail!("not a valid file: {:?}", path);
        }

        let pix_fmt = if pixel_depth == 3 { "rgb24" } else { "rgba" };
        let start = start.as_ref();

        let mut output = Command::new("ffmpeg")
            .args([
                "-ss",
                start,
                "-i",
                path.to_str().ok_or(eyre!("path is not utf8 string"))?,
                "-loglevel",
                "error",
                "-f",
                "image2pipe",
                "-vf",
                format!("scale={}:{}", width, height).as_ref(),
                "-sws_flags",
                "bicubic", // resize algo
                "-pix_fmt",
                pix_fmt,
                "-vcodec",
                "rawvideo",
                "-",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| eyre!("unable to get output: {:?}", err))?;

        let stdout = output.stdout.take().expect("cannot get stdout");

        Ok(Self {
            stdout,
            width,
            height,
            pixel_depth,
            max_nb_frames,
            current_frame: 0,
        })
    }

    /// Read a frame until the data is exhausted
    pub fn read_frame(&mut self) -> Result<Option<Vec<u8>>> {
        // If we have hit the frame limit, stop reading
        if self.current_frame >= self.max_nb_frames {
            return Ok(None);
        }

        let frame_size = self.width as usize * self.height as usize * self.pixel_depth as usize;
        let mut buffer = vec![0; frame_size];

        // FIXME: not sure read_exact is what we want here
        self.stdout
            .read_exact(&mut buffer)
            .map_err(|err| eyre!("failed to read: {:?}", err))?;

        self.current_frame += 1;

        Ok(Some(buffer))
    }
}
