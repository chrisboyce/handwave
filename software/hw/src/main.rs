use display::get_a;
use display::get_some_pattern;
use display::Display;
use esp_idf_hal::i2c::*;
use esp_idf_hal::prelude::*;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};
use esp_idf_sys as _;
use ht16k33::HT16K33;

mod display;
mod font;

/// Map logical matrix location to actual device coordinate 
///
/// Writing to location (0,0) in the display doesn't actually cause the corner
/// LED to light up. Instead, the LED at location (0,6) lights up. To make it
/// easier to work with the matrix, the DISPLAY_MAP returns the *actual* 
/// display x/y coordinates needed to a particular *logical* matrix location
#[rustfmt::skip]
const DISPLAY_MAP: [[(u8, u8); 8]; 8] = [
    [(0,6),(2,6),(4,6),(6,6),(8,6),(10,6),(12,6),(14,6)],
    [(0,5),(2,5),(4,5),(6,5),(8,5),(10,5),(12,5),(14,5)],
    [(0,4),(2,4),(4,4),(6,4),(8,4),(10,4),(12,4),(14,4)],
    [(0,3),(2,3),(4,3),(6,3),(8,3),(10,3),(12,3),(14,3)],
    [(0,2),(2,2),(4,2),(6,2),(8,2),(10,2),(12,2),(14,2)],
    [(0,1),(2,1),(4,1),(6,1),(8,1),(10,1),(12,1),(14,1)],
    [(0,0),(2,0),(4,0),(6,0),(8,0),(10,0),(12,0),(14,0)],
    [(0,7),(2,7),(4,7),(6,7),(8,7),(10,7),(12,7),(14,7)],
];

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    // let mut led_pin = PinDriver::output(peripherals.pins.gpio7).unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into());

    let led_matrix = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let mut ht16k33 = HT16K33::new(led_matrix, 0x70_u8);

    ht16k33.initialize().unwrap();
    ht16k33.set_display(ht16k33::Display::ON).unwrap();

    loop {
        let mut display = Display::new();
        let scroll_delay = 100;

        // for column in get_a() {
        //     display.push_column(column);
        //     ht16k33.clear_display_buffer();
        //     for (led, enabled) in display.to_leds() {
        //         ht16k33.update_display_buffer(led, enabled);
        //     }
        //     ht16k33.write_display_buffer().unwrap();
        //     FreeRtos::delay_ms(scroll_delay);
        // }
        // display.push_column(0);

        for column in get_some_pattern() {
            display.push_column(column);
            // This loop draws all the LEDs which make up the current `display`
            ht16k33.clear_display_buffer();
            for (led, enabled) in display.to_leds() {
                ht16k33.update_display_buffer(led, enabled);
            }
            ht16k33.write_display_buffer().unwrap();
            FreeRtos::delay_ms(scroll_delay);
        }

        FreeRtos::delay_ms(1000);
    }

    // loop {
    //     // Each frame is a 64-bit number, where each bit represents the on/off
    //     // state of an LED.
    //     for frame in frames {
    //         ht16k33.clear_display_buffer();
    //         // Iterate over each bit in the current frame
    //         for i in 0..64 {
    //             // i / 8 will increase by one every 8 iterations:
    //             // i    : 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    //             // i / 8: 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1
    //             //
    //             // i % 8 just repeatedly counts 0-7:
    //             // i    : 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    //             // i % 8: 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2
    //             //
    //             // Using these two numbers, we can easily walk over each index
    //             // in the matrix.
    //             let (x, y) = DISPLAY_MAP[i / 8][i % 8];

    //             //
    //             let led_location = LedLocation::new(x, y).unwrap();

    //             // Use bit shifting logic to determine if the current bit is set
    //             // or not. We use the bitwise shift `>>` to move the `i`th bit
    //             // into the "ones" place. We then bitwise "and" operator `&`
    //             // to retain only the first bit in our result. Lastly, we
    //             // check if our sole bit is equal to 1 or not.
    //             let should_be_on = frame >> i & 1 == 1;

    //             ht16k33.update_display_buffer(led_location, should_be_on);
    //         }
    //         ht16k33.write_display_buffer().unwrap();
    //         FreeRtos::delay_ms(100);
    //     }
    //     // FreeRtos::delay_ms(1000);
    // }
}
