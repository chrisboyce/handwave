use std::f32::consts::TAU;
use std::time::Duration;

use display::frame_to_leds;
use display::Display;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::i2c::I2cConfig;
use esp_idf_svc::hal::i2c::I2cDriver;
use esp_idf_svc::hal::i2s;
use esp_idf_svc::hal::i2s::config as I2sConfig;
use esp_idf_svc::hal::i2s::config::StdConfig;
use esp_idf_svc::hal::i2s::I2sDriver;
use esp_idf_svc::hal::i2s::I2sTx;
use esp_idf_svc::hal::i2s::I2sTxSupported;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::sys::EspError;
// use esp_idf_hal::i2c::*;
// use esp_idf_hal::prelude::*;
// use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};
// use esp_idf_sys as _;
use ht16k33::LedLocation;
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
    [(0,7),(2,7),(4,7),(6,7),(8,7),(10,7),(12,7),(14,7)],
    [(0,6),(2,6),(4,6),(6,6),(8,6),(10,6),(12,6),(14,6)],
    [(0,5),(2,5),(4,5),(6,5),(8,5),(10,5),(12,5),(14,5)],
    [(0,4),(2,4),(4,4),(6,4),(8,4),(10,4),(12,4),(14,4)],
    [(0,3),(2,3),(4,3),(6,3),(8,3),(10,3),(12,3),(14,3)],
    [(0,2),(2,2),(4,2),(6,2),(8,2),(10,2),(12,2),(14,2)],
    [(0,1),(2,1),(4,1),(6,1),(8,1),(10,1),(12,1),(14,1)],
    [(0,0),(2,0),(4,0),(6,0),(8,0),(10,0),(12,0),(14,0)],
];

const TIMEOUT: Duration = Duration::from_millis(100);
const SAMPLE_RATE_HZ: u32 = 16000;
const OMEGA_INC: f32 = TAU / SAMPLE_RATE_HZ as f32;
const BITS_PER_SAMPLE: I2sConfig::DataBitWidth = I2sConfig::DataBitWidth::Bits16;
const DMA_BUFFERS: usize = 12;
const DMA_FRAMES: usize = 240;

struct SendTriangleWave {
    buffer: Vec<u8>,
}

impl SendTriangleWave {
    fn new(freq: f32) -> Self {
        let buffer_size = (SAMPLE_RATE_HZ as f32 / freq) as usize;
        let mut buffer = vec![0; buffer_size * 4];
        let mut value: f32 = 0.0;
        let mut value_inc = 0.1 / (buffer_size as f32);

        for i in (0..buffer.len()).step_by(4) {
            let i_value = (value * (i16::MAX as f32)) as i16 as u16;

            buffer[i] = (i_value & 0x00ff) as u8;
            buffer[i + 1] = ((i_value & 0xff00) >> 8) as u8;
            buffer[i + 2] = (i_value & 0x00ff) as u8;
            buffer[i + 3] = ((i_value & 0xff00) >> 8) as u8;
            value += value_inc;

            if value_inc > 0.0 && value > 1.0 {
                value = 2.0 - value;
                value_inc = -value_inc;
            } else if value_inc < 0.0 && value < 1.0 {
                value = -2.0 - value;
                value_inc = -value_inc;
            }
        }

        Self { buffer }
    }
}

impl SendTriangleWave {
    pub fn send<Dir: I2sTxSupported>(
        &mut self,
        driver: &mut I2sDriver<'_, Dir>,
    ) -> Result<usize, EspError> {
        driver.write(&self.buffer, 1000)
    }
}

fn main() {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let i2s_config = I2sConfig::Config::default();
    let clk_config = I2sConfig::StdClkConfig::from_sample_rate_hz(SAMPLE_RATE_HZ)
        .clk_src(I2sConfig::ClockSource::Pll160M);
    let gpio_config = I2sConfig::StdGpioConfig::default();

    let slot_config = I2sConfig::StdSlotConfig::philips_slot_default(
        BITS_PER_SAMPLE,
        I2sConfig::SlotMode::Stereo,
    );
    let std_config = I2sConfig::StdConfig::new(i2s_config, clk_config, slot_config, gpio_config);

    println!("Initializing I2S driver");
    let bclk = peripherals.pins.gpio2;
    let dout = peripherals.pins.gpio4;
    let ws = peripherals.pins.gpio1;
    let mclk = AnyIOPin::none();
    let mut i2s =
        I2sDriver::<I2sTx>::new_std_tx(peripherals.i2s0, &std_config, bclk, dout, mclk, ws)
            .unwrap();
    let mut wave = SendTriangleWave::new(440.0);
    println!("Enabling output");
    i2s.tx_enable().unwrap();

    println!("Starting transmission");

    loop {
        wave.send(&mut i2s).unwrap();
    }
    // I2sDriver::new_std_tx(
    //     peripherals.i2s0,
    //     &std_config,
    //     bclk,
    //     dout,
    //     AnyIOPin::none(),
    //     ws,
    // )
    // .unwrap();
    // let mut i2s = I2sStdDriver::<I2sTx>::new_tx(
    //     peripherals.i2s0,
    //     std_config,
    //     bclk,
    //     Some(dout),
    //     AnyIOPin::none(),
    //     ws,
    // )
    // .unwrap();

    let sda = peripherals.pins.gpio10;
    let scl = peripherals.pins.gpio8;

    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100_000.into());

    let led_matrix = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let mut ht16k33 = HT16K33::new(led_matrix, 0x70_u8);

    ht16k33.initialize().unwrap();

    ht16k33.set_display(ht16k33::Display::ON).unwrap();
    loop {
        for i in 0..16 {
            for j in 0..8 {
                ht16k33.clear_display_buffer();
                let led = LedLocation::new(i, j).unwrap();
                ht16k33.update_display_buffer(led, true);
                ht16k33.write_display_buffer().unwrap();
                FreeRtos::delay_ms(50);
            }
        }
        FreeRtos::delay_ms(50);
    }

    loop {
        let mut scrolling_text = Display::new_scrolling_text(&"Wow!{");
        let mut animation = Display::new_animation(&vec![
            18393018710207300353,
            // 562958543486983,
            // 562958543488770,
            // 562958543814658,
            // 562958660927490,
            // 562980018323458,
            // 570655124881410,
            // 1970333427040258,
            // 504966116808982530,
        ]);

        let scroll_delay = 70;

        let mut counter = 0;
        loop {
            // Update the display
            scrolling_text.update();
            // Manually slow down the animation rate by skipping every
            // other loop
            if counter % 3 == 0 {
                animation.update();
            }
            counter += 1;

            // The `mask` value here selects the top 4 rows of the scrolling
            // text and the bottom 4 from the animation
            // let composited_frame = scrolling_text.composite_leds(&animation, 17361641481138401520);
            ht16k33.clear_display_buffer();
            // for (led, enabled) in frame_to_leds(composited_frame) {
            for (led, enabled) in animation.to_leds() {
                // for (led, enabled) in scrolling_text.to_leds() {
                ht16k33.update_display_buffer(led, enabled);
            }
            ht16k33.write_display_buffer().unwrap();
            FreeRtos::delay_ms(scroll_delay);
        }
    }
}
