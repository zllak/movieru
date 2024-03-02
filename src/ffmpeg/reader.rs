use crate::pixel::PixelFormat;
use eyre::{eyre, Result};
use std::io::Read;
use std::process::ChildStdout;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Debug)]
pub(crate) struct FFMpegVideoReader {
    infos: super::infos::FFMpegInfos,
    width: u16,
    height: u16,
    pixel_format: PixelFormat,
    stdout: ChildStdout,
}

impl FFMpegVideoReader {
    /// Reads a video from a given file
    /// TODO: should we allow specifying the desired pixel format ?
    pub fn from_file(path: impl Into<PathBuf>) -> Result<Self> {
        // Non-generic inner function
        fn _from_file(path: PathBuf) -> Result<FFMpegVideoReader> {
            if !path.as_path().is_file() {
                eyre::bail!("not a valid file: {:?}", path);
            }

            let infos = super::infos::FFMpegInfos::from_file(path.clone())
                .map_err(|err| eyre!("failed to fetch file infos: {:?}", err))?;
            let (width, height) = infos
                .dimensions()
                .ok_or(eyre!("no dimensions found for given file"))?;
            let pixel_format = infos.pixel_format().ok_or(eyre!(
                "no pixel format could be extracted from the given file"
            ))?;

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

            Ok(FFMpegVideoReader {
                infos,
                stdout,
                width,
                height,
                pixel_format,
            })
        }

        _from_file(path.into())
    }

    /// Returns the dimensions of the video
    pub fn dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
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
