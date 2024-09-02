use opencv::{Result,core,highgui,imgcodecs,imgproc};

pub fn show_img() -> Result<()> {
    let src = match imgcodecs::imread("lion.png", imgcodecs::IMREAD_COLOR) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Failed to load image: {:?}", e);
            return Err(e);
        }
    };
    highgui::imshow("hello opencv!", &src)?;
    highgui::wait_key(1000)?;
    highgui::destroy_all_windows()?;
    Ok(())
}