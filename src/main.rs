extern crate glium;
extern crate winit;
use log::*;
use simple_logger::SimpleLogger;

fn main(){
    println!("Hello World. Mukund if you touch this project again you're done");
    SimpleLogger::new().init().unwrap();
    info!("Loggers are functional");
}
