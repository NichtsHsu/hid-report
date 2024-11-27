#![no_std]
#![deny(missing_docs)]

//! Parse USB HID report descriptors and pretty print them.
//!
//! # Example
//!
//! ```
//! use hid_report::{parse, pretty_print};
//!
//! let bytes = [
//!     0x05, 0x0C, 0x09, 0x01, 0xA1, 0x01, 0x85, 0x02, 0x19,
//!     0x00, 0x2A, 0x3C, 0x02, 0x15, 0x00, 0x26, 0x3C, 0x02,
//!     0x95, 0x01, 0x75, 0x10, 0x81, 0x00, 0xC0,
//! ];
//! let mut items = parse(bytes);
//! assert_eq!(items.next().unwrap().to_string(), "Usage Page (Consumer)");
//! assert_eq!(items.next().unwrap().to_string(), "Usage (Consumer Control)");
//! assert_eq!(items.next().unwrap().to_string(), "Collection (Application)");
//! assert_eq!(items.next().unwrap().to_string(), "Report ID (2)");
//! assert_eq!(items.next().unwrap().to_string(), "Usage Minimum (Undefined)");
//! assert_eq!(items.next().unwrap().to_string(), "Usage Maximum (AC Format)");
//! assert_eq!(items.next().unwrap().to_string(), "Logical Minimum (0)");
//! assert_eq!(items.next().unwrap().to_string(), "Logical Maximum (572)");
//! assert_eq!(items.next().unwrap().to_string(), "Report Count (1)");
//! assert_eq!(items.next().unwrap().to_string(), "Report Size (16)");
//! assert_eq!(
//!     items.next().unwrap().to_string(),
//!     "Input (Data, Array, Absolute, No Wrap, Linear, Preferred State, No Null Position)"
//! );
//! assert_eq!(items.next().unwrap().to_string(), "End Collection");
//! assert_eq!(items.next(), None);
//!
//! let items = parse(bytes).collect::<Vec<_>>();
//!
//! const EXPECTED: &str = indoc::indoc! {"
//!     0x05, 0x0C        // Usage Page (Consumer)
//!     0x09, 0x01        // Usage (Consumer Control)
//!     0xA1, 0x01        //   Collection (Application)
//!     0x85, 0x02        //   Report ID (2)
//!     0x19, 0x00        //   Usage Minimum (Undefined)
//!     0x2A, 0x3C, 0x02  //   Usage Maximum (AC Format)
//!     0x15, 0x00        //   Logical Minimum (0)
//!     0x26, 0x3C, 0x02  //   Logical Maximum (572)
//!     0x95, 0x01        //   Report Count (1)
//!     0x75, 0x10        //   Report Size (16)
//!     0x81, 0x00        //   Input (Data, Array, Absolute, No Wrap, Linear, Preferred State, No Null Position)
//!     0xC0              // End Collection"
//! };
//!
//! assert_eq!(pretty_print(&items), EXPECTED);
//! ```

extern crate alloc;
extern crate core as std;

mod error;
mod global_items;
mod local_items;
mod macros;
mod main_items;
mod privates;
mod reserved;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use std::fmt::Display;

pub use error::*;
pub use global_items::*;
pub use local_items::*;
pub use main_items::*;
pub(crate) use privates::*;
pub use reserved::*;

