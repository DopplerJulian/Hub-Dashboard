use std::path::Path;

use image::{
    ImageBuffer, ImageReader, Rgb,
    imageops::{ColorMap, FilterType, dither, resize},
};

pub fn load_and_dither<P>(path: P) -> ImageBuffer<Rgb<u8>, Vec<u8>>
where
    P: AsRef<Path>,
{
    let mut img = ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();

    let target_width: u32 = 800;
    let target_height: u32 = 480;

    if img.height() != target_height || img.width() != target_width {
        resize(
            &mut img,
            target_width,
            target_height,
            FilterType::CatmullRom,
        );
    }

    dither(&mut img, &RedWhiteBlack);

    img
}

#[derive(Clone, Copy)]
pub struct RedWhiteBlack;

impl ColorMap for RedWhiteBlack {
    type Color = Rgb<u8>;

    #[inline(always)]
    fn index_of(&self, color: &Rgb<u8>) -> usize {
        let rgb = color.0;
        if rgb[0] > 127 && rgb[1] <= 127 && rgb[2] <= 127 {
            1
        } else if ((rgb[0] as u16 + rgb[1] as u16 + rgb[2] as u16) / 3) > 127 {
            2
        } else {
            0
        }
    }

    #[inline(always)]
    fn lookup(&self, idx: usize) -> Option<Self::Color> {
        const WHITE: [u8; 3] = [255; 3];
        const BLACK: [u8; 3] = [0; 3];
        const RED: [u8; 3] = [255, 0, 0];

        match idx {
            0 => Some(BLACK.into()),
            1 => Some(RED.into()),
            2 => Some(WHITE.into()),
            _ => None,
        }
    }

    /// Indicate `NeuQuant` implements `lookup`.
    fn has_lookup(&self) -> bool {
        true
    }

    #[inline(always)]
    fn map_color(&self, color: &mut Rgb<u8>) {
        let new_color = self.lookup(self.index_of(color)).unwrap();
        *color = new_color;
    }
}
