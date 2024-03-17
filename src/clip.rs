use crate::{ffmpeg, frame::IterFrame};
use eyre::eyre;
use std::{fmt::Display, path::PathBuf, time::Duration};

// TODO: move this to a separate file ?
#[derive(Debug, Clone, Default)]
pub struct TimeDuration {
    pub hour: u32,
    pub min: u32,
    pub sec: u32,
}

impl TimeDuration {
    pub fn new(hour: u32, min: u32, sec: u32) -> Self {
        Self { hour, min, sec }
    }
}

impl Display for TimeDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.min, self.sec)
    }
}

#[derive(Debug, Clone)]
pub struct Clip {
    path: PathBuf,
    // Clip informations
    // TODO: create a ClipMetadata to encapsulate everything
    infos: ffmpeg::FFMpegInfos,
    duration: Duration,
    start: TimeDuration, // used to generate subclips
    max_nb_frames: u32,  // number of frames to read before stopping
    dimensions: (u32, u32),
    fps: f32,
    pixel_depth: u8,
    nb_frames: usize,
}

impl Clip {
    /// Create a new clip
    fn new(
        path: impl Into<PathBuf>,
        start: Option<TimeDuration>,
        end: Option<TimeDuration>,
    ) -> eyre::Result<Self> {
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
            let (_, pixel_depth) = infos.pixel().ok_or(eyre!("no pixel format found"))?;
            let fps = infos.fps().ok_or(eyre!("no video fps found"))?;
            let nb_frames = infos.nb_frames().ok_or(eyre!("no video nb frames found"))?;
            let start = start.unwrap_or_default();
            let max_nb_frames = end
                .map(|end| {
                    let secs = end.hour * 3600 + end.min * 60 + end.sec;
                    let nb_frames = secs as f32 * fps;
                    nb_frames as u32
                })
                .unwrap_or(nb_frames as u32);

            Ok(Self {
                path,
                infos,
                start,
                max_nb_frames,
                duration,
                dimensions,
                fps,
                pixel_depth,
                nb_frames,
            })
        };

        _from_file(path.into())
    }

    /// Creates a new clip from a file.
    pub fn from_file(path: impl Into<PathBuf>) -> eyre::Result<Self> {
        Self::new(path, None, None)
    }

    /// Iter on all the frames of the video.
    /// FFMpegVideoReader will seek until the `start`, and will stop after
    /// `max_nb_frames`
    /// We pass `max_nb_frames` to `IterFrame` just for the iterator's size hint.
    pub fn iter_frames(self) -> eyre::Result<IterFrame> {
        let reader = ffmpeg::FFMpegVideoReader::from_file(
            &self.path,
            self.dimensions,
            self.pixel_depth,
            self.start.to_string(),
            self.max_nb_frames,
        )?;

        Ok(IterFrame::new(
            reader,
            self.dimensions,
            self.max_nb_frames as usize,
        ))
    }

    /// Create a subclip from the current clip
    pub fn subclip(&self, start: TimeDuration, during: TimeDuration) -> eyre::Result<Self> {
        Self::new(self.path.clone(), Some(start), Some(during))
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
