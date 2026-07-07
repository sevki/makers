fn main() {
    // The single process-exit point (Phase B, #432): the library reports how
    // the run ended; only this shim turns that into an exit status.
    std::process::exit(make_sys::make_main::main());
}
