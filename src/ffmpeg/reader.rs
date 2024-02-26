use anyhow::{anyhow, bail};
use std::io::Read;
use std::process::ChildStdout;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

struct RawFrame {
    data: Vec<u8>,
}

#[derive(Debug)]
struct FFMpegVideoReader {
    infos: super::infos::FFMpegInfos,
    stdout: ChildStdout,
}

impl FFMpegVideoReader {
    /// Reads a video from a given file
    pub fn from_file(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        // Non-generic inner function
        fn _from_file(path: PathBuf) -> anyhow::Result<FFMpegVideoReader> {
            if !path.as_path().is_file() {
                anyhow::bail!("not a valid file: {:?}", path);
            }

            let infos = super::infos::FFMpegInfos::from_file(path.clone())
                .map_err(|err| anyhow!("failed to fetch file infos: {:?}", err))?;
            let (width, height) = infos
                .dimensions()
                .ok_or(anyhow!("no dimensions found for given file"))?;

            let mut output = Command::new("ffmpeg")
                .args([
                    "-i",
                    path.to_str().ok_or(anyhow!("path is not utf8 string"))?,
                    "-loglevel",
                    "error",
                    "-f",
                    "image2pipe",
                    "-vf",
                    format!("scale={}:{}", width, height).as_ref(),
                    "-sws_flags",
                    "bicubic", // resize algo
                    "-pix_fmt",
                    "rgb24", // pixel format
                    "-vcodec",
                    "rawvideo",
                    "-",
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|err| anyhow!("unable to get output: {:?}", err))?;

            let stdout = output.stdout.take().expect("cannot get stdout");

            Ok(FFMpegVideoReader { infos, stdout })
        }

        _from_file(path.into())
    }

    /// Read a frame until the data is exhausted
    pub fn read_frame(&mut self) -> anyhow::Result<Option<RawFrame>> {
        let (width, height) = self
            .infos
            .dimensions()
            .ok_or(anyhow!("no dimensions found for given file"))?;

        // FIXME: depth is hardcoded to 3, should be 4 if there is an alpha layer
        let frame_size = width as usize * height as usize * 3;
        let mut buffer = vec![0; frame_size];

        // FIXME: not sure read_exact is what we want here
        self.stdout
            .read_exact(&mut buffer)
            .map_err(|err| anyhow!("failed to read: {:?}", err))?;

        Ok(Some(RawFrame { data: buffer }))
    }
}

#[cfg(test)]
mod tests {
    use super::FFMpegVideoReader;

    #[test]
    fn test() {
        let mut video = FFMpegVideoReader::from_file("/home/zllak/Downloads/test.mp4").unwrap();

        while let Some(frame) = video.read_frame().unwrap() {}
    }
}
