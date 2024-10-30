This crate converts parsing strings into a u32 using `from_string()` and converts a u32
back to a parsing string using `to_string`. It also provides helper functions to update
and check the contents of the u32. It its tested against every parsing code found in the
Byzantine and Nestle 1904 text/data files.

There is no idiomatic rust "Parsed" type or struct in this code. But one may be added
in the future.

It is designed for tagging Biblical Greek, but it is also designed to be expandable to
support Biblical Hebrew.

This code is provided as is. You are free to use and distribute under the MIT license
under the condition that you acknowledge that you use this code at your own risk. No
warantee is given or implied.

This code may have bugs, but, it currently successfully in use in production, such as
for the https://scripturial.com/ project.

Rust Crate: https://crates.io/crates/bibleparsing
