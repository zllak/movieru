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
    pub fn to_file(path: impl Into<PathBuf>, size: (u16, u16), fps: f32) -> eyre::Result<Self> {
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
                format!("{}x{}", size.0, size.1).as_str(),
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
                path.into()
                    .to_str()
                    .ok_or(eyre!("path is not a utf8 string"))?,
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
    pub fn write_frame(&mut self, frame: Vec<u8>) -> eyre::Result<()> {
        self.stdin.write_all(frame.as_ref()).or_else(|err| {
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

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test() {
        let mut video =
            crate::ffmpeg::FFMpegVideoReader::from_file("/home/zllak/Downloads/test.mp4").unwrap();
        let dimensions = video.dimensions();
        let num_pix = dimensions.0 as usize * dimensions.1 as usize;
        println!(">>> {:?}", num_pix);

        let mut out = FFMpegVideoWriter::to_file("/tmp/outputtest.mp4", dimensions, 30f32).unwrap();

        let start = Instant::now();
        while let Ok(Some(mut frame)) = video.read_frame() {
            // Grayscale
            // R * 0.3 + G * 0.59 + B * 0.11
            let raw: &mut [u8] = &mut frame;
            for i in 0..num_pix {
                let i = i * 3;
                let gray = (raw[i] as f32 * 0.3) as u8
                    + (raw[i + 1] as f32 * 0.59) as u8
                    + (raw[i + 2] as f32 * 0.11) as u8;
                raw[i] = gray;
                raw[i + 1] = gray;
                raw[i + 2] = gray;
            }

            out.write_frame(frame).unwrap();
        }
        println!("took: {:?}", start.elapsed());
    }
}
