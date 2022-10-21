
use log::{trace,debug,info,warn,error, log_enabled, Level};

mod foo {
    mod bar {
        pub fn run() {
            log::warn!("[bar] warn");
            log::info!("[bar] info");
            log::debug!("[bar] debug");
        }
    }

    pub fn run() {
        log::warn!("[foo] warn");
        log::info!("[foo] info");
        log::debug!("[foo] debug");
        bar::run();
    }
}

fn main() {

    env_logger::init();

    trace!("Trace!");
    debug!("Debug!");
    info!("Info!");
    warn!("Warn!");
    error!("Error!");

    if log_enabled!(Level::Trace) {
        trace!("Extra trace.........................");
    }

    if log_enabled!(Level::Info) {
        info!("Extra info.........................");
    }

    foo::run();
    
    
}