/// Report items enumeration.
///
/// The HID Report descriptor is made up of items that provide information
/// about the device.
///
/// All items contain a 1-byte prefix which denotes the basic type of the item. The
/// HID class defines two basic formats for items:
///
/// * Short items: 1–5 bytes total length; used for the most commonly occurring
///   items. A short item typically contains 1 or 0 bytes of optional data.
/// * Long items: 3–258 bytes in length; used for items that require larger data
///   structures for parts.
///
/// NOTE: No long item tags are defined, these tags are reserved for future use.
///
/// The short item format packs the item size, type, and tag into the first byte. The
/// first byte may be followed by 0, 1, 2, or 4 optional data bytes depending on the
/// size of the data.
///
/// | Bit -8 | Bit 7-4 | Bit 3-2 | Bit 1-0 |
/// | --- | --- | --- | --- |
/// | \[data] | bTag | bType | bSize |
///
/// bSize: Numeric expression specifying size of data:
///
/// | bSize | size of data |
/// | --- | --- |
/// | 0 | 0 bytes |
/// | 1 | 1 byte |
/// | 2 | 2 bytes |
/// | 3 | 4 bytes |
///
/// bType: Numeric expression identifying type of item where:
///
/// | bType | type of item |
/// | --- | --- |
/// | 0 | Main |
/// | 1 | Global |
/// | 2 | Local |
/// | 3 | Reserved |
///
/// bTag: Numeric expression specifying the function of the item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReportItem {
    /// An [Input] item.
    Input(Input),
    /// An [Output] item.
    Output(Output),
    /// A [Feature] item.
    Feature(Feature),
    /// A [Collection] item.
    Collection(Collection),
    /// An [EndCollection] item.
    EndCollection(EndCollection),
    /// A [UsagePage] item.
    UsagePage(UsagePage),
    /// A [LogicalMinimum] item.
    LogicalMinimum(LogicalMinimum),
    /// A [LogicalMaximum] item.
    LogicalMaximum(LogicalMaximum),
    /// A [PhysicalMinimum] item.
    PhysicalMinimum(PhysicalMinimum),
    /// A [PhysicalMaximum] item.
    PhysicalMaximum(PhysicalMaximum),
    /// A [UnitExponent] item.
    UnitExponent(UnitExponent),
    /// A [Unit] item.
    Unit(Unit),
    /// A [ReportSize] item.
    ReportSize(ReportSize),
    /// A [ReportId] item.
    ReportId(ReportId),
    /// A [ReportCount] item.
    ReportCount(ReportCount),
    /// A [Push] item.
    Push(Push),
    /// A [Pop] item.
    Pop(Pop),
    /// A [Usage] item.
    Usage(Usage),
    /// A [UsageMinimum] item.
    UsageMinimum(UsageMinimum),
    /// A [UsageMaximum] item.
    UsageMaximum(UsageMaximum),
    /// A [DesignatorIndex] item.
    DesignatorIndex(DesignatorIndex),
    /// A [DesignatorMinimum] item.
    DesignatorMinimum(DesignatorMinimum),
    /// A [DesignatorMaximum] item.
    DesignatorMaximum(DesignatorMaximum),
    /// A [StringIndex] item.
    StringIndex(StringIndex),
    /// A [StringMinimum] item.
    StringMinimum(StringMinimum),
    /// A [StringMaximum] item.
    StringMaximum(StringMaximum),
    /// A [Delimiter] item.
    Delimiter(Delimiter),
    /// A [Reserved] item.
    Reserved(Reserved),
}

impl AsRef<[u8]> for ReportItem {
    fn as_ref(&self) -> &[u8] {
        match self {
            ReportItem::Input(inner) => inner.as_ref(),
            ReportItem::Output(inner) => inner.as_ref(),
            ReportItem::Feature(inner) => inner.as_ref(),
            ReportItem::Collection(inner) => inner.as_ref(),
            ReportItem::EndCollection(inner) => inner.as_ref(),
            ReportItem::UsagePage(inner) => inner.as_ref(),
            ReportItem::LogicalMinimum(inner) => inner.as_ref(),
            ReportItem::LogicalMaximum(inner) => inner.as_ref(),
            ReportItem::PhysicalMinimum(inner) => inner.as_ref(),
            ReportItem::PhysicalMaximum(inner) => inner.as_ref(),
            ReportItem::UnitExponent(inner) => inner.as_ref(),
            ReportItem::Unit(inner) => inner.as_ref(),
            ReportItem::ReportSize(inner) => inner.as_ref(),
            ReportItem::ReportId(inner) => inner.as_ref(),
            ReportItem::ReportCount(inner) => inner.as_ref(),
            ReportItem::Push(inner) => inner.as_ref(),
            ReportItem::Pop(inner) => inner.as_ref(),
            ReportItem::Usage(inner) => inner.as_ref(),
            ReportItem::UsageMinimum(inner) => inner.as_ref(),
            ReportItem::UsageMaximum(inner) => inner.as_ref(),
            ReportItem::DesignatorIndex(inner) => inner.as_ref(),
            ReportItem::DesignatorMinimum(inner) => inner.as_ref(),
            ReportItem::DesignatorMaximum(inner) => inner.as_ref(),
            ReportItem::StringIndex(inner) => inner.as_ref(),
            ReportItem::StringMinimum(inner) => inner.as_ref(),
            ReportItem::StringMaximum(inner) => inner.as_ref(),
            ReportItem::Delimiter(inner) => inner.as_ref(),
            ReportItem::Reserved(inner) => inner.as_ref(),
        }
    }
}

