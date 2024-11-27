macro_rules! __impls_for_short_items {
    ($(#[$outer:meta])* $item:ident: $prefix:literal;) => {
        $(#[$outer])*
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $item([u8; 5]);

        impl AsRef<[u8]> for $item {
            fn as_ref(&self) -> &[u8] {
                let end = crate::__data_size(self.0[0]) + 1;
                &self.0[..end]
            }
        }

        impl Default for $item {
            fn default() -> Self {
                Self([Self::PREFIX, 0, 0, 0, 0])
            }
        }

        impl $item {
            /// Prefix consists of tag(bit 7-4), type(bit 3-2) and size(bit 1-0).
            /// The "size" part is set to `00` in this constant value.
            pub const PREFIX: u8 = $prefix;

            /// Create an item with prefix check.
            pub fn new(raw: &[u8]) -> Result<Self, crate::HidError> {
                if raw.is_empty() { return Err(crate::HidError::EmptyRawInput) };
                if raw[0] & 0b1111_1100 != Self::PREFIX {
                    return Err(crate::HidError::PrefixNotMatch);
                }
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

            /// Create an item *WITHOUT* prefix check.
            ///
            /// # Safety
            ///
            /// Must ensure that the prefix part is correct.
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
                let end = crate::__data_size(self.0[0]) + 1;
                &self.0[1..end]
            }

            /// Create an item with specific data.
            ///
            /// *NOTE*: data size must be: 0, 1, 2 or 4.
            pub fn new_with(data: &[u8]) -> Result<Self, crate::HidError> {
                let mut item = Self([0; 5]);
                item.0[0] = $prefix;
                crate::__set_data_size(&mut item.0[0], data)?;
                item.data_mut().copy_from_slice(data);
                Ok(item)
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
                let end = crate::__data_size(self.0[0]) + 1;
                &mut self.0[1..end]
            }
        }
    };
    ($(#[$outer:meta])* $item:ident: $prefix:literal; $($rest:tt)*) => {
        __impls_for_short_items! { $(#[$outer])* $item: $prefix; }
        __impls_for_short_items! { $($rest)* }
    }
}

macro_rules! __matches_bit {
    ($field:expr, $pos:literal, $zero:literal, $one:literal) => {
        match $field & (1 << $pos) {
            0 => $zero,
            _ => $one,
        }
    };
}

pub(crate) use __impls_for_short_items;
pub(crate) use __matches_bit;
