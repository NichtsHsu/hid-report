use crate::{__data_to_signed, __data_to_unsigned, macros::*};
use alloc::vec::Vec;
use std::fmt::Display;

__impls_for_short_items! {
    /// Unsigned integer specifying the current Usage Page.
    ///
    /// Since a usage are 32 bit values, Usage
    /// Page items can be used to conserve space in a
    /// report descriptor by setting the high order 16 bits
    /// of a subsequent usages. Any usage that follows
    /// which is defines 16 bits or less is interpreted as a
    /// Usage ID and concatenated with the Usage Page
    /// to form a 32 bit Usage.
    ///
    /// # Data (Little Endian)
    ///
    /// * 0x00: Undefined
    /// * 0x01: Generic Desktop
    /// * 0x02: Simulation Controls
    /// * 0x03: VR Controls
    /// * 0x04: Sport Controls
    /// * 0x05: Game Controls
    /// * 0x06: Generic Device Controls
    /// * 0x07: Keyboard/Keypad
    /// * 0x08: LED
    /// * 0x09: Button
    /// * 0x0A: Ordinal
    /// * 0x0B: Telephony Device
    /// * 0x0C: Consumer
    /// * 0x0D: Digitizer
    /// * 0x0E: Haptics
    /// * 0x0F: PID
    /// * 0x10: Unicode
    /// * 0x12: Eye and Head Trackers
    /// * 0x14: Auxiliary Display
    /// * 0x20: Sensors
    /// * 0x40: Medical Instrument
    /// * 0x41: Braille Display
    /// * 0x59: Lighting And Illumination
    /// * 0x80-0x83: Monitor
    /// * 0x84-0x87: Power
    /// * 0x8C: Bar Code Scanner
    /// * 0x8D: Scale
    /// * 0x8E: Magnetic Stripe Reading
    /// * 0x8F: Reserved Point of Sale
    /// * 0x90: Camera Control
    /// * 0x91: Arcade
    /// * 0x92: Gaming Device
    /// * 0xF1D0: FIDO Alliance
    /// * 0xFF00-x0FFFF: Vendor Defined
    /// * Other: Reserved
    UsagePage: 0b0000_0100;
    /// Extent value in logical units. This is the
    /// minimum value that a variable or array item will
    /// report.
    ///
    /// For example, a mouse reporting x
    /// position values from 0 to 128 would have a
    /// Logical Minimum of 0 and a [Logical Maximum](LogicalMaximum)
    /// of 128.
    LogicalMinimum: 0b0001_0100;
    /// Extent value in logical units. This is the
    /// maximum value that a variable or array item will
    /// report.
    ///
    /// For example, a mouse reporting x
    /// position values from 0 to 128 would have a
    /// [Logical Minimum](LogicalMinimum) of 0 and a Logical Maximum
    /// of 128.
    LogicalMaximum: 0b0010_0100;
    /// Minimum value for the physical extent of a variable item.
    /// This represents the [Logical Minimum](LogicalMinimum)
    /// with units applied to it.
    PhysicalMinimum: 0b0011_0100;
    /// Maximum value for the physical extent of a variable item.
    /// This represents the [Logical Maximum](LogicalMaximum)
    /// with units applied to it.
    PhysicalMaximum: 0b0100_0100;
    /// Value of the unit exponent in base 10.
    ///
    /// # Data (Little Endian)
    ///
    /// * 0x0: 0
    /// * 0x1: 1
    /// * 0x2: 2
    /// * 0x3: 3
    /// * 0x4: 4
    /// * 0x5: 5
    /// * 0x6: 6
    /// * 0x7: 7
    /// * 0x8: -8
    /// * 0x9: -7
    /// * 0xA: -6
    /// * 0xB: -5
    /// * 0xC: -4
    /// * 0xD: -3
    /// * 0xE: -2
    /// * 0xF: -1
    UnitExponent: 0b0101_0100;
    /// Unit values.
    ///
    /// # Data (Little Endian)
    ///
    /// The Unit item qualifies value in the unit of [nibbles](https://en.wikipedia.org/wiki/Nibble).
    /// i.e., bit 3-0 is the nibble 0, bit 7-4 is the nibble 1, and so on.
    ///
    /// | Nibble | System | 0 | 1 | 2 | 3 | 4 |
    /// | --- | --- | --- | --- | --- | --- | --- |
    /// | 0 | System | None | SI Linear | SI Rotation | English Linear | English Rotation |
    /// | 1 | Length | None | Centimeter | Radians | Inch | Degrees |
    /// | 2 | Mass | None | Gram | Gram | Slug | Slug |
    /// | 3 | Time | None | Seconds | Seconds | Seconds | Seconds |
    /// | 4 | Temperature | None | Kelvin | Kelvin | Fahrenheit | Fahrenheit |
    /// | 5 | Current | None | Ampere | Ampere | Ampere | Ampere |
    /// | 6 | Luminous Intensity | None | Candela | Candela | Candela | Candela |
    ///
    /// Codes 0x5-0xE are reserved; code 0xF is vendor-defined.
    Unit: 0b0110_0100;
    /// Unsigned integer specifying the size of the report
    /// fields in bits.
    ///
    /// This allows the parser to build an
    /// item map for the report handler to use.
    ReportSize: 0b0111_0100;
    /// Unsigned value that specifies the Report ID.
    ///
    /// If a Report ID tag is used anywhere in Report
    /// descriptor, all data reports for the device are
    /// preceded by a single byte ID field. All items
    /// succeeding the first Report ID tag but preceding
    /// a second Report ID tag are included in a report
    /// prefixed by a 1-byte ID. All items succeeding the
    /// second but preceding a third Report ID tag are
    /// included in a second report prefixed by a second
    /// ID, and so on.
    ///
    /// This Report ID value indicates the prefix added
    /// to a particular report. For example, a Report
    /// descriptor could define a 3-byte report with a
    /// Report ID of `01`. This device would generate a
    /// 4-byte data report in which the first byte is `01`.
    /// The device may also generate other reports, each
    /// with a unique ID. This allows the host to
    /// distinguish different types of reports arriving
    /// over a single interrupt in pipe. And allows the
    /// device to distinguish different types of reports
    /// arriving over a single interrupt out pipe. Report
    /// ID zero is reserved and should not be used.
    ReportId: 0b1000_0100;
    /// Unsigned integer specifying the number of data
    /// fields for the item; determines how many fields
    /// are included in the report for this particular item
    /// (and consequently how many bits are added to
    /// the report).
    ReportCount: 0b1001_0100;
    /// Places a copy of the global item state table on the stack.
    Push: 0b1010_0100;
    /// Replaces the item state table with the top structure from the stack.
    Pop: 0b1011_0100;
}

