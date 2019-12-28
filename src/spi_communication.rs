use std::os::raw::c_int;

extern "C" {
    pub fn bB_Init() -> c_int;
    pub fn getBrickStats() -> ();
    pub fn brickBusInit() -> ();
    pub fn getModules() -> ();
    pub fn permCallSPI() -> ();
    pub fn bB_terminate() -> ();
}


pub unsafe fn init_spi() -> bool{
    bB_Init();
    info!("Connected to SPI Device");
    getBrickStats();
    brickBusInit();
    info!("BrickBus Initialized!");
    info!("Getting Brick Devices...");
    getModules();
    return true;
}

pub fn get_message() -> &array{

    unsafe{permCallSPI();}

}

pub unsafe fn close_spi(){
    bB_terminate();
}
