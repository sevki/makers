// The build runs on a tokio runtime unconditionally (#598): there is one
// execution model in this codebase, not a synchronous one for the CLI and an
// asynchronous one for the server. A command-line `make` is simply the N=1
// tenant case — one tenant, one execution slot, the process exiting when it
// finishes — so it goes through the same `TenantRuntime` the server hands to
// every other tenant.
fn main() {
    let tenant = match make_sys::runtime::TenantRuntime::new() {
        Ok(tenant) => tenant,
        // Nothing has been read or built yet, so there is no output sink to
        // report through and nothing to clean up.
        Err(e) => {
            eprintln!("make: *** cannot start the runtime: {e}.  Stop.");
            std::process::exit(2);
        }
    };
    // The single process-exit point (Phase B, #432): the library reports how
    // the run ended; only this shim turns that into an exit status.
    std::process::exit(tenant.block_on(async { make_sys::make_main::main() }));
}
