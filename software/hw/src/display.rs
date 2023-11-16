use bytemuck::cast;
use ht16k33::LedLocation;

use crate::{
    font::string_to_columns,
    util::{from_columns, to_columns},
    DISPLAY_MAP,
};

/// A type alias to make it more "ergonomic" to work with `u8`s in the context
/// of 8-LED colums.
pub type Column = u8;

/// Defines display behavior and state
pub enum DisplayMode {
    /// Marquee-style scrolling of the given columns
    Scroll {
        /// Used to determine which line to "push" next on to the display
        next_column_index: usize,

        /// The entire list of columns to scroll through. Note that only a
        /// subset of these will be likely visible at a time, since they
        /// may not all fit on the display at once. That is to say, this
        /// holds all of the columns, not just the ones being displayed.
        columns: Vec<Column>,
    },

    /// Cycles through a vector of `u64`s, treating them as frames of an
    /// animation.
    Animate { frames: Vec<u64>, next_frame: usize },
}

/// Display state
pub struct Display {
    /// The on/off state of the 64 LEDs on the display as represented by a
    /// 64-bit number
    pub leds: u64,

    pub mode: DisplayMode,
}

impl Display {
    /// Convert the current `Display` into a list of LED states, each indicating
    /// its [`LedLocation`] and its on/off state via a `bool`.
    pub fn to_leds(&self) -> Vec<(LedLocation, bool)> {
        let columns = to_columns(self.leds);
        columns
            .iter()
            // The `enumerate` call essentially adds the index/counter
            // to each of the columns being iterated over
            .enumerate()
            // `map` takes in one value, in this case it's a "tuple" of
            // two values, the `column_index` and the `column_value`. It then
            // returns a different value, usually using the initial value
            // in some way.
            .map(|(column_index, column_value)| {
                // Loop over each of 8 bits, numbered 0-7
                (0..8).map(move |i| {
                    // Use "bitwise" operations to determine if the i'th bit
                    // is set
                    let led_state = column_value >> i & 1;

                    // Convert the "logical" location we want into the actual
                    // coordinates the matrix needs to address the desired
                    // location.
                    let (x, y) = DISPLAY_MAP[i][7 - column_index];
                    let led_location = LedLocation::new(x, y).unwrap();

                    // Return a tuple containing the LedLocation, and the on/off
                    // state.
                    (led_location, led_state == 1)
                })
            })
            // At this point, we actually have a nested collection, since each
            // column has 8 LedLocations, and there are 8 columns. The `flatten`
            // call has the effect of flattening out all the LedLocations into
            // a single array.
            .flatten()
            // Lastly, collect our new list into a Vec
            .collect()
    }

    /// Create a new, empty display
    pub fn new() -> Self {
        Self {
            // By default, the display will be set to the following mode
            mode: DisplayMode::Scroll {
                next_column_index: 0,
                columns: string_to_columns(&"BAB"),
            },
            leds: 0,
        }
    }

    /// This function updates the state of the LEDs depending on which
    /// [`DisplayMode`] is used.
    pub fn update(&mut self) {
        match self.mode {
            DisplayMode::Scroll {
                ref mut next_column_index,
                ref columns,
            } => {
                let next_column = columns[*next_column_index];
                let current_columns = to_columns(self.leds);
                let shifted_columns = [
                    current_columns[1],
                    current_columns[2],
                    current_columns[3],
                    current_columns[4],
                    current_columns[5],
                    current_columns[6],
                    current_columns[7],
                    next_column,
                ];
                self.leds = from_columns(shifted_columns);

                *next_column_index = (*next_column_index + 1_usize) % columns.len();
            }
            DisplayMode::Animate { .. } => todo!(),
        }
    }
}
