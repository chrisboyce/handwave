use crate::display::Column;

/// Based on https://gist.github.com/rothwerx/700f275d078b3483509f

/// Maps a numeric index corresponding to a particular character into its
/// 64-bit representation as on/off LEDs on the matrix.
///
/// This mapping is based on an [existing orders and values]
/// (https://gist.github.com/rothwerx/700f275d078b3483509f).
///
///

const FONT_MAP_4_X_4: [u16; 1] = [
    0b0111_1010_1010_0111, // 'A'
];

/// Due to the "endianness", even though the padding columns are on the right
/// of the other bits, when we loop over the 8 bytes what make up this 64-bit
/// number the bytes on the right will be the first that are accessed
const FONT_MAP_NEW: [u64; 2] = [
    0b00111110_01111110_11001000_11001000_01111110_00111110_00000000_00000000, // 'A'
    0b01101100_10010010_10010010_11111110_11111110_10000010_00000000_00000000, // 'B'
];

#[rustfmt::skip]
const FONT_MAP: [u64; 69] = [
    0b00110000_01111000_11001100_11001100_11111100_11001100_11001100_00000000, // 'A' 
    0b11111100_01100110_01100110_01111100_01100110_01100110_11111100_00000000, // 'B' 
    0b00111100_01100110_11000000_11000000_11000000_01100110_00111100_00000000, // 'C' 
    0b11111000_01101100_01100110_01100110_01100110_01101100_11111000_00000000, // 'D' 
    0b11111110_01100010_01101000_01111000_01101000_01100010_11111110_00000000, // 'E' 
    0b11111110_01100010_01101000_01111000_01101000_01100000_11110000_00000000, // 'F' 
    0b00111100_01100110_11000000_11000000_11001110_01100110_00111110_00000000, // 'G' 
    0b11001100_11001100_11001100_11111100_11001100_11001100_11001100_00000000, // 'H' 
    0b01111000_00110000_00110000_00110000_00110000_00110000_01111000_00000000, // 'I' 
    0b00011110_00001100_00001100_00001100_11001100_11001100_01111000_00000000, // 'J' 
    0b11110110_01100110_01101100_01111000_01101100_01100110_11110110_00000000, // 'K' 
    0b11110000_01100000_01100000_01100000_01100010_01100110_11111110_00000000, // 'L' 
    0b11000110_11101110_11111110_11111110_11010110_11000110_11000110_00000000, // 'M' 
    0b11000110_11100110_11110110_11011110_11001110_11000110_11000110_00000000, // 'N' 
    0b00111000_01101100_11000110_11000110_11000110_01101100_00111000_00000000, // 'O' 
    0b11111100_01100110_01100110_01111100_01100000_01100000_11110000_00000000, // 'P' 
    0b01111000_11001100_11001100_11001100_11011100_01111000_00011100_00000000, // 'Q' 
    0b11111100_01100110_01100110_01111100_01101100_01100110_11110110_00000000, // 'R' 
    0b01111000_11001100_11100000_01110000_00011100_11001100_01111000_00000000, // 'S' 
    0b11111100_10110100_00110000_00110000_00110000_00110000_01111000_00000000, // 'T' 
    0b11001100_11001100_11001100_11001100_11001100_11001100_11111100_00000000, // 'U' 
    0b11001100_11001100_11001100_11001100_11001100_01111000_00110000_00000000, // 'V' 
    0b11000110_11000110_11000110_11010110_11111110_11101110_11000110_00000000, // 'W' 
    0b11000110_11000110_01101100_00111000_00111000_01101100_11000110_00000000, // 'X' 
    0b11001100_11001100_11001100_01111000_00110000_00110000_01111000_00000000, // 'Y' 
    0b11111110_11000110_10001100_00011000_00110010_01100110_11111110_00000000, // 'Z' 
    0b00000000_00000000_01111000_00001100_01111100_11001100_01110110_00000000, // 'a' 
    0b11100000_01100000_01100000_01111100_01100110_01100110_11011100_00000000, // 'b' 
    0b00000000_00000000_01111000_11001100_11000000_11001100_01111000_00000000, // 'c' 
    0b00011100_00001100_00001100_01111100_11001100_11001100_01110110_00000000, // 'd' 
    0b00000000_00000000_01111000_11001100_11111100_11000000_01111000_00000000, // 'e' 
    0b00111000_01101100_01100000_11110000_01100000_01100000_11110000_00000000, // 'f' 
    0b00000000_00000000_01110110_11001100_11001100_01111100_00001100_11111000, // 'g' 
    0b11100000_01100000_01101100_01110110_01100110_01100110_11100110_00000000, // 'h' 
    0b00110000_00000000_01110000_00110000_00110000_00110000_01111000_00000000, // 'i' 
    0b00001100_00000000_00001100_00001100_00001100_11001100_11001100_01111000, // 'j' 
    0b11100000_01100000_01100110_01101100_01111000_01101100_11100110_00000000, // 'k' 
    0b01110000_00110000_00110000_00110000_00110000_00110000_01111000_00000000, // 'l' 
    0b00000000_00000000_11001100_11111110_11111110_11010110_11000110_00000000, // 'm' 
    0b00000000_00000000_11111000_11001100_11001100_11001100_11001100_00000000, // 'n' 
    0b00000000_00000000_01111000_11001100_11001100_11001100_01111000_00000000, // 'o' 
    0b00000000_00000000_11011100_01100110_01100110_01111100_01100000_11110000, // 'p' 
    0b00000000_00000000_01110110_11001100_11001100_01111100_00001100_00011110, // 'q' 
    0b00000000_00000000_10011100_01110110_01100110_01100000_11110000_00000000, // 'r' 
    0b00000000_00000000_01111100_11000000_01111000_00001100_11111000_00000000, // 's' 
    0b00010000_00110000_01111100_00110000_00110000_00110100_00011000_00000000, // 't' 
    0b00000000_00000000_11001100_11001100_11001100_11001100_01110110_00000000, // 'u' 
    0b00000000_00000000_11001100_11001100_11001100_01111000_00110000_00000000, // 'v' 
    0b00000000_00000000_11000110_11000110_11010110_11111110_01101100_00000000, // 'w' 
    0b00000000_00000000_11000110_01101100_00111000_01101100_11000110_00000000, // 'x' 
    0b00000000_00000000_11001100_11001100_11001100_01111100_00001100_11111000, // 'y' 
    0b00000000_00000000_11111100_10011000_00110000_01100100_11111100_00000000, // 'z' 
    0b01111000_11001100_11011100_11111100_11101100_11001100_01111100_00000000, // '0' 
    0b00110000_01110000_00110000_00110000_00110000_00110000_11111100_00000000, // '1' 
    0b01111000_11001100_00001100_00111000_01100000_11001100_11111100_00000000, // '2' 
    0b01111000_11001100_00001100_00111000_00001100_11001100_01111000_00000000, // '3' 
    0b00011100_00111100_01101100_11001100_11111110_00001100_00011110_00000000, // '4' 
    0b11111100_11000000_11111000_00001100_00001100_11001100_01111000_00000000, // '5' 
    0b00111000_01100000_11000000_11111000_11001100_11001100_01111000_00000000, // '6' 
    0b11111100_11001100_00001100_00011000_00110000_00110000_00110000_00000000, // '7' 
    0b01111000_11001100_11001100_01111000_11001100_11001100_01111000_00000000, // '8' 
    0b01111000_11001100_11001100_01111100_00001100_00011000_01110000_00000000, // '9' 
    0b00110000_01111000_01111000_00110000_00110000_00000000_00110000_00000000, // '!' 
    0b01101100_01101100_11111110_01101100_11111110_01101100_01101100_00000000, // '#' 
    0b00110000_01111100_11000000_01111000_00001100_11111000_00110000_00000000, // '$' 
    0b00000000_11000110_11001100_00011000_00110000_01100110_11000110_00000000, // '%' 
    0b00111000_01101100_00111000_01110110_11011100_11001100_01110110_00000000, // '&' 
    0b01111000_11001100_00001100_00011000_00110000_00000000_00110000_00000000, // '?' 
    0b11111111_11000011_10100101_10011001_10011001_10100101_11000011_11111111, // Our "not found" pattern 
];

