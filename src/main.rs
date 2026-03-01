mod post_processing;

use std::{path::PathBuf, str::FromStr};

use crate::post_processing::load_and_dither;

fn main() {
    println!("Program, Start!");
    let next: PathBuf = [".", "history", "test_name.png"].iter().collect();
    let path = PathBuf::from_str(r".\history\screenshot.png").unwrap();

    let img = load_and_dither(&path);

    print!("New image at: {}", next.to_str().unwrap());
    img.save(&next).unwrap();
}
