#![warn(unused_extern_crates)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

use std::os::raw::c_int;
use clap::{App, Arg};
use crate::config::Cfg;

mod config;

extern "C" {
    pub fn spiInit() -> c_int;
    pub fn getBrickStats() -> ();
    pub fn brickBusInit() -> ();
    pub fn getModules() -> ();
}


unsafe fn init_spi() -> bool{
    spiInit();
    info!("Connected to SPI Device");
    getBrickStats();
    brickBusInit();
    info!("BrickBus Initialized!");
    info!("Getting Brick Devices...");
    getModules();
    return true;
}


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
    let config = init();
    env_logger::init();
    // Main App
    unsafe {
        init_spi();
    }
    info!("spi2mqtt exit")
}
