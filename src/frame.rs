use crate::pixel::PixelFormat;

pub struct Frame {
    data: Vec<u8>,
    pixel_format: PixelFormat,
}

impl Frame {
    pub fn new(data: Vec<u8>, pixel_format: PixelFormat) -> Self {
        Self { data, pixel_format }
    }
}

// ----------------------------------------------------------------------------

pub struct IterFrame<'a> {
    reader: &'a crate::ffmpeg::FFMpegVideoReader,
}

impl<'a> IterFrame<'a> {
    fn new(reader: &'a crate::ffmpeg::FFMpegVideoReader) -> Self {
        Self { reader }
    }
}

impl<'a> Iterator for IterFrame<'a> {
    type Item = &'a mut Frame;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
