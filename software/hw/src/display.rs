use ht16k33::LedLocation;

use crate::DISPLAY_MAP;

pub type Column = u8;

pub struct Display {
    columns: [Column; 8],
}

#[rustfmt::skip]
pub fn get_a() -> [u8; 6] {
    [
        0b00111110,
        0b01111110, 
        0b11001000, 
        0b11001000, 
        0b01111110, 
        0b00111110]
}
#[rustfmt::skip]
pub fn get_b() -> [u8; 6] {
    [
        0b10000010,
        0b11111110,
        0b11111110,
        0b10010010,
        0b10010010,
        0b01101100,
    ]
}

#[rustfmt::skip]
pub fn get_some_pattern() -> [u8; 3] {
    [
        0b11111111,
        0b10100000, 
        0b11100000, 
    ]
}
impl Display {
    /// Convert the display state into a u64 representing the 64 LED states
    pub fn _as_u64(&self) -> u64 {
        bytemuck::cast(self.columns)
    }

    pub fn to_leds(&self) -> Vec<(LedLocation, bool)> {
        self.columns
            // First, turn `colums` into a value which can repeatedly
            // return items
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

    pub fn _push_columns<C: std::iter::IntoIterator<Item = Column>>(&mut self, columns: C) {
        for column in columns.into_iter() {
            self.push_column(column);
        }
    }

    pub fn push_column(&mut self, column: Column) {
        self.columns = [
            self.columns[1],
            self.columns[2],
            self.columns[3],
            self.columns[4],
            self.columns[5],
            self.columns[6],
            self.columns[7],
            column,
        ]
    }

    /// Create a new, empty display
    pub fn new() -> Self {
        Self {
            columns: [0, 0, 0, 0, 0, 0, 0, 0],
        }
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
