// The build runs on a tokio runtime unconditionally (#598): there is one
// execution model in this codebase, not a synchronous one for the CLI and an
// asynchronous one for the server. A command-line `make` is simply the N=1
// tenant case — one tenant, one execution slot, the process exiting when it
// finishes — so it goes through the same slot the server hands every other
// tenant, and this shim stays the single `std::process::exit` point
// (Phase B, #432).
fn main() {
    std::process::exit(make_sys::runtime::run_tenant(|slot| {
        slot.block_on(async { make_sys::make_main::main() })
    }));
}
