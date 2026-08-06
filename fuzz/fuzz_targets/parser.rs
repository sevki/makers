#![no_main]

use std::sync::Once;

use libfuzzer_sys::fuzz_target;
use make_sys::makedb::MakeDb;
use make_sys::make_main::initialize_stopchar_map;
use make_sys::parser::classify_line;

static INIT: Once = Once::new();

fuzz_target!(|data: &[u8]| {
    // The parser consults the global stopchar map; initialize it once.
    INIT.call_once(initialize_stopchar_map);

    // A fresh database per input rather than one shared across the whole run:
    // `classify_line` interns AST nodes into it, so a long-lived database would
    // accumulate an entry per distinct input and grow without bound over a
    // fuzzing session.
    let db = MakeDb::default();

    // Feed the input through the line classifier the same way a real
    // makefile is parsed: one logical line at a time.
    for line in data.split(|&b| b == b'\n') {
        let _ = classify_line(&db, line, false);
        let _ = classify_line(&db, line, true);
    }
});