// These `const`s provide a different way to refer to the character values
pub const A: u64 = FONT_MAP[0];
pub const B: u64 = FONT_MAP[1];
pub const C: u64 = FONT_MAP[2];
pub const D: u64 = FONT_MAP[3];
pub const E: u64 = FONT_MAP[4];
pub const F: u64 = FONT_MAP[5];
pub const G: u64 = FONT_MAP[6];
pub const H: u64 = FONT_MAP[7];
pub const I: u64 = FONT_MAP[8];
pub const J: u64 = FONT_MAP[9];
pub const K: u64 = FONT_MAP[10];
pub const L: u64 = FONT_MAP[11];
pub const M: u64 = FONT_MAP[12];
pub const N: u64 = FONT_MAP[13];
pub const O: u64 = FONT_MAP[14];
pub const P: u64 = FONT_MAP[15];
pub const Q: u64 = FONT_MAP[16];
pub const R: u64 = FONT_MAP[17];
pub const S: u64 = FONT_MAP[18];
pub const T: u64 = FONT_MAP[19];
pub const U: u64 = FONT_MAP[20];
pub const V: u64 = FONT_MAP[21];
pub const W: u64 = FONT_MAP[22];
pub const X: u64 = FONT_MAP[23];
pub const Y: u64 = FONT_MAP[24];
pub const Z: u64 = FONT_MAP[25];

