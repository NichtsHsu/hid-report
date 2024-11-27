# USB HID report descriptor parser

## Example

```rust
use hid_report::{parse, pretty_print};

let bytes = [
    0x05, 0x0C, 0x09, 0x01, 0xA1, 0x01, 0x85, 0x02, 0x19,
    0x00, 0x2A, 0x3C, 0x02, 0x15, 0x00, 0x26, 0x3C, 0x02,
    0x95, 0x01, 0x75, 0x10, 0x81, 0x00, 0xC0,
];
let mut items = parse(bytes);
assert_eq!(items.next().unwrap().to_string(), "Usage Page (Consumer)");
assert_eq!(items.next().unwrap().to_string(), "Usage (Consumer Control)");
assert_eq!(items.next().unwrap().to_string(), "Collection (Application)");
assert_eq!(items.next().unwrap().to_string(), "Report ID (2)");
assert_eq!(items.next().unwrap().to_string(), "Usage Minimum (Undefined)");
assert_eq!(items.next().unwrap().to_string(), "Usage Maximum (AC Format)");
assert_eq!(items.next().unwrap().to_string(), "Logical Minimum (0)");
assert_eq!(items.next().unwrap().to_string(), "Logical Maximum (572)");
assert_eq!(items.next().unwrap().to_string(), "Report Count (1)");
assert_eq!(items.next().unwrap().to_string(), "Report Size (16)");
assert_eq!(
    items.next().unwrap().to_string(),
    "Input (Data, Array, Absolute, No Wrap, Linear, Preferred State, No Null Position)"
);
assert_eq!(items.next().unwrap().to_string(), "End Collection");
assert_eq!(items.next(), None);

let items = parse(bytes).collect::<Vec<_>>();

const EXPECTED: &str = indoc::indoc! {"
    0x05, 0x0C        // Usage Page (Consumer)
    0x09, 0x01        // Usage (Consumer Control)
    0xA1, 0x01        //   Collection (Application)
    0x85, 0x02        //   Report ID (2)
    0x19, 0x00        //   Usage Minimum (Undefined)
    0x2A, 0x3C, 0x02  //   Usage Maximum (AC Format)
    0x15, 0x00        //   Logical Minimum (0)
    0x26, 0x3C, 0x02  //   Logical Maximum (572)
    0x95, 0x01        //   Report Count (1)
    0x75, 0x10        //   Report Size (16)
    0x81, 0x00        //   Input (Data, Array, Absolute, No Wrap, Linear, Preferred State, No Null Position)
    0xC0              // End Collection"
};

assert_eq!(pretty_print(&items), EXPECTED);
```
