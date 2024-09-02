use opencv::{
    core,
    highgui,
    imgproc,
    objdetect,
    types,
    Result,
    imgcodecs,
};
use opencv::prelude::CascadeClassifierTrait;

pub fn face_object_detection() -> Result<()> {
    let window = "face detection";

    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    let xml = core::find_file_def("haarcascades/haarcascade_frontalface_alt.xml")?;
    let mut face_detector = objdetect::CascadeClassifier::new(&xml)?;

    let mut image = imgcodecs::imread("multifaces.jpeg", imgcodecs::IMREAD_COLOR)?;

    let mut gray = core::Mat::default();
    imgproc::cvt_color_def(&image, &mut gray, imgproc::COLOR_BGR2GRAY)?;

    let mut faces = types::VectorOfRect::new();
    face_detector.detect_multi_scale(
        &gray,
        &mut faces,
        1.3, 
        1,  
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size { width: 10, height: 10 }, // 탐지 객체
        core::Size { width: 1000, height: 1000 },
    )?;

    for face in faces.iter() {
        let scaled_face = core::Rect::new(
            face.x * 1,
            face.y * 1,
            face.width * 1,
            face.height * 1,
        );
        imgproc::rectangle(&mut image, scaled_face, core::Scalar::new(255.0, 0.0, 0.0, 0.0), 2, 8, 0)?;

        let text = format!("Face: ({}, {})", face.x, face.y);
        let org = core::Point::new(face.x, face.y - 10);
        imgproc::put_text(&mut image, &text, org, imgproc::FONT_HERSHEY_SIMPLEX, 0.5, core::Scalar::new(0.0, 255.0, 0.0, 0.0), 1, imgproc::LINE_AA, false)?;
    }

    println!("Detected faces: {}", faces.len());

    // highgui::imshow(window, &image)?;
    // highgui::wait_key(0)?;
    Ok(())
}