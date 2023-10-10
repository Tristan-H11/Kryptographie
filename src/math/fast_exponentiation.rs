use log::{debug, error, info, trace, warn};

pub fn hello() {
    trace!("Trace test");
    debug!("Debug Test");
    info!("Info Test");
    warn!("Warn Test");
    error!("Error Test");
    println!("fast_exponentation is here!")
}

pub fn run(base: i32, exponent: i32, modulos: i32) -> i32 {
    panic!("TODO")
}