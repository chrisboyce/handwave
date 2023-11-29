use crate::{display::Column, util::to_columns};

/// Based on <https://gist.github.com/rothwerx/700f275d078b3483509f>

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
        'A' => 9189745159477133312,
        'a' => 1086798065051893760,
        'B' => 7967309272029593600,
        'b' => 1013610157977894912,
        'C' => 9331882296053071872,
        'c' => 1229782938195853312,
        'D' => 9115709516103548928,
        'd' => 9156118282367926272,
        'E' => 9336403558750224384,
        'e' => 870625283139436544,
        'F' => 10416984890535837696,
        'f' => 4634273285796790272,
        'G' => 2202684085925576704,
        'g' => 4487068767339151360,
        'H' => 18379207744482705408,
        'h' => 1085385173232517120,
        'I' => 18374686479671623680,
        'i' => 16068843470457929728,
        'J' => 18339080998957875200,
        'j' => 6773696414053564416,
        'K' => 9314046669131612160,
        'k' => 651337513465544704,
        'L' => 72340177099423744,
        'l' => 72477607479738368,
        'M' => 18383711412829552640,
        'm' => 1085384071841841152,
        'N' => 18376956013388496896,
        'n' => 1085385171353468928,
        'O' => 9115709513939288064,
        'o' => 1013610156082069504,
        'P' => 6958220376715296768,
        'p' => 1739555094166241280,
        'Q' => 9043933394878070784,
        'q' => 4549801260991119360,
        'R' => 8039084287284215808,
        'r' => 1157442765391396864,
        'S' => 10273152278592487424,
        's' => 1302970847283118080,
        'T' => 9259542668734660608,
        't' => 74659072908984320,
        'U' => 18302911468678414336,
        'u' => 2162010400424460288,
        'V' => 16148785786395820032,
        'v' => 1731072232454619136,
        'W' => 18375280228818747392,
        'w' => 2162042286261665792,
        'X' => 14350737979939487744,
        'x' => 1227798289693278208,
        'Y' => 16145421229275217920,
        'y' => 4468983725207584768,
        'Z' => 13952593140407009280,
        'z' => 1232039144696315904,
        '0' => 4490450865206853632,
        '1' => 9160321642071588864,
        '2' => 3551445936313991168,
        '3' => 3911738330454163456,
        '4' => 9153575075096559616,
        '5' => 5064659835866316800,
        '6' => 452973816449073152,
        '7' => 8091916573708648448,
        '8' => 3911738330135396352,
        '9' => 4488199082338156544,
        '!' => 9007199254740992000,
        '?' => 3479106990762885120,
        '&' => 663798573871136768,
        '.' => 72057594037927936,
        ',' => 144396663052566528,
        '(' => 4701195061021376512,
        ')' => 4485866703837724672,
        ':' => 1297036692682702848,
        ';' => 1297318167659413504,
        '/' => 1155177711073755136,
        '+' => 289390378165993472,
        '-' => 289360691352043520,
        '=' => 1446803456760217600,
        '<' => 1227798246458392576,
        '>' => 291063817616490496,
        '_' => 72340172838010880,
        '#' => 729312793085345792,
        '%' => 1155740695386914816,
        '*' => 1227827976507228160,
        '$' => 2606035153054597120,
        // 'SmileyFace_1 4342219379529302590
        // 'FrownFace_1 434221499872950739
        '{' => 4342219379529302590,
        '}' => 434221499872950739,
        ' ' => 0,
        // Pattern to indicate we had no match for the given `char`
        _ => 0b11111111_10000001_10000001_10000001_10000001_10000001_10000001_11111111,
    };

    let char_as_columns = to_columns(char_as_u64);

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
        .flat_map(|cur_char| [char_to_columns(cur_char), vec![0]].into_iter().flatten())
        .collect()
}
