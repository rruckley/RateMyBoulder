//! Sample module to test openCV
//! 
use log::info;
use anyhow::Result;
use opencv::{
    self as cv,
    prelude::*,
    highgui,
    imgproc,
};

fn main() -> Result<()> {
    env_logger::init();

    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    
    info!("Starting {} v {}",pkg,ver);

    highgui::named_window(pkg, highgui::WINDOW_FULLSCREEN)?;

    //let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    let mut frame = Mat::default();

    // Read the image
    let img = opencv::imgcodecs::imread("boulder1.jpg", cv::imgcodecs::IMREAD_GRAYSCALE)?;

    // Seem pretty close
    let mut thresh1= 51.0;
    let mut thresh2 = 75.0;

    loop {
        info!("Now using thresholds {} , {}",thresh1, thresh2);
        imgproc::canny(
            &img,
            &mut frame,
            thresh1,
            thresh2,
            3,
            true
        )?;
        highgui::imshow(pkg, &frame)?;
        thresh1 += 5.0;
        thresh2 += 6.0;
        
        let key = highgui::wait_key(5000)?;
        if key == 113 { // quit with q
            break;
        }
    }

    Ok(())
}