impl Display for ReportItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportItem::Input(inner) => inner.fmt(f),
            ReportItem::Output(inner) => inner.fmt(f),
            ReportItem::Feature(inner) => inner.fmt(f),
            ReportItem::Collection(inner) => inner.fmt(f),
            ReportItem::EndCollection(inner) => inner.fmt(f),
            ReportItem::UsagePage(inner) => inner.fmt(f),
            ReportItem::LogicalMinimum(inner) => inner.fmt(f),
            ReportItem::LogicalMaximum(inner) => inner.fmt(f),
            ReportItem::PhysicalMinimum(inner) => inner.fmt(f),
            ReportItem::PhysicalMaximum(inner) => inner.fmt(f),
            ReportItem::UnitExponent(inner) => inner.fmt(f),
            ReportItem::Unit(inner) => inner.fmt(f),
            ReportItem::ReportSize(inner) => inner.fmt(f),
            ReportItem::ReportId(inner) => inner.fmt(f),
            ReportItem::ReportCount(inner) => inner.fmt(f),
            ReportItem::Push(inner) => inner.fmt(f),
            ReportItem::Pop(inner) => inner.fmt(f),
            ReportItem::Usage(inner) => inner.fmt(f),
            ReportItem::UsageMinimum(inner) => inner.fmt(f),
            ReportItem::UsageMaximum(inner) => inner.fmt(f),
            ReportItem::DesignatorIndex(inner) => inner.fmt(f),
            ReportItem::DesignatorMinimum(inner) => inner.fmt(f),
            ReportItem::DesignatorMaximum(inner) => inner.fmt(f),
            ReportItem::StringIndex(inner) => inner.fmt(f),
            ReportItem::StringMinimum(inner) => inner.fmt(f),
            ReportItem::StringMaximum(inner) => inner.fmt(f),
            ReportItem::Delimiter(inner) => inner.fmt(f),
            ReportItem::Reserved(inner) => inner.fmt(f),
        }
    }
}

impl ReportItem {
    /// Create a new item from raw byte stream.
    ///
    /// Items that cannot be recognized will be treated as [`Reserved`](ReportItem::Reserved).
    /// If you want to fail on unknown items, use [`new_strict()`](ReportItem::new_strict()) instead.
    ///
    /// # Example
    ///
    /// ```
    /// use hid_report::ReportItem;
    ///
    /// let raw = [0x26, 0x3c, 0x02];
    /// let item = ReportItem::new(&raw).unwrap();
    /// assert!(matches!(item, ReportItem::LogicalMaximum(_)));
    /// assert_eq!(item.to_string(), "Logical Maximum (572)");
    /// ```
    pub fn new(raw: &[u8]) -> Result<Self, HidError> {
        if raw.is_empty() {
            return Err(HidError::EmptyRawInput);
        };
        let expected = __data_size(raw[0]);
        if expected + 1 != raw.len() {
            return Err(HidError::DataSizeNotMatch {
                expected,
                provided: raw.len() - 1,
            });
        };
        unsafe {
            Ok(match raw[0] & 0b1111_1100 {
                Input::PREFIX => ReportItem::Input(Input::new_unchecked(raw)),
                Output::PREFIX => ReportItem::Output(Output::new_unchecked(raw)),
                Feature::PREFIX => ReportItem::Feature(Feature::new_unchecked(raw)),
                Collection::PREFIX => ReportItem::Collection(Collection::new_unchecked(raw)),
                EndCollection::PREFIX => {
                    ReportItem::EndCollection(EndCollection::new_unchecked(raw))
                }
                UsagePage::PREFIX => ReportItem::UsagePage(UsagePage::new_unchecked(raw)),
                LogicalMinimum::PREFIX => {
                    ReportItem::LogicalMinimum(LogicalMinimum::new_unchecked(raw))
                }
                LogicalMaximum::PREFIX => {
                    ReportItem::LogicalMaximum(LogicalMaximum::new_unchecked(raw))
                }
                PhysicalMinimum::PREFIX => {
                    ReportItem::PhysicalMinimum(PhysicalMinimum::new_unchecked(raw))
                }
                PhysicalMaximum::PREFIX => {
                    ReportItem::PhysicalMaximum(PhysicalMaximum::new_unchecked(raw))
                }
                UnitExponent::PREFIX => ReportItem::UnitExponent(UnitExponent::new_unchecked(raw)),
                Unit::PREFIX => ReportItem::Unit(Unit::new_unchecked(raw)),
                ReportSize::PREFIX => ReportItem::ReportSize(ReportSize::new_unchecked(raw)),
                ReportId::PREFIX => ReportItem::ReportId(ReportId::new_unchecked(raw)),
                ReportCount::PREFIX => ReportItem::ReportCount(ReportCount::new_unchecked(raw)),
                Push::PREFIX => ReportItem::Push(Push::new_unchecked(raw)),
                Pop::PREFIX => ReportItem::Pop(Pop::new_unchecked(raw)),
                Usage::PREFIX => ReportItem::Usage(Usage::new_unchecked(raw)),
                UsageMinimum::PREFIX => ReportItem::UsageMinimum(UsageMinimum::new_unchecked(raw)),
                UsageMaximum::PREFIX => ReportItem::UsageMaximum(UsageMaximum::new_unchecked(raw)),
                DesignatorIndex::PREFIX => {
                    ReportItem::DesignatorIndex(DesignatorIndex::new_unchecked(raw))
                }
                DesignatorMinimum::PREFIX => {
                    ReportItem::DesignatorMinimum(DesignatorMinimum::new_unchecked(raw))
                }
                DesignatorMaximum::PREFIX => {
                    ReportItem::DesignatorMaximum(DesignatorMaximum::new_unchecked(raw))
                }
                StringIndex::PREFIX => ReportItem::StringIndex(StringIndex::new_unchecked(raw)),
                StringMinimum::PREFIX => {
                    ReportItem::StringMinimum(StringMinimum::new_unchecked(raw))
                }
                StringMaximum::PREFIX => {
                    ReportItem::StringMaximum(StringMaximum::new_unchecked(raw))
                }
                Delimiter::PREFIX => ReportItem::Delimiter(Delimiter::new_unchecked(raw)),
                _ => ReportItem::Reserved(Reserved::new_unchecked(raw)),
            })
        }
    }

