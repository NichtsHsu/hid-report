use crate::Reserved;

/// Error type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HidError {
    /// Unexpected data size for short item, must be 0, 1, 2 or 4.
    InvalidDataSize,
    /// Data size in prefix doesn't match provided data size.
    DataSizeNotMatch {
        /// Expected data size described in the prefix part.
        expected: usize,
        /// Provided data size.
        provided: usize,
    },
    /// Prefix doesn't match the item type.
    PrefixNotMatch,
    /// Raw input is empty.
    EmptyRawInput,
    /// Strict mode is set and reserved item is found.
    ReservedItem(Reserved),
}
