#![macro_use]

#[macro_export]
macro_rules! test_case {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/", $fname)
    };
}

#[macro_export]
macro_rules! parsing_string {
    ($p:expr) => {
        from_string($p).expect("Parsing failed")
    };
}
