use crate::{BufferedImageLuminanceSource, Exceptions, LuminanceSource};
use image::{DynamicImage, RgbaImage};
use resvg::{self, usvg::Options};

pub struct SVGLuminanceSource(BufferedImageLuminanceSource);

impl LuminanceSource for SVGLuminanceSource {
    fn getRow(&self, y: usize) -> Vec<u8> {
        self.0.getRow(y)
    }

    fn getMatrix(&self) -> Vec<u8> {
        self.0.getMatrix()
    }

    fn getWidth(&self) -> usize {
        self.0.getWidth()
    }

    fn getHeight(&self) -> usize {
        self.0.getHeight()
    }

    fn invert(&mut self) {
        self.0.invert()
    }

    fn isCropSupported(&self) -> bool {
        self.0.isCropSupported()
    }

    fn crop(
        &self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Box<dyn LuminanceSource>, Exceptions> {
        self.0.crop(left, top, width, height)
    }

    fn isRotateSupported(&self) -> bool {
        self.0.isRotateSupported()
    }

    fn rotateCounterClockwise(&self) -> Result<Box<dyn LuminanceSource>, Exceptions> {
        self.0.rotateCounterClockwise()
    }

    fn rotateCounterClockwise45(&self) -> Result<Box<dyn LuminanceSource>, Exceptions> {
        self.0.rotateCounterClockwise45()
    }
}

impl SVGLuminanceSource {
    pub fn new(svg_data: &[u8]) -> Result<Self, Exceptions> {
        // Load the SVG file
        let Ok(tree) = resvg::usvg::Tree::from_data(svg_data, &Options::default()) else {
            return Err(Exceptions::format(format!("could not parse svg data: {}", "err")));
        };

        let Some(mut pixmap) = resvg::tiny_skia::Pixmap::new(tree.size.width() as u32, tree.size.height() as u32) else {
            return Err(Exceptions::format("could not create pixmap"));
        };

        resvg::render(
            &tree,
            resvg::usvg::FitTo::Original,
            resvg::tiny_skia::Transform::default(),
            pixmap.as_mut(),
        );

        let Some(buffer) = RgbaImage::from_raw(tree.size.width() as u32, tree.size.height() as u32, pixmap.data().to_vec()) else {
        return Err(Exceptions::format("could not create image buffer"));
    };

        // let Ok(image) = image::load_from_memory_with_format(pixmap.data(), image::ImageFormat::Bmp)  else {
        //     return Err(Exceptions::format("could not generate image"));
        // };

        Ok(Self(BufferedImageLuminanceSource::new(DynamicImage::from(
            buffer,
        ))))
    }
}
