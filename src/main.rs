mod bluetooth;
mod post_processing;

use std::{path::PathBuf, str::FromStr};

use crate::{
    bluetooth::display_image,
    post_processing::{load_and_dither, to_bytes},
};

#[tokio::main]
async fn main() {
    println!("Program, Start!");
    let next: PathBuf = [".", "history", "latest.png"].iter().collect();
    let path = PathBuf::from_str(r".\history\screenshot.png").unwrap();

    let img = load_and_dither(&path);

    println!("New image at: {}", next.to_str().unwrap());
    img.save(&next).unwrap();
    let image_bytes = to_bytes(&img);

    display_image(image_bytes.as_slice()).await;
}