    /// Create a new item from raw byte stream in strict mode.
    ///
    /// Items that cannot be recognized will be treated as [`HidError::ReservedItem`].
    pub fn new_strict(raw: &[u8]) -> Result<Self, crate::HidError> {
        if raw.is_empty() {
            return Err(crate::HidError::EmptyRawInput);
        };
        let expected = __data_size(raw[0]);
        if expected + 1 != raw.len() {
            return Err(HidError::DataSizeNotMatch {
                expected,
                provided: raw.len() - 1,
            });
        };
        unsafe {
            Ok(match raw[0] & 0b1111_1100 {
                Input::PREFIX => ReportItem::Input(Input::new_unchecked(raw)),
                Output::PREFIX => ReportItem::Output(Output::new_unchecked(raw)),
                Feature::PREFIX => ReportItem::Feature(Feature::new_unchecked(raw)),
                Collection::PREFIX => ReportItem::Collection(Collection::new_unchecked(raw)),
                EndCollection::PREFIX => {
                    ReportItem::EndCollection(EndCollection::new_unchecked(raw))
                }
                UsagePage::PREFIX => ReportItem::UsagePage(UsagePage::new_unchecked(raw)),
                LogicalMinimum::PREFIX => {
                    ReportItem::LogicalMinimum(LogicalMinimum::new_unchecked(raw))
                }
                LogicalMaximum::PREFIX => {
                    ReportItem::LogicalMaximum(LogicalMaximum::new_unchecked(raw))
                }
                PhysicalMinimum::PREFIX => {
                    ReportItem::PhysicalMinimum(PhysicalMinimum::new_unchecked(raw))
                }
                PhysicalMaximum::PREFIX => {
                    ReportItem::PhysicalMaximum(PhysicalMaximum::new_unchecked(raw))
                }
                UnitExponent::PREFIX => ReportItem::UnitExponent(UnitExponent::new_unchecked(raw)),
                Unit::PREFIX => ReportItem::Unit(Unit::new_unchecked(raw)),
                ReportSize::PREFIX => ReportItem::ReportSize(ReportSize::new_unchecked(raw)),
                ReportId::PREFIX => ReportItem::ReportId(ReportId::new_unchecked(raw)),
                ReportCount::PREFIX => ReportItem::ReportCount(ReportCount::new_unchecked(raw)),
                Push::PREFIX => ReportItem::Push(Push::new_unchecked(raw)),
                Pop::PREFIX => ReportItem::Pop(Pop::new_unchecked(raw)),
                Usage::PREFIX => ReportItem::Usage(Usage::new_unchecked(raw)),
                UsageMinimum::PREFIX => ReportItem::UsageMinimum(UsageMinimum::new_unchecked(raw)),
                UsageMaximum::PREFIX => ReportItem::UsageMaximum(UsageMaximum::new_unchecked(raw)),
                DesignatorIndex::PREFIX => {
                    ReportItem::DesignatorIndex(DesignatorIndex::new_unchecked(raw))
                }
                DesignatorMinimum::PREFIX => {
                    ReportItem::DesignatorMinimum(DesignatorMinimum::new_unchecked(raw))
                }
                DesignatorMaximum::PREFIX => {
                    ReportItem::DesignatorMaximum(DesignatorMaximum::new_unchecked(raw))
                }
                StringIndex::PREFIX => ReportItem::StringIndex(StringIndex::new_unchecked(raw)),
                StringMinimum::PREFIX => {
                    ReportItem::StringMinimum(StringMinimum::new_unchecked(raw))
                }
                StringMaximum::PREFIX => {
                    ReportItem::StringMaximum(StringMaximum::new_unchecked(raw))
                }
                Delimiter::PREFIX => ReportItem::Delimiter(Delimiter::new_unchecked(raw)),
                _ => return Err(HidError::ReservedItem(Reserved::new_unchecked(raw))),
            })
        }
    }

