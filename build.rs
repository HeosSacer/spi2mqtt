fn main() {
    let mut config = cc::Build::new();
    config
        .file("src/c/Appl.c")
        .file("src/c/bB_EasyAPI.c")
        .file("src/c/bB_master.c")
        .file("src/c/SPI_Driver.c")
        .file("src/c/bcm2835.c")
        .include("src/c")
        .compile("spi_interface");
}