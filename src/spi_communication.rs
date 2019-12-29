use std::os::raw::c_int;

extern "C" {
    pub fn getBrickStats() -> ();
    pub fn brickBusInit() -> u16;
    pub fn getModules() -> ();
    pub fn permCallSPI() -> ();
    pub fn bB_terminate() -> ();
    fn bB_getWord(node: u16, slaveNo: u16 , bytePos: u16) -> u16;
    fn scheduleStackSingle() -> ();
    fn _main() -> ();
}


pub fn init_spi() -> bool{
    info!("Initiating SPI Device...");
    let mut timeout_cnt: i16 = 0;
    unsafe {
        // Try to Init connection
        while timeout_cnt < 10 {
            if brickBusInit() > 0{
                break;
            }
            timeout_cnt += 1;
        }
        getBrickStats();
        info!("Connected to SPI Device");
        info!("BrickBus Initialized!");
        info!("Getting Brick Devices...");
        getModules();
    }
    return true;
}

fn get_word(node: u16, slave_no: u16 , byte_pos: u16) -> u16{
    unsafe {
        return bB_getWord(node, slave_no, byte_pos);;
    }
}

fn overflow_check(i: u16) -> u16{
    if i > 32767 {
        return 0;
    }
    return i as u16;
}

pub fn get_message() -> [u16; 16]{
    let mut byte_pos: u16 = 0;
    let mut buf: [u16; 16] = Default::default();
    unsafe{scheduleStackSingle();}
    for m in buf.iter_mut() {
        if byte_pos > 1 && byte_pos < 8 { // only for power bytes
            *m = overflow_check(256 * get_word(1, 1, byte_pos)
                + get_word(1, 1, byte_pos + 1));
        } else {
            *m = (256 * get_word(1, 1, byte_pos)
                + get_word(1, 1, byte_pos + 1));
        }

            print!("Byte{}:{} ", byte_pos, m);
        byte_pos += 2;
    }
    print!("\r");
    return buf;
}

pub fn debug_out(){
    unsafe{
        //scheduleStackSingle();
        permCallSPI();
    }
}

pub fn close_spi(){
    unsafe{bB_terminate();}
}