    /// Create a new item from raw byte stream, without checking data length.
    ///
    /// Items that cannot be recognized will be treated as [`Reserved`](ReportItem::Reserved).
    /// If you want to fail on unknown items, use
    /// [`new_strict_unchecked()`](ReportItem::new_strict_unchecked()) instead.
    ///
    /// # Safety
    ///
    /// You should ensure that the raw data is a valid HID report item.
    pub unsafe fn new_unchecked(raw: &[u8]) -> Self {
        match raw[0] & 0b1111_1100 {
            Input::PREFIX => ReportItem::Input(Input::new_unchecked(raw)),
            Output::PREFIX => ReportItem::Output(Output::new_unchecked(raw)),
            Feature::PREFIX => ReportItem::Feature(Feature::new_unchecked(raw)),
            Collection::PREFIX => ReportItem::Collection(Collection::new_unchecked(raw)),
            EndCollection::PREFIX => ReportItem::EndCollection(EndCollection::new_unchecked(raw)),
            UsagePage::PREFIX => ReportItem::UsagePage(UsagePage::new_unchecked(raw)),
            LogicalMinimum::PREFIX => {
                ReportItem::LogicalMinimum(LogicalMinimum::new_unchecked(raw))
            }
            LogicalMaximum::PREFIX => {
                ReportItem::LogicalMaximum(LogicalMaximum::new_unchecked(raw))
            }
            PhysicalMinimum::PREFIX => {
                ReportItem::PhysicalMinimum(PhysicalMinimum::new_unchecked(raw))
            }
            PhysicalMaximum::PREFIX => {
                ReportItem::PhysicalMaximum(PhysicalMaximum::new_unchecked(raw))
            }
            UnitExponent::PREFIX => ReportItem::UnitExponent(UnitExponent::new_unchecked(raw)),
            Unit::PREFIX => ReportItem::Unit(Unit::new_unchecked(raw)),
            ReportSize::PREFIX => ReportItem::ReportSize(ReportSize::new_unchecked(raw)),
            ReportId::PREFIX => ReportItem::ReportId(ReportId::new_unchecked(raw)),
            ReportCount::PREFIX => ReportItem::ReportCount(ReportCount::new_unchecked(raw)),
            Push::PREFIX => ReportItem::Push(Push::new_unchecked(raw)),
            Pop::PREFIX => ReportItem::Pop(Pop::new_unchecked(raw)),
            Usage::PREFIX => ReportItem::Usage(Usage::new_unchecked(raw)),
            UsageMinimum::PREFIX => ReportItem::UsageMinimum(UsageMinimum::new_unchecked(raw)),
            UsageMaximum::PREFIX => ReportItem::UsageMaximum(UsageMaximum::new_unchecked(raw)),
            DesignatorIndex::PREFIX => {
                ReportItem::DesignatorIndex(DesignatorIndex::new_unchecked(raw))
            }
            DesignatorMinimum::PREFIX => {
                ReportItem::DesignatorMinimum(DesignatorMinimum::new_unchecked(raw))
            }
            DesignatorMaximum::PREFIX => {
                ReportItem::DesignatorMaximum(DesignatorMaximum::new_unchecked(raw))
            }
            StringIndex::PREFIX => ReportItem::StringIndex(StringIndex::new_unchecked(raw)),
            StringMinimum::PREFIX => ReportItem::StringMinimum(StringMinimum::new_unchecked(raw)),
            StringMaximum::PREFIX => ReportItem::StringMaximum(StringMaximum::new_unchecked(raw)),
            Delimiter::PREFIX => ReportItem::Delimiter(Delimiter::new_unchecked(raw)),
            _ => ReportItem::Reserved(Reserved::new_unchecked(raw)),
        }
    }

