use crate::macros::*;
use std::fmt::Display;

__impls_for_short_items! {
    /// Refers to the data from one or more similar controls on a device.
    ///
    /// For example, variable data such as reading the position of a single axis
    /// or a group of levers or array data such as one or more push buttons or
    /// switches.
    ///
    /// # Data (Little-Endian)
    ///
    /// * Bit 0: Data(0) | Constant(1)
    /// * Bit 1: Array(0) | Variable(1)
    /// * Bit 2: Absolute(0) | Relative(1)
    /// * Bit 3: No Wrap(0) | Wrap(1)
    /// * Bit 4: Linear(0) | Non Linear(1)
    /// * Bit 5: Preferred State(0) | No Preferred(1)
    /// * Bit 6: No Null Position(0) | Null State(1)
    /// * Bit 7: Reserved
    /// * Bit 8: Bit Field(0) | Buffered Bytes(1)
    /// * Bit 31-9: Reserved
    Input: 0b1000_0000;
    /// Refers to the data to one or more similar controls on a device
    /// such as setting the position of a single axis or a group of levers (variable data).
    /// Or, it can represent data to one or more LEDs (array data).
    ///
    /// # Data (Little-Endian)
    ///
    /// * Bit 0: Data(0) | Constant(1)
    /// * Bit 1: Array(0) | Variable(1)
    /// * Bit 2: Absolute(0) | Relative(1)
    /// * Bit 3: No Wrap(0) | Wrap(1)
    /// * Bit 4: Linear(0) | Non Linear(1)
    /// * Bit 5: Preferred State(0) | No Preferred(1)
    /// * Bit 6: No Null Position(0) | Null State(1)
    /// * Bit 7: Non Volatile(0) | Volatile(1)
    /// * Bit 8: Bit Field(0) | Buffered Bytes(1)
    /// * Bit 31-9: Reserved
    Output: 0b1001_0000;
    /// Describes device input and output not intended for
    /// consumption by the end user.
    ///
    /// For example, a software feature or Control Panel toggle.
    ///
    /// # Data (Little-Endian)
    ///
    /// * Bit 0: Data(0) | Constant(1)
    /// * Bit 1: Array(0) | Variable(1)
    /// * Bit 2: Absolute(0) | Relative(1)
    /// * Bit 3: No Wrap(0) | Wrap(1)
    /// * Bit 4: Linear(0) | Non Linear(1)
    /// * Bit 5: Preferred State(0) | No Preferred(1)
    /// * Bit 6: No Null Position(0) | Null State(1)
    /// * Bit 7: Non Volatile(0) | Volatile(1)
    /// * Bit 8: Bit Field(0) | Buffered Bytes(1)
    /// * Bit 31-9: Reserved
    Feature: 0b1011_0000;
    /// A meaningful grouping of [Input], [Output], and [Feature] items.
    ///
    /// For example, mouse, keyboard, joystick, and pointer.
    ///
    /// # Data (Little-Endian)
    ///
    /// * 0x00: Physical
    /// * 0x01: Application
    /// * 0x02: Logical
    /// * 0x03: Report
    /// * 0x04: Named Array
    /// * 0x05: Usage Switch
    /// * 0x06: Usage Modifier
    /// * 0x07-0x7F: Reserved
    /// * 0x80-0xFF: Vendor Defined
    Collection: 0b1010_0000;
    /// A terminating item used to specify the end of a
    /// [collection](Collection) of items.
    EndCollection: 0b1100_0000;
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Input"),
            1 => write!(
                f,
                "Input ({}, {}, {}, {}, {}, {}, {})",
                __matches_bit!(self.data()[0], 0, "Data", "Constant"),
                __matches_bit!(self.data()[0], 1, "Array", "Variable"),
                __matches_bit!(self.data()[0], 2, "Absolute", "Relative"),
                __matches_bit!(self.data()[0], 3, "No Wrap", "Wrap"),
                __matches_bit!(self.data()[0], 4, "Linear", "Non Linear"),
                __matches_bit!(self.data()[0], 5, "Preferred State", "No Preferred"),
                __matches_bit!(self.data()[0], 6, "No Null Position", "Null State"),
            ),
            2.. => write!(
                f,
                "Input ({}, {}, {}, {}, {}, {}, {}, {})",
                __matches_bit!(self.data()[0], 0, "Data", "Constant"),
                __matches_bit!(self.data()[0], 1, "Array", "Variable"),
                __matches_bit!(self.data()[0], 2, "Absolute", "Relative"),
                __matches_bit!(self.data()[0], 3, "No Wrap", "Wrap"),
                __matches_bit!(self.data()[0], 4, "Linear", "Non Linear"),
                __matches_bit!(self.data()[0], 5, "Preferred State", "No Preferred"),
                __matches_bit!(self.data()[0], 6, "No Null Position", "Null State"),
                __matches_bit!(self.data()[1], 0, "Bit Field", "Buffered Bytes"),
            ),
        }
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Output"),
            1 => write!(
                f,
                "Output ({}, {}, {}, {}, {}, {}, {}, {})",
                __matches_bit!(self.data()[0], 0, "Data", "Constant"),
                __matches_bit!(self.data()[0], 1, "Array", "Variable"),
                __matches_bit!(self.data()[0], 2, "Absolute", "Relative"),
                __matches_bit!(self.data()[0], 3, "No Wrap", "Wrap"),
                __matches_bit!(self.data()[0], 4, "Linear", "Non Linear"),
                __matches_bit!(self.data()[0], 5, "Preferred State", "No Preferred"),
                __matches_bit!(self.data()[0], 6, "No Null Position", "Null State"),
                __matches_bit!(self.data()[0], 7, "Non Volatile", "Volatile"),
            ),
            2.. => write!(
                f,
                "Output ({}, {}, {}, {}, {}, {}, {}, {}, {})",
                __matches_bit!(self.data()[0], 0, "Data", "Constant"),
                __matches_bit!(self.data()[0], 1, "Array", "Variable"),
                __matches_bit!(self.data()[0], 2, "Absolute", "Relative"),
                __matches_bit!(self.data()[0], 3, "No Wrap", "Wrap"),
                __matches_bit!(self.data()[0], 4, "Linear", "Non Linear"),
                __matches_bit!(self.data()[0], 5, "Preferred State", "No Preferred"),
                __matches_bit!(self.data()[0], 6, "No Null Position", "Null State"),
                __matches_bit!(self.data()[0], 7, "Non Volatile", "Volatile"),
                __matches_bit!(self.data()[1], 0, "Bit Field", "Buffered Bytes"),
            ),
        }
    }
}

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Feature"),
            1 => write!(
                f,
                "Feature ({}, {}, {}, {}, {}, {}, {}, {})",
                __matches_bit!(self.data()[0], 0, "Data", "Constant"),
                __matches_bit!(self.data()[0], 1, "Array", "Variable"),
                __matches_bit!(self.data()[0], 2, "Absolute", "Relative"),
                __matches_bit!(self.data()[0], 3, "No Wrap", "Wrap"),
                __matches_bit!(self.data()[0], 4, "Linear", "Non Linear"),
                __matches_bit!(self.data()[0], 5, "Preferred State", "No Preferred"),
                __matches_bit!(self.data()[0], 6, "No Null Position", "Null State"),
                __matches_bit!(self.data()[0], 7, "Non Volatile", "Volatile"),
            ),
            2.. => write!(
                f,
                "Feature ({}, {}, {}, {}, {}, {}, {}, {}, {})",
                __matches_bit!(self.data()[0], 0, "Data", "Constant"),
                __matches_bit!(self.data()[0], 1, "Array", "Variable"),
                __matches_bit!(self.data()[0], 2, "Absolute", "Relative"),
                __matches_bit!(self.data()[0], 3, "No Wrap", "Wrap"),
                __matches_bit!(self.data()[0], 4, "Linear", "Non Linear"),
                __matches_bit!(self.data()[0], 5, "Preferred State", "No Preferred"),
                __matches_bit!(self.data()[0], 6, "No Null Position", "Null State"),
                __matches_bit!(self.data()[0], 7, "Non Volatile", "Volatile"),
                __matches_bit!(self.data()[1], 0, "Bit Field", "Buffered Bytes"),
            ),
        }
    }
}

impl Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Collection"),
            1.. => write!(
                f,
                "Collection ({})",
                match self.data()[0] {
                    0 => "Physical",
                    1 => "Application",
                    2 => "Logical",
                    3 => "Report",
                    4 => "Named Array",
                    5 => "Usage Switch",
                    6 => "Usage Modifier",
                    7..=0x7f => "Reserved",
                    0x80..=0xff => "Vendor Defined",
                }
            ),
        }
    }
}

impl Display for EndCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "End Collection")
    }
}
