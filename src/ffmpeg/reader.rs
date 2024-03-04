use crate::pixel::PixelFormat;
use eyre::{bail, eyre, Result};
use std::io::Read;
use std::process::ChildStdout;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub(crate) struct FFMpegVideoReader {
    width: u16,
    height: u16,
    pixel_format: PixelFormat,
    stdout: ChildStdout,
}

impl FFMpegVideoReader {
    /// Reads a video from a given file.
    /// This methods does not get the video informations from FFMpeg, it uses
    /// what is given as parameters
    pub fn from_file(
        path: impl Into<PathBuf>,
        (width, height): (u16, u16),
        pixel_format: PixelFormat,
    ) -> Result<Self> {
        // Non-generic inner function
        let _from_file = move |path: PathBuf| -> Result<Self> {
            if !path.as_path().is_file() {
                bail!("not a valid file: {:?}", path);
            }

            // To simplify things, for now, use rgb24 or rgba
            let pix_fmt = if pixel_format.has_alpha_layer() {
                "rgba"
            } else {
                "rgb24"
            };

            let mut output = Command::new("ffmpeg")
                .args([
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
                pixel_format,
            })
        };

        _from_file(path.into())
    }

    /// Read a frame until the data is exhausted
    pub fn read_frame(&mut self) -> Result<Option<Vec<u8>>> {
        let depth = if self.pixel_format.has_alpha_layer() {
            4
        } else {
            3
        };
        let frame_size = self.width as usize * self.height as usize * depth;
        let mut buffer = vec![0; frame_size];

        // FIXME: not sure read_exact is what we want here
        self.stdout
            .read_exact(&mut buffer)
            .map_err(|err| eyre!("failed to read: {:?}", err))?;

        Ok(Some(buffer))
    }
}
