//! Sample module to test openCV
//! 
use log::info;
use anyhow::Result;
use opencv::{
    prelude::*,
    videoio,
    highgui
};

fn main() -> Result<()> {
    env_logger::init();

    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    
    info!("Starting {} v {}",pkg,ver);

    highgui::named_window(pkg, highgui::WINDOW_FULLSCREEN)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    let mut frame = Mat::default();

    loop {
        cam.read(&mut frame)?;
        highgui::imshow(pkg, &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 {
            break;
        }
    }

    Ok(())
}
