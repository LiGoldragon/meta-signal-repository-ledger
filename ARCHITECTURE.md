# meta-signal-repository-ledger Architecture

`meta-signal-repository-ledger` is the meta Signal contract for the
`repository-ledger` component: the privileged authority and configuration
wire, reached over the daemon's meta socket.

The ordinary peer-callable surface — event submission and bounded reads —
lives in `signal-repository-ledger`. Storage lives in `repository-ledger`.

## Authority

`ethos/signal.ethos` is the schema authority. Ethos Zero generates
`src/generated/signal.rs` from it, the generated file is committed, and
`build.rs` asserts the committed projection against a fresh generation on
every build. No contract type is hand-written.

`examples/canonical.datom` is the contract's canonical text. Every line was
emitted by the codec that reads it back, and `tests/generated_contract.rs`
actualizes each line and asserts it re-renders to itself.

The portable rkyv frame, its three kinds and its four-byte big-endian length
prefix come from `signal`. One request is one frame carrying the rkyv archive
of `Query`; one reply is one frame of `Response`.

## Request surface

- `Register(Registration)` — admit a repository to the ledger under a class.
- `Retire(Retirement)` — withdraw a repository.
- `SetSpoolDirectory(SpoolDirectoryPolicy)` — set the fallback hook spool
  directory the daemon drains.
- `SetMirror(MirrorPolicy)` — set, and enable or disable, a repository's
  mirror target.

## Reply surface

- `Registered(Registered)`, `Retired(Retired)`,
  `SpoolDirectoryPolicySet(SpoolDirectoryPolicySet)`,
  `MirrorPolicySet(MirrorPolicySet)` — the accepted mutation, echoed by
  identity.
- `RequestUnimplemented(RequestUnimplemented)` — the daemon accepted the
  contract shape but the runtime lacks that operation.

## Shared vocabulary

`Name`, `FilesystemPath` and `Registration` are imported from
`signal-repository-ledger` rather than redeclared, so a repository's identity
and its registration record have one definition across both legs of the
component. `Class` is re-exported for callers that build a `Registration`.

## Naming discipline

`OperationKind` and `UnimplementedReason` are tag enums. Ethos Zero gives a
bare variant the declared type of the same name as its payload, so no declared
type in this contract may spell `Register`, `Retire`, `SetSpoolDirectory`,
`SetMirror`, `StoreUnavailable`, `MirrorDispatchNotBuilt` or
`NotInPrototypeScope`. `tests/generated_contract.rs` carries the behavioural
witness that these stay bare on the wire.

## Owns

- Repository registration and retirement operations.
- The fallback spool directory policy.
- Per-repository mirror policy, including its enabled flag.

## Does Not Own

- The daemon, socket listeners, actors, or memory-engine storage.
- Event ingestion and the ordinary read queries.
- Gitolite server installation, or the mirror dispatch itself.

## Constraints

- Every operation here is privileged and reaches the daemon's meta socket
  only; none is exposed on the ordinary socket.
- Contract records stay domain-specific and runtime-free.
- Policy defaults are the runtime's to choose, not this contract's: Ethos Zero
  derives no `Default`, and a ceiling a runtime needs is asked for explicitly.
