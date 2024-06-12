use simple_logger;
use log::{self, info};
use bevy;
fn main(){
    simple_logger::SimpleLogger::new().env().init().unwrap();
    info!("loggers are now functional, bevy is also loaded if all goes well");

}
