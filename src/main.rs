//! Sample module to test openCV
//! 
use log::info;
use anyhow::Result;
use opencv::{
    self as cv,
    prelude::*,
    highgui,
    core::Vector,
};

const HSV_INC : i32 = 16;

fn main() -> Result<()> {
    env_logger::init();

    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    
    info!("Starting {} v {}",pkg,ver);

    highgui::named_window(pkg, highgui::WINDOW_FULLSCREEN)?;

    //let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    let mut frame = Mat::default();

    // Read the image
    let img = opencv::imgcodecs::imread("boulder2.png", cv::imgcodecs::IMREAD_ANYCOLOR)?;
    let _hsv = cv::imgproc::cvt_color(&img, &mut frame, cv::imgproc::COLOR_RGB2HSV,0 )?;
    // Try and find some colours first
    // Conver to HSV space
    //let hsv_img = imgproc::

    // Seem pretty close
    /***
    let thresh1= 51.0;
    let thresh2 = 75.0;
    info!("Now using thresholds {} , {}",thresh1, thresh2);
    imgproc::canny(
        &img,
        &mut frame,
        thresh1,
        thresh2,
        3,
        true
    )?;
    **/
    let mut mask = Mat::default();

    

    let mut hsv = 0;

    loop {
        if hsv > 240 { break; } ;
        // Testing Red
        let lower : Vector<i32> = cv::core::Vector::from_iter(vec![hsv,150,150]);
        let upper : Vector<i32> = cv::core::Vector::from_iter(vec![hsv+HSV_INC,250,250]);

        info!("Base range: {hsv} - {}",hsv+48);
        cv::core::in_range(&img, &lower, &upper, &mut mask)?;

        let mut segment = Mat::default();
        cv::core::bitwise_and(&img, &255.0, &mut segment, &mask)?;

        highgui::imshow(pkg, &segment)?;
        let key = highgui::wait_key(5000)?;
        if key == 113 { // quit with q
            break;
        }

        hsv += HSV_INC;
    }

    Ok(())
}
