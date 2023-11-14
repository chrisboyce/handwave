use bytemuck::cast;

use crate::display::Column;

pub fn to_columns(u: u64) -> [Column; 8] {
    cast(u)
}

pub fn from_columns(columns: [Column; 8]) -> u64 {
    cast(columns)
}