// Rust doesn't like lower-case `const`s. We can manually disable this [lint check]
// (https://doc.rust-lang.org/reference/attributes/diagnostics.html#lint-check-attributes)
// with the `#[allow(non_upper_case_globals)]` direcctive, but that's a warning
// that we might be mis-using the ability to have lower-case `const`s and it
// doesn't match recommended coding styles.
//
// ```
// pub const a: u64 = FONT_MAP[26];
// pub const b: u64 = FONT_MAP[27];
// pub const c: u64 = FONT_MAP[28];
// pub const d: u64 = FONT_MAP[29];
// ...
// ```

#[allow(non_upper_case_globals)]
pub const o: u64 = FONT_MAP[40];

// Note too, that we can't represent characters like '!' using this pattern,
// since `pub const !: u64..` isn't valid syntax due to the restrictions on
// `const` names.

/// Even though, at the moment,
pub struct Char(u64);

impl Char {
    pub fn new_from_char(char_to_convert: char) -> Self {
        let converted_value = char_to_u64(char_to_convert);
        Self(converted_value)
    }
}

/// Convert a character into its 64-bit LED representation,
pub fn char_to_u64(char_to_convert: char) -> u64 {
    match char_to_convert {
        'A' => FONT_MAP[0],
        'B' => FONT_MAP[1],
        'C' => FONT_MAP[2],
        'D' => FONT_MAP[3],
        'E' => FONT_MAP[4],
        'F' => FONT_MAP[5],
        'G' => FONT_MAP[6],
        'H' => FONT_MAP[7],
        'I' => FONT_MAP[8],
        'J' => FONT_MAP[9],
        'K' => FONT_MAP[10],
        'L' => FONT_MAP[11],
        'M' => FONT_MAP[12],
        'N' => FONT_MAP[13],
        'O' => FONT_MAP[14],
        'P' => FONT_MAP[15],
        'Q' => FONT_MAP[16],
        'R' => FONT_MAP[17],
        'S' => FONT_MAP[18],
        'T' => FONT_MAP[19],
        'U' => FONT_MAP[20],
        'V' => FONT_MAP[21],
        'W' => FONT_MAP[22],
        'X' => FONT_MAP[23],
        'Y' => FONT_MAP[24],
        'Z' => FONT_MAP[25],
        'a' => FONT_MAP[26],
        'b' => FONT_MAP[27],
        'c' => FONT_MAP[28],
        'd' => FONT_MAP[29],
        'e' => FONT_MAP[30],
        'f' => FONT_MAP[31],
        'g' => FONT_MAP[32],
        'h' => FONT_MAP[33],
        'i' => FONT_MAP[34],
        'j' => FONT_MAP[35],
        'k' => FONT_MAP[36],
        'l' => FONT_MAP[37],
        'm' => FONT_MAP[38],
        'n' => FONT_MAP[39],
        'o' => FONT_MAP[40],
        'p' => FONT_MAP[41],
        'q' => FONT_MAP[42],
        'r' => FONT_MAP[43],
        's' => FONT_MAP[44],
        't' => FONT_MAP[45],
        'u' => FONT_MAP[46],
        'v' => FONT_MAP[47],
        'w' => FONT_MAP[48],
        'x' => FONT_MAP[49],
        'y' => FONT_MAP[50],
        'z' => FONT_MAP[51],
        '0' => FONT_MAP[52],
        '1' => FONT_MAP[53],
        '2' => FONT_MAP[54],
        '3' => FONT_MAP[55],
        '4' => FONT_MAP[56],
        '5' => FONT_MAP[57],
        '6' => FONT_MAP[58],
        '7' => FONT_MAP[59],
        '8' => FONT_MAP[60],
        '9' => FONT_MAP[61],
        '!' => FONT_MAP[62],
        '#' => FONT_MAP[63],
        '$' => FONT_MAP[64],
        '%' => FONT_MAP[65],
        '&' => FONT_MAP[66],
        '?' => FONT_MAP[67],
        ' ' => 0,
        // All other characters get the "missing" pattern
        _ => FONT_MAP[68],
    }
}

