mod show_img;
use show_img::show_img;
use opencv::{Result,core,highgui,imgcodecs,imgproc};

mod object_detection;
use object_detection::face_object_detection;

mod save_img;
use save_img::save_img;

pub fn main() -> Result<()> {
    // show_img()?;
    // save_img()?;
    use std::time::Instant;
    let now = Instant::now();
    {
        face_object_detection()?;
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}