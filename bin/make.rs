// The build runs on a tokio runtime unconditionally (#598): there is one
// execution model in this codebase, not a synchronous one for the CLI and an
// asynchronous one for the server. A command-line `make` is simply the N=1
// tenant case of what the server does with many.
//
// `current_thread` is the behavior-preserving flavor for this slice: nothing
// in the engine is async yet, so a work-stealing pool would only add threads
// that never run anything. Choosing the tenant task shape for real — and with
// it the runtime flavor a tenant gets — is E0b.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // The single process-exit point (Phase B, #432): the library reports how
    // the run ended; only this shim turns that into an exit status.
    std::process::exit(make_sys::make_main::main());
}
