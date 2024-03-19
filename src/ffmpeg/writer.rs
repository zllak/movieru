use eyre::eyre;
use std::{
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{ChildStderr, ChildStdin, Command, Stdio},
};

#[derive(Debug)]
pub(crate) struct FFMpegVideoWriter {
    stderr: BufReader<ChildStderr>,
    stdin: ChildStdin,
}

impl FFMpegVideoWriter {
    // Size is (width, height)
    pub fn to_file(path: &PathBuf, (width, height): (u32, u32), fps: f32) -> eyre::Result<Self> {
        // Assumes a lot of things:
        // libx264 codec, medium encoding preset, rgb24 pixel format

        let mut command = Command::new("ffmpeg")
            .args([
                "-y",
                "-loglevel",
                "error",
                "-f",
                "rawvideo",
                "-s",
                format!("{}x{}", width, height).as_str(),
                "-pix_fmt",
                "rgb24",
                "-r",
                format!("{:.2}", fps).as_str(),
                "-an",
                "-i",
                "-",
                "-vcodec",
                "libx264",
                "-preset",
                "medium",
                // "-threads", "X"
                path.to_str().ok_or(eyre!("path is not a utf8 string"))?,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| eyre!("unable to spawn command: {:?}", err))?;

        let stdin = command.stdin.take().expect("cannot get stdin");
        let stderr = BufReader::new(command.stderr.take().expect("cannot get stderr"));

        Ok(FFMpegVideoWriter { stdin, stderr })
    }

    /// Write a frame to the output file
    pub fn write_frame(&mut self, frame: &[u8]) -> eyre::Result<()> {
        self.stdin.write_all(frame).or_else(|err| {
            // Got an error, read stderr
            let mut stderr = String::new();
            loop {
                let read = self
                    .stderr
                    .read_line(&mut stderr)
                    .map_err(|err| eyre!("unable to read stderr: {:?}", err))?;
                if read == 0 {
                    break;
                }
            }
            Err(eyre::eyre!(
                "unable to write: {:?}, stderr: {:?}",
                err,
                stderr
            ))
        })
    }
}
