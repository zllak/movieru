use crate::{ffmpeg, frame::IterFrame, pixel::PixelFormat};
use eyre::eyre;
use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
pub struct Clip {
    path: PathBuf,
    // Clip informations
    // TODO: create a ClipMetadata to encapsulate everything
    infos: ffmpeg::FFMpegInfos,
    pixel_format: PixelFormat,
    duration: Duration,
    dimensions: (u32, u32),
    fps: f32,
}

impl Clip {
    /// Creates a new clip from a file.
    pub fn from_file(path: impl Into<PathBuf>) -> eyre::Result<Self> {
        let _from_file = move |path: PathBuf| -> eyre::Result<Self> {
            if !path.is_file() {
                eyre::bail!("file not found");
            }

            let infos = ffmpeg::FFMpegInfos::from_file(&path)?;
            let dimensions = infos
                .dimensions()
                .ok_or(eyre!("no video dimensions found"))?;
            let duration =
                Duration::from_secs_f32(infos.duration().ok_or(eyre!("no video duration found"))?);
            let pixel_format = infos.pixel_format().ok_or(eyre!("no pixel format found"))?;
            let fps = infos.fps().ok_or(eyre!("no video fps found"))?;

            Ok(Self {
                path,
                infos,
                duration,
                dimensions,
                fps,
                pixel_format,
            })
        };

        _from_file(path.into())
    }

    /// Iter on all the frames of the video.
    pub fn iter_frames(self) -> eyre::Result<IterFrame> {
        let reader = ffmpeg::FFMpegVideoReader::from_file(
            &self.path,
            self.dimensions,
            self.pixel_format.clone(),
        )?;

        Ok(IterFrame::new(reader, self.dimensions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EffectsExt;
    use std::time::Instant;

    #[test]
    fn test() {
        let clip = Clip::from_file("/home/zllak/Downloads/test.mp4").unwrap();

        let now = Instant::now();
        let count = clip.iter_frames().unwrap().grayscale().count();
        println!("COUNTED {:?} FRAMES in {:?}", count, now.elapsed());
    }
}
