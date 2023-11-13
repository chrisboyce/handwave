use bytemuck::cast;
use ht16k33::LedLocation;

use crate::{font::string_to_columns, DISPLAY_MAP};

pub type Column = u8;

pub enum DisplayMode {
    Scroll {
        current_column: usize,
        columns: Vec<Column>,
    },
    Animate(Vec<u64>),
}

pub struct Display {
    leds: u64,
    mode: DisplayMode,
}

impl Display {
    pub fn to_leds(&self) -> Vec<(LedLocation, bool)> {
        let columns: [u8; 8] = cast(self.leds);
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

    // pub fn _push_columns<C: std::iter::IntoIterator<Item = Column>>(&mut self, columns: C) {
    //     for column in columns.into_iter() {
    //         self.push_column(column);
    //     }
    // }

    pub fn push_column(self, column: Column) -> Self {
        let columns: [u8; 8] = cast(self.leds);
        let columns = [
            columns[1], columns[2], columns[3], columns[4], columns[5], columns[6], columns[7],
            column,
        ];
        let leds = cast(columns);
        Self { leds, ..self }
    }

    /// Create a new, empty display
    pub fn new() -> Self {
        Self {
            // By default, the display will be set to the following mode
            mode: DisplayMode::Scroll {
                current_column: 0,
                columns: string_to_columns(&"Hi Pop! :)"),
            },
            leds: 0,
        }
    }

    pub fn tick(&self) -> Self {
        // match self.mode {
        //     DisplayMode::Scroll {
        //         current_column,
        //         columns,
        //     } => {
        //         // let mut
        //         // self.push_column(columns[current_column]);
        //         // current_column = (current_column + 1) % columns.len();
        //     }
        //     DisplayMode::Animate(_) => {}
        // }
        todo!()
    }

    /// Return list of 8 u8's, each one representing the bits that are "on" in
    /// particular column
    ///
    // For reference, the bits are mapped to the display grid by the following
    // ordering:
    //
    // 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 <- columns
    //
    // 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 ,
    // 8 , 9 , 10, 11, 12, 13, 14, 15,
    // 16, 17, 18, 19, 20, 21, 22, 23,
    // 24, 25, 26, 27, 28, 29, 30, 31,
    // 32, 33, 34, 35, 36, 37, 38, 39,
    // 40, 41, 42, 43, 44, 45, 46, 47,
    // 48, 49, 50, 51, 52, 53, 54, 55,
    // 56, 57, 58, 59, 60, 61, 62, 63,
    fn _columns(&self) -> [u8; 8] {
        // let masks: [u64; 8] =
        //     [0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000];

        todo!()
    }
}
