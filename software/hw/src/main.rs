use display::frame_to_leds;
use display::Display;
use esp_idf_svc::hal::i2c::I2cConfig;
use esp_idf_svc::hal::i2c::I2cDriver;
use esp_idf_svc::hal::prelude::Peripherals;
// use esp_idf_hal::i2c::*;
// use esp_idf_hal::prelude::*;
// use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};
// use esp_idf_sys as _;
use ht16k33::HT16K33;

mod display;
mod font;
mod util;

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
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100_000.into());

    let led_matrix = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let mut ht16k33 = HT16K33::new(led_matrix, 0x70_u8);

    ht16k33.initialize().unwrap();

    ht16k33.set_display(ht16k33::Display::ON).unwrap();

    // loop {
    //     let mut scrolling_text = Display::new_scrolling_text(&"Wow!{");
    //     let mut animation = Display::new_animation(&vec![
    //         562958543486983,
    //         562958543488770,
    //         562958543814658,
    //         562958660927490,
    //         562980018323458,
    //         570655124881410,
    //         1970333427040258,
    //         504966116808982530,
    //     ]);

    //     let scroll_delay = 70;

    //     let mut counter = 0;
    //     loop {
    //         // Update the display
    //         scrolling_text.update();
    //         // Manually slow down the animation rate by skipping every
    //         // other loop
    //         if counter % 3 == 0 {
    //             animation.update();
    //         }
    //         counter += 1;

    //         // The `mask` value here selects the top 4 rows of the scrolling
    //         // text and the bottom 4 from the animation
    //         // let composited_frame = scrolling_text.composite_leds(&animation, 17361641481138401520);
    //         ht16k33.clear_display_buffer();
    //         // for (led, enabled) in frame_to_leds(composited_frame) {
    //         // for (led, enabled) in animation.to_leds() {
    //         for (led, enabled) in scrolling_text.to_leds() {
    //             ht16k33.update_display_buffer(led, enabled);
    //         }
    //         ht16k33.write_display_buffer().unwrap();
    //         FreeRtos::delay_ms(scroll_delay);
    //     }
    // }
    // log::info!("Hello, world!");
}
