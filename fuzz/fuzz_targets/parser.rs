#![no_main]

use std::sync::Once;

use libfuzzer_sys::fuzz_target;
use make_sys::make_main::initialize_stopchar_map;
use make_sys::parser::classify_line;

static INIT: Once = Once::new();

fuzz_target!(|data: &[u8]| {
    // The parser consults the global stopchar map; initialize it once.
    INIT.call_once(initialize_stopchar_map);

    // Feed the input through the line classifier the same way a real
    // makefile is parsed: one logical line at a time.
    for line in data.split(|&b| b == b'\n') {
        let _ = classify_line(line, false);
        let _ = classify_line(line, true);
    }
});
