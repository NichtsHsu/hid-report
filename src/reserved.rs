use std::fmt::Display;

use crate::__data_size;

/// Items that are reserved for future use.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reserved([u8; 5]);

impl AsRef<[u8]> for Reserved {
    fn as_ref(&self) -> &[u8] {
        let end = __data_size(self.0[0]) + 1;
        &self.0[..end]
    }
}

impl Reserved {
    /// Create an item with size check.
    pub fn new(raw: &[u8]) -> Result<Self, crate::HidError> {
        if raw.is_empty() {
            return Err(crate::HidError::EmptyRawInput);
        };
        let expected = crate::__data_size(raw[0]);
        if expected + 1 != raw.len() {
            return Err(crate::HidError::DataSizeNotMatch {
                expected,
                provided: raw.len() - 1,
            });
        };
        let mut storage = [0; 5];
        storage[..raw.len()].copy_from_slice(raw);
        Ok(Self(storage))
    }

    /// Create an item without size check.
    ///
    /// # Safety
    ///
    /// Must ensure that the size part in the prefix is correct.
    pub unsafe fn new_unchecked(raw: &[u8]) -> Self {
        let mut storage = [0; 5];
        storage[..raw.len()].copy_from_slice(raw);
        Self(storage)
    }

    /// Get prefix part of the item. Equivalent to `item.as_ref()[0]`.
    pub fn prefix(&self) -> u8 {
        self.0[0]
    }

    /// Get data part of the item. Equivalent to `&item.as_ref()[1..]`.
    pub fn data(&self) -> &[u8] {
        let end = __data_size(self.0[0]) + 1;
        &self.0[1..end]
    }

    /// Set data part of the item.
    ///
    /// *NOTE*: data size must be: 0, 1, 2 or 4.
    pub fn set_data(&mut self, data: &[u8]) -> Result<&mut Self, crate::HidError> {
        crate::__set_data_size(&mut self.0[0], data)?;
        self.data_mut().copy_from_slice(data);
        Ok(self)
    }

    /// Get mutable data part of the item.
    pub fn data_mut(&mut self) -> &mut [u8] {
        let end = __data_size(self.0[0]) + 1;
        &mut self.0[1..end]
    }
}

impl Display for Reserved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.len() {
            1.. => write!(f, "Reserved"),
            0 => unreachable!(),
        }
    }
}
