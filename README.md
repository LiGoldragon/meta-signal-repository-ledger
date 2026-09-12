# meta-signal-repository-ledger

Meta Signal contract for repository ledger authority and configuration.

The contract carries repository registration and retirement, the fallback
spool directory policy, and per-repository mirror policy. The ordinary
peer-callable surface lives in `signal-repository-ledger`; runtime storage
lives in `repository-ledger`.

`ethos/signal.ethos` is the schema authority. `build.rs` regenerates it with
Ethos Zero and asserts the committed projection in `src/generated/signal.rs`
matches. `examples/canonical.datom` is the contract's canonical text, emitted
by the codec that reads it back.
