use crate::common::Result;
use crate::{BufferedImageLuminanceSource, Exceptions, LuminanceSource};
use image::{DynamicImage, RgbaImage};
use resvg::usvg::TreeParsing;
use resvg::{self, usvg::Options};

pub struct SVGLuminanceSource(BufferedImageLuminanceSource);

impl LuminanceSource for SVGLuminanceSource {
    fn get_row(&self, y: usize) -> Vec<u8> {
        self.0.get_row(y)
    }

    fn get_column(&self, x: usize) -> Vec<u8> {
        self.0.get_column(x)
    }

    fn get_matrix(&self) -> Vec<u8> {
        self.0.get_matrix()
    }

    fn get_width(&self) -> usize {
        self.0.get_width()
    }

    fn get_height(&self) -> usize {
        self.0.get_height()
    }

    fn is_crop_supported(&self) -> bool {
        self.0.is_crop_supported()
    }

    fn crop(&self, left: usize, top: usize, width: usize, height: usize) -> Result<Self> {
        self.0.crop(left, top, width, height).map(Self)
    }

    fn is_rotate_supported(&self) -> bool {
        self.0.is_rotate_supported()
    }

    fn invert(&mut self) {
        self.0.invert()
    }

    fn rotate_counter_clockwise(&self) -> Result<Self> {
        self.0.rotate_counter_clockwise().map(Self)
    }

    fn rotate_counter_clockwise_45(&self) -> Result<Self> {
        self.0.rotate_counter_clockwise_45().map(Self)
    }

    fn get_luma8_point(&self, x: usize, y: usize) -> u8 {
        self.0.get_luma8_point(x, y)
    }
}

impl SVGLuminanceSource {
    pub fn new(svg_data: &[u8]) -> Result<Self> {
        // Load the SVG file
        let Ok(tree) = resvg::usvg::Tree::from_data(svg_data, &Options::default()) else {
            return Err(Exceptions::format_with(format!(
                "could not parse svg data: {}",
                "err"
            )));
        };

        let Some(mut pixmap) =
            resvg::tiny_skia::Pixmap::new(tree.size.width() as u32, tree.size.height() as u32)
        else {
            return Err(Exceptions::format_with("could not create pixmap"));
        };

        let tree_resvg = resvg::Tree::from_usvg(&tree);

        tree_resvg.render(resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());

        let Some(buffer) = RgbaImage::from_raw(
            tree.size.width() as u32,
            tree.size.height() as u32,
            pixmap.data().to_vec(),
        ) else {
            return Err(Exceptions::format_with("could not create image buffer"));
        };

        // let Ok(image) = image::load_from_memory_with_format(pixmap.data(), image::ImageFormat::Bmp)  else {
        //     return Err(Exceptions::format("could not generate image"));
        // };

        Ok(Self(BufferedImageLuminanceSource::new(DynamicImage::from(
            buffer,
        ))))
    }
}
