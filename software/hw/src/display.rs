pub struct Display(u64);

// if we make the byte order in the u46 the order for the column data, we can just grab each byte in turn to get the next column

impl Display {
    pub fn new(state: u64) -> Self {
        Self(state)
    }

    /// Return list of 8 u8's, each one representing the bits that are "on" in
    /// particular column
    ///
    // For reference, the bits are mapped to the display grid by the following
    // ordering:
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