impl Display for UsagePage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Usage Page"),
            1.. => write!(
                f,
                "Usage Page ({})",
                match __data_to_unsigned(self.data()) {
                    0x00 => "Undefined",
                    0x01 => "Generic Desktop",
                    0x02 => "Simulation Controls",
                    0x03 => "VR Controls",
                    0x04 => "Sport Controls",
                    0x05 => "Game Controls",
                    0x06 => "Generic Device Controls",
                    0x07 => "Keyboard/Keypad",
                    0x08 => "LED",
                    0x09 => "Button",
                    0x0A => "Ordinal",
                    0x0B => "Telephony Device",
                    0x0C => "Consumer",
                    0x0D => "Digitizers",
                    0x0E => "Haptics",
                    0x0F => "Physical Input Device",
                    0x10 => "Unicode",
                    0x11 => "SoC",
                    0x12 => "Eye and Head Trackers",
                    0x14 => "Auxiliary Display",
                    0x20 => "Sensors",
                    0x40 => "Medical Instrument",
                    0x41 => "Braille Display",
                    0x59 => "Lighting And Illumination",
                    0x80 => "Monitor",
                    0x81 => "Monitor Enumerated",
                    0x82 => "VESA Virtual Controls",
                    0x84 => "Power",
                    0x85 => "Battery System",
                    0x8C => "Bar Code Scanner",
                    0x8D => "Scale",
                    0x8E => "Magnetic Stripe Reading",
                    0x90 => "Camera Control",
                    0x91 => "Arcade",
                    0x92 => "Gaming Device",
                    0xF1D0 => "FIDO Alliance",
                    0xFF00..=0xFFFF => "Vendor Defined",
                    _ => "Reserved",
                }
            ),
        }
    }
}

