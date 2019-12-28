#![warn(unused_extern_crates)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

use clap::{App, Arg};
use crate::config::Cfg;

use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod config;
mod spi_communication;

fn init() -> Cfg{
    // Init
    let arg = App::new("spi2mqtt")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Location of the config file")
                .takes_value(true)
                .default_value("config.yaml"),
        );

    let matches = &arg.get_matches();
    let config = matches.value_of("config").unwrap();
    let cfg_loaded = config::load_cfg(config);
    //logger::init_logger(&cfg_loaded);
    return cfg_loaded;
}

fn main() {
    // Init
    env_logger::init();
    let config = init();

    // Init spi
    unsafe {
        spi_communication::init_spi();
    }

    // Set Handler for Ctrl-C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // Get Messages
    while running.load(Ordering::SeqCst) {
            spi_communication::get_message()
    }
    unsafe{spi_communication::close_spi()};
    info!("spi2mqtt exit");
}