    /// Create a new item from raw byte stream in strict mode, without checking data length.
    ///
    /// Items that cannot be recognized will be treated as [`HidError::ReservedItem`].
    /// Also, this is the only error that may be reported.
    ///
    /// # Safety
    ///
    /// You should ensure that the raw data is a valid HID report item.
    pub unsafe fn new_strict_unchecked(raw: &[u8]) -> Result<Self, HidError> {
        Ok(match raw[0] & 0b1111_1100 {
            Input::PREFIX => ReportItem::Input(Input::new_unchecked(raw)),
            Output::PREFIX => ReportItem::Output(Output::new_unchecked(raw)),
            Feature::PREFIX => ReportItem::Feature(Feature::new_unchecked(raw)),
            Collection::PREFIX => ReportItem::Collection(Collection::new_unchecked(raw)),
            EndCollection::PREFIX => ReportItem::EndCollection(EndCollection::new_unchecked(raw)),
            UsagePage::PREFIX => ReportItem::UsagePage(UsagePage::new_unchecked(raw)),
            LogicalMinimum::PREFIX => {
                ReportItem::LogicalMinimum(LogicalMinimum::new_unchecked(raw))
            }
            LogicalMaximum::PREFIX => {
                ReportItem::LogicalMaximum(LogicalMaximum::new_unchecked(raw))
            }
            PhysicalMinimum::PREFIX => {
                ReportItem::PhysicalMinimum(PhysicalMinimum::new_unchecked(raw))
            }
            PhysicalMaximum::PREFIX => {
                ReportItem::PhysicalMaximum(PhysicalMaximum::new_unchecked(raw))
            }
            UnitExponent::PREFIX => ReportItem::UnitExponent(UnitExponent::new_unchecked(raw)),
            Unit::PREFIX => ReportItem::Unit(Unit::new_unchecked(raw)),
            ReportSize::PREFIX => ReportItem::ReportSize(ReportSize::new_unchecked(raw)),
            ReportId::PREFIX => ReportItem::ReportId(ReportId::new_unchecked(raw)),
            ReportCount::PREFIX => ReportItem::ReportCount(ReportCount::new_unchecked(raw)),
            Push::PREFIX => ReportItem::Push(Push::new_unchecked(raw)),
            Pop::PREFIX => ReportItem::Pop(Pop::new_unchecked(raw)),
            Usage::PREFIX => ReportItem::Usage(Usage::new_unchecked(raw)),
            UsageMinimum::PREFIX => ReportItem::UsageMinimum(UsageMinimum::new_unchecked(raw)),
            UsageMaximum::PREFIX => ReportItem::UsageMaximum(UsageMaximum::new_unchecked(raw)),
            DesignatorIndex::PREFIX => {
                ReportItem::DesignatorIndex(DesignatorIndex::new_unchecked(raw))
            }
            DesignatorMinimum::PREFIX => {
                ReportItem::DesignatorMinimum(DesignatorMinimum::new_unchecked(raw))
            }
            DesignatorMaximum::PREFIX => {
                ReportItem::DesignatorMaximum(DesignatorMaximum::new_unchecked(raw))
            }
            StringIndex::PREFIX => ReportItem::StringIndex(StringIndex::new_unchecked(raw)),
            StringMinimum::PREFIX => ReportItem::StringMinimum(StringMinimum::new_unchecked(raw)),
            StringMaximum::PREFIX => ReportItem::StringMaximum(StringMaximum::new_unchecked(raw)),
            Delimiter::PREFIX => ReportItem::Delimiter(Delimiter::new_unchecked(raw)),
            _ => return Err(HidError::ReservedItem(Reserved::new_unchecked(raw))),
        })
    }

