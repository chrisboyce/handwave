use crate::display::Column;

/// Based on https://gist.github.com/rothwerx/700f275d078b3483509f

/// Maps a numeric index corresponding to a particular character into its
/// 64-bit representation as on/off LEDs on the matrix.
///
/// This mapping is based on an [existing orders and values]
/// (https://gist.github.com/rothwerx/700f275d078b3483509f).
///
///

/// Due to the "endianness", even though the padding columns are on the right
/// of the other bits, when we loop over the 8 bytes what make up this 64-bit
/// number the bytes on the right will be the first that are accessed
const FONT_MAP_NEW: [u64; 2] = [
    0b00111110_01111110_11001000_11001000_01111110_00111110_00000000_00000000, // 'A'
    0b01101100_10010010_10010010_11111110_11111110_10000010_00000000_00000000, // 'B'
];

#[rustfmt::skip]
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
pub fn _create_matrix(lines: &str) -> u64 {
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
pub fn _create_matrix_from_pattern(lines: [&str; 8]) -> u64 {
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

    _create_matrix(&joined_lines)
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
        .find(|(_column_index, column_value)| *column_value != 0);

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