impl Display for LogicalMinimum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Logical Minimum"),
            1.. => write!(f, "Logical Minimum ({})", __data_to_signed(self.data())),
        }
    }
}

impl Display for LogicalMaximum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Logical Maximum"),
            1.. => write!(f, "Logical Maximum ({})", __data_to_signed(self.data())),
        }
    }
}

impl Display for PhysicalMinimum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Physical Minimum"),
            1.. => write!(f, "Physical Minimum ({})", __data_to_signed(self.data())),
        }
    }
}

impl Display for PhysicalMaximum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Physical Maximum"),
            1.. => write!(f, "Physical Maximum ({})", __data_to_signed(self.data())),
        }
    }
}

impl Display for UnitExponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Unit Exponent"),
            1.. => match __data_to_signed(self.data()) {
                exp @ 0..=7 => write!(f, "Unit Exponent {exp}"),
                exp @ 8..=15 => write!(f, "Unit Exponent {}", exp - 16),
                _ => write!(f, "Unit Exponent"),
            },
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut units = Vec::new();
        if let [byte, ..] = self.data() {
            let system = byte & 0x0F;
            let length = (byte & 0xF0) >> 4;
            match system {
                1 => units.push("System: SI Linear"),
                2 => units.push("System: SI Rotation"),
                3 => units.push("System: English Linear"),
                4 => units.push("System: English Rotation"),
                5..=0xE => units.push("System: Reserved"),
                0xF => units.push("System: Vendor Defined"),
                _ => unreachable!(),
            }
            match length {
                1 => units.push("Length: Centimeter"),
                2 => units.push("Length: Radians"),
                3 => units.push("Length: Inch"),
                4 => units.push("Length: Degrees"),
                5..=0xE => units.push("Length: Reserved"),
                0xF => units.push("Length: Vendor Defined"),
                _ => unreachable!(),
            }
        }
        if let [_, byte, ..] = self.data() {
            let mass = byte & 0x0F;
            let time = (byte & 0xF0) >> 4;
            match mass {
                1 | 2 => units.push("Mass: Gram"),
                3 | 4 => units.push("Mass: Slug"),
                5..=0xE => units.push("Mass: Reserved"),
                0xF => units.push("Mass: Vendor Defined"),
                _ => unreachable!(),
            }
            match time {
                1..=4 => units.push("Time: Seconds"),
                5..=0xE => units.push("Time: Reserved"),
                0xF => units.push("Time: Vendor Defined"),
                _ => unreachable!(),
            }
        }
        if let [_, _, byte, ..] = self.data() {
            let temperature = byte & 0x0F;
            let current = (byte & 0xF0) >> 4;
            match temperature {
                1 | 2 => units.push("Temperature: Kelvin"),
                3 | 4 => units.push("Temperature: Fahrenheit"),
                5..=0xE => units.push("Temperature: Reserved"),
                0xF => units.push("Temperature: Vendor Defined"),
                _ => unreachable!(),
            }
            match current {
                1..=4 => units.push("Current: Ampere"),
                5..=0xE => units.push("Current: Reserved"),
                0xF => units.push("Current: Vendor Defined"),
                _ => unreachable!(),
            }
        }
        if let [_, _, _, byte, ..] = self.data() {
            let luminous_intensity = byte & 0x0F;
            match luminous_intensity {
                1..=4 => units.push("Luminous Intensity: Candela"),
                5..=0xE => units.push("Luminous Intensity: Reserved"),
                0xF => units.push("Luminous Intensity: Vendor Defined"),
                _ => unreachable!(),
            }
        }
        if units.is_empty() {
            write!(f, "Unit")
        } else {
            write!(f, "Unit({})", units.join(", "))
        }
    }
}

impl Display for ReportSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Report Size"),
            1.. => write!(f, "Report Size ({})", __data_to_unsigned(self.data())),
        }
    }
}

impl Display for ReportId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Report ID"),
            1.. => write!(f, "Report ID ({})", __data_to_unsigned(self.data())),
        }
    }
}

impl Display for ReportCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data().len() {
            0 => write!(f, "Report Count"),
            1.. => write!(f, "Report Count ({})", __data_to_unsigned(self.data())),
        }
    }
}

impl Display for Push {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Push")
    }
}

impl Display for Pop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pop")
    }
}