    /// Get prefix part of the item. Equivalent to `item.as_ref()[0]`.
    pub fn prefix(&self) -> u8 {
        self.as_ref()[0]
    }

    /// Get data part of the item. Equivalent to `&item.as_ref()[1..]`.
    pub fn data(&self) -> &[u8] {
        &self.as_ref()[1..]
    }
}

struct Iter<ByteStreamIter: Iterator<Item = u8>> {
    byte_stream_iter: ByteStreamIter,
    usage_page: Option<UsagePage>,
}

struct StrictIter<ByteStreamIter: Iterator<Item = u8>> {
    byte_stream_iter: ByteStreamIter,
    usage_page: Option<UsagePage>,
}

impl<ByteStreamIter: Iterator<Item = u8>> Iterator for Iter<ByteStreamIter> {
    type Item = ReportItem;
    fn next(&mut self) -> Option<Self::Item> {
        let prefix = self.byte_stream_iter.next()?;
        let size = __data_size(prefix);
        let mut storage = [0u8; 5];
        storage[0] = prefix;
        for i in 0..size {
            storage[i + 1] = self.byte_stream_iter.next()?;
        }
        let mut item = unsafe { ReportItem::new_unchecked(&storage) };
        if let ReportItem::UsagePage(usage_page) = &item {
            self.usage_page = Some(usage_page.clone());
        }
        if let Some(usage_page) = &self.usage_page {
            match &mut item {
                ReportItem::Usage(usage) => usage.set_usage_page(usage_page.clone()),
                ReportItem::UsageMinimum(usage_minimum) => {
                    usage_minimum.set_usage_page(usage_page.clone())
                }
                ReportItem::UsageMaximum(usage_maximum) => {
                    usage_maximum.set_usage_page(usage_page.clone())
                }
                _ => (),
            }
        }
        Some(item)
    }
}

impl<ByteStreamIter: Iterator<Item = u8>> Iterator for StrictIter<ByteStreamIter> {
    type Item = Result<ReportItem, HidError>;
    fn next(&mut self) -> Option<Self::Item> {
        let prefix = self.byte_stream_iter.next()?;
        let size = __data_size(prefix);
        let mut storage = [0u8; 5];
        storage[0] = prefix;
        for i in 0..size {
            storage[i + 1] = self.byte_stream_iter.next()?;
        }
        let mut item = unsafe { ReportItem::new_strict_unchecked(&storage) };
        if let Ok(ReportItem::UsagePage(usage_page)) = &item {
            self.usage_page = Some(usage_page.clone());
        }
        if let Some(usage_page) = &self.usage_page {
            match &mut item {
                Ok(ReportItem::Usage(usage)) => usage.set_usage_page(usage_page.clone()),
                Ok(ReportItem::UsageMinimum(usage_minimum)) => {
                    usage_minimum.set_usage_page(usage_page.clone())
                }
                Ok(ReportItem::UsageMaximum(usage_maximum)) => {
                    usage_maximum.set_usage_page(usage_page.clone())
                }
                _ => (),
            }
        }
        Some(item)
    }
}

/// Parse a byte stream into a report item iterator.
///
/// Items that cannot be recognized will be treated as [`Reserved`](ReportItem::Reserved).
/// If you want to fail on unknown items, use [`parse_strict()`](parse_strict()) instead.
///
/// # Example
///
/// ```
/// use hid_report::parse;
///
/// let bytes = [
///     0x05, 0x0C, 0x09, 0x01, 0xA1, 0x01, 0x85, 0x02, 0x19,
///     0x00, 0x2A, 0x3C, 0x02, 0x15, 0x00, 0x26, 0x3C, 0x02,
///     0x95, 0x01, 0x75, 0x10, 0x81, 0x00, 0xC0,
/// ];
/// let mut items = parse(bytes);
/// assert_eq!(items.next().unwrap().to_string(), "Usage Page (Consumer)");
/// assert_eq!(items.next().unwrap().to_string(), "Usage (Consumer Control)");
/// assert_eq!(items.next().unwrap().to_string(), "Collection (Application)");
/// assert_eq!(items.next().unwrap().to_string(), "Report ID (2)");
/// assert_eq!(items.next().unwrap().to_string(), "Usage Minimum (Undefined)");
/// assert_eq!(items.next().unwrap().to_string(), "Usage Maximum (AC Format)");
/// assert_eq!(items.next().unwrap().to_string(), "Logical Minimum (0)");
/// assert_eq!(items.next().unwrap().to_string(), "Logical Maximum (572)");
/// assert_eq!(items.next().unwrap().to_string(), "Report Count (1)");
/// assert_eq!(items.next().unwrap().to_string(), "Report Size (16)");
/// assert_eq!(
///     items.next().unwrap().to_string(),
///     "Input (Data, Array, Absolute, No Wrap, Linear, Preferred State, No Null Position)"
/// );
/// assert_eq!(items.next().unwrap().to_string(), "End Collection");
/// assert_eq!(items.next(), None);
/// ```
pub fn parse<ByteStream: IntoIterator<Item = u8>>(
    byte_stream: ByteStream,
) -> impl Iterator<Item = ReportItem> {
    Iter {
        byte_stream_iter: byte_stream.into_iter(),
        usage_page: None,
    }
}

