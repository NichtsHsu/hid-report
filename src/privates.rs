use crate::HidError;

pub(crate) fn __set_data_size(prefix: &mut u8, data: &[u8]) -> Result<(), HidError> {
    *prefix &= !0b11;
    match data.len() {
        0 => (),
        1 => *prefix |= 0b01,
        2 => *prefix |= 0b10,
        4 => *prefix |= 0b11,
        _ => return Err(HidError::InvalidDataSize),
    };
    Ok(())
}

pub(crate) fn __data_size(prefix: u8) -> usize {
    match prefix & 0b11 {
        0b00 => 0,
        0b01 => 1,
        0b10 => 2,
        0b11 => 4,
        _ => unreachable!(),
    }
}

pub(crate) fn __data_to_signed(data: &[u8]) -> i32 {
    match data {
        [] => 0,
        [a] => i8::from_le_bytes([*a]) as i32,
        [a, b] | [a, b, _] => i16::from_le_bytes([*a, *b]) as i32,
        [a, b, c, d, ..] => i32::from_le_bytes([*a, *b, *c, *d]),
    }
}

pub(crate) fn __data_to_unsigned(data: &[u8]) -> u32 {
    match data {
        [] => 0,
        [a] => u8::from_le_bytes([*a]) as u32,
        [a, b] | [a, b, _] => u16::from_le_bytes([*a, *b]) as u32,
        [a, b, c, d, ..] => u32::from_le_bytes([*a, *b, *c, *d]),
    }
}
