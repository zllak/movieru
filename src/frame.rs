use crate::pixel::PixelFormat;

pub struct Frame {
    data: Vec<u8>,
    size: usize,
    pixel_format: PixelFormat,
}

impl Frame {
    pub fn new(data: Vec<u8>, size: usize, pixel_format: PixelFormat) -> Self {
        Self {
            data,
            size,
            pixel_format,
        }
    }

    /// Transforms the current frame using the given function.
    /// TODO: maybe we should have a notion of "pixel" (RGB, YUV, alpha layer, ...)
    /// Only assumes rgb24 for now, and does not try to be smart when transforming.
    pub fn transform<F>(mut self, func: F) -> Self
    where
        F: Fn(u8, u8, u8) -> (u8, u8, u8),
    {
        let raw: &mut [u8] = &mut self.data;
        for i in 0..self.size {
            let i = i * 3;
            let (r, g, b) = func(raw[i], raw[i + 1], raw[i + 2]);
            raw[i] = r;
            raw[i + 1] = g;
            raw[i + 2] = b;
        }

        self
    }
}

// ----------------------------------------------------------------------------

pub struct IterFrame {
    reader: crate::ffmpeg::FFMpegVideoReader,
    size: usize,
    pixel_format: PixelFormat,
}

impl IterFrame {
    pub(crate) fn new(
        reader: crate::ffmpeg::FFMpegVideoReader,
        size: usize,
        pixel_format: PixelFormat,
    ) -> Self {
        Self {
            reader,
            size,
            pixel_format,
        }
    }
}

impl Iterator for IterFrame {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader
            .read_frame()
            .ok()?
            .map(|raw_frame| Frame::new(raw_frame, self.size, self.pixel_format.clone()))
    }
}