/// Parse a byte stream into a report item iterator in strict mode.
///
/// Items that cannot be recognized will be treated as [`HidError::ReservedItem`].
/// Also, this is the only error that may be reported.
pub fn parse_strict<ByteStream: IntoIterator<Item = u8>>(
    byte_stream: ByteStream,
) -> impl Iterator<Item = Result<ReportItem, HidError>> {
    StrictIter {
        byte_stream_iter: byte_stream.into_iter(),
        usage_page: None,
    }
}

/// Dump items into a byte stream.
pub fn dump<'a, ItemStream: IntoIterator<Item = &'a ReportItem>>(
    item_stream: ItemStream,
) -> Vec<u8> {
    let mut v = Vec::new();
    for item in item_stream {
        v.extend_from_slice(item.as_ref());
    }
    v
}

/// Print items to string in a pretty way.
///
/// # Example
///
/// ```
/// use hid_report::{parse, pretty_print};
///
/// let bytes = [
///     0x05, 0x0C, 0x09, 0x01, 0xA1, 0x01, 0x85, 0x02, 0x19,
///     0x00, 0x2A, 0x3C, 0x02, 0x15, 0x00, 0x26, 0x3C, 0x02,
///     0x95, 0x01, 0x75, 0x10, 0x81, 0x00, 0xC0,
/// ];
/// let items = parse(bytes).collect::<Vec<_>>();
///
/// const EXPECTED: &str = indoc::indoc! {"
///     0x05, 0x0C        // Usage Page (Consumer)
///     0x09, 0x01        // Usage (Consumer Control)
///     0xA1, 0x01        //   Collection (Application)
///     0x85, 0x02        //   Report ID (2)
///     0x19, 0x00        //   Usage Minimum (Undefined)
///     0x2A, 0x3C, 0x02  //   Usage Maximum (AC Format)
///     0x15, 0x00        //   Logical Minimum (0)
///     0x26, 0x3C, 0x02  //   Logical Maximum (572)
///     0x95, 0x01        //   Report Count (1)
///     0x75, 0x10        //   Report Size (16)
///     0x81, 0x00        //   Input (Data, Array, Absolute, No Wrap, Linear, Preferred State, No Null Position)
///     0xC0              // End Collection"
/// };
///
/// assert_eq!(pretty_print(&items), EXPECTED);
/// ```
pub fn pretty_print<'a, ItemStream: IntoIterator<Item = &'a ReportItem>>(
    item_stream: ItemStream,
) -> String {
    let mut max_len = 0;
    let mut tmp = Vec::new();
    let mut tab: usize = 0;
    for item in item_stream {
        match item {
            ReportItem::Collection(_) | ReportItem::Push(_) => tab += 1,
            ReportItem::EndCollection(_) | ReportItem::Pop(_) => tab = tab.saturating_sub(1),
            _ => (),
        }
        max_len = std::cmp::max(max_len, item.as_ref().len());
        tmp.push((
            item.as_ref()
                .iter()
                .map(|byte| format!("{:#04X}", byte))
                .collect::<Vec<_>>()
                .join(", "),
            item.to_string(),
            tab * 2 + 1,
        ));
    }
    let width_of_raw = max_len * 6;
    tmp.into_iter()
        .map(|(raw, comment, tab)| format!("{:<width_of_raw$}//{:<tab$}{}", raw, ' ', comment))
        .collect::<Vec<_>>()
        .join("\n")
}
