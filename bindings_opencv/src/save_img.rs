use opencv::{core::{self, MatTraitConst}, highgui, imgcodecs, Result};

pub fn save_img() -> Result<()> {

    let mut src = core::Mat::default();
    src = imgcodecs::imread("lion.jpeg", imgcodecs::IMREAD_COLOR)?;

    if src.empty(){
        println!("{}", "image load failed");
        std::process::exit(0);
    }

    let mut parms = core::Vector::default();
    parms.push(imgcodecs::IMWRITE_JPEG_QUALITY);
    parms.push(95);
    imgcodecs::imwrite("write_test.png", &src, &parms)?;
    
    highgui::imshow("image", &src)?;
    highgui::wait_key(10000)?;
    highgui::destroy_all_windows()?;

    Ok(())
}