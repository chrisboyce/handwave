use esp_idf_hal::i2c::*;
use esp_idf_hal::prelude::*;
use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver, peripherals::Peripherals};
use esp_idf_sys as _;
use ht16k33::{LedLocation, HT16K33};

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio7).unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into());

    let led_matrix = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let mut ht16k33 = HT16K33::new(led_matrix, 0x70_u8);

    ht16k33.initialize().unwrap();
    ht16k33.set_display(ht16k33::Display::ON).unwrap();

    #[rustfmt::skip]
    let display_map = [
        [(0,6),(2,6),(4,6),(6,6),(8,6),(10,6),(12,6),(14,6)],
        [(0,5),(2,5),(4,5),(6,5),(8,5),(10,5),(12,5),(14,5)],
        [(0,4),(2,4),(4,4),(6,4),(8,4),(10,4),(12,4),(14,4)],
        [(0,3),(2,3),(4,3),(6,3),(8,3),(10,3),(12,3),(14,3)],
        [(0,2),(2,2),(4,2),(6,2),(8,2),(10,2),(12,2),(14,2)],
        [(0,1),(2,1),(4,1),(6,1),(8,1),(10,1),(12,1),(14,1)],
        [(0,0),(2,0),(4,0),(6,0),(8,0),(10,0),(12,0),(14,0)],
        [(0,7),(2,7),(4,7),(6,7),(8,7),(10,7),(12,7),(14,7)],
    ];
    // let led_location = LedLocation::new(g.0, g.1).unwrap();
    loop {
        // ht16k33.update_display_buffer(led_location, true);
        // ht16k33.write_display_buffer().unwrap();
        // ht16k33.update_display_buffer(led_location, false);
        // ht16k33.write_display_buffer().unwrap();
        for i in 0..8 {
            for j in 0..8 {
                let g = display_map[i][j];
                let led_location = LedLocation::new(g.0, g.1).unwrap();
                ht16k33.clear_display_buffer();
                ht16k33.update_display_buffer(led_location, true);
                ht16k33.write_display_buffer().unwrap();
                FreeRtos::delay_us(1000);
            }
        }
        // FreeRtos::delay_ms(1000);
        // led_pin.set_low().unwrap();

        // led_pin.set_high().unwrap();
        // FreeRtos::delay_ms(1000);
        // println!("blink");
    }
}