/// Converts a series of bits represented in a string to a u64
///
/// Our 8x8 LED matrix has 64 LEDs, the same number of bits in a 64-bit integer.
/// This means we can represent the on/off state of the LEDs as bits in a u64.
///
/// This function strips any characters that aren't "0" or "1" from a string
/// and then converts the resulting binary number in string form into a u64.
///
/// ```rust
/// leds = create_matrix("00000000 00000000 11111111 11111111 00000000 11111111 10101010 01010101");
/// ```
pub fn create_matrix(lines: &str) -> u64 {
    let mut result: u64 = 0;
    for char in lines.chars() {
        match char {
            '1' => {
                result = result << 1;
                result = result | 1;
            }
            '0' => {
                result = result << 1;
            }
            _ => {}
        }
    }
    result
}

/// Creates a 64-bit LED matrix value from a pattern of lines
///
/// For each line, the pattern is repeated until 8 characters have been created.
/// Pattern lengths should be 1-8 characters in length.
///
pub fn create_matrix_from_pattern(lines: [&str; 8]) -> u64 {
    let mut joined_lines = vec![];

    for line in lines {
        let line = line
            .repeat(8_usize.div_ceil(line.len()))
            .chars()
            .take(8)
            .collect::<String>();
        joined_lines.push(line);
    }

    let joined_lines = joined_lines.join("");

    create_matrix(&joined_lines)
}

pub fn char_to_columns(char_to_convert: char) -> Vec<Column> {
    let char_as_u64 = match char_to_convert {
        'A' => FONT_MAP_NEW[0],
        'B' => FONT_MAP_NEW[1],
        ' ' => 0,
        // Pattern to indicate we had no match for the given `char`
        _ => 0b11111111_10000001_10000001_10000001_10000001_10000001_10000001_11111111,
    };

    // `Column` is just a type alias to `u8`, so bytemuck can cast the u64 to
    // 8 `u8`s
    let char_as_columns: [Column; 8] = bytemuck::cast(char_as_u64);

    let first_non_empty_column: Option<(usize, Column)> = char_as_columns
        .into_iter()
        .enumerate()
        .find(|(column_index, column_value)| *column_value != 0);

    match first_non_empty_column {
        // Here, we "destructure" the value of `first_non_empty_column` into
        // a specific variable named `column_index`, and discard the column
        // value by assigning it to `_` since we don't actually need the value
        // here.
        Some((column_index, _)) => char_as_columns[(column_index as usize)..].to_vec(),

        // If all columns are empty, that means the value is zero. Assume this
        // is because we matched a space character above, so we manually return
        // a single empty column to represent the space.
        None => vec![0],
    }
}

pub fn string_to_columns(input: &str) -> Vec<Column> {
    input
        .chars()
        .flat_map(|cur_char| char_to_columns(cur_char))
        .collect()
}
