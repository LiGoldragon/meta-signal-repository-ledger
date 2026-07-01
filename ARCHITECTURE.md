# meta-signal-repository-ledger Architecture

`meta-signal-repository-ledger` is the meta-signal authority contract for
`repository-ledger`.

It exists because privileged repository policy must be separated by contract
and socket from ordinary repository event/query traffic. The daemon has one
meta listener actor for this contract and one ordinary listener actor for
`signal-repository-ledger`.

## 0.5 · Direction

`meta-signal-repository-ledger` is the meta authority contract for `repository-ledger`. It exists because privileged repository policy must be separated by contract and socket from ordinary repository event/query traffic: the daemon runs one meta listener actor for this contract and one ordinary listener actor for `signal-repository-ledger`. Meta-signal operations are contract-local verbs in verb form on the wire (Layer 1); the daemon owns the typed Component Commands (Layer 2) that project to Sema class labels (Layer 3) for observation only — no public `Mutate`/`Retract` wrapper appears on the wire.

## Migration History — Three-Layer Model (2026-05-22)

This contract uses the three-layer model affirmed 2026-05-20 per
`primary/skills/component-triad.md`.

Layer 1 meta-signal contract operations are contract-local verbs on the
wire: `Register(Registration)`, `Retire(Retirement)`,
`SetSpoolDirectory(SpoolDirectoryPolicy)`, and
`SetMirror(MirrorPolicy)`. The contract depends on `signal-frame` for
request/reply frames and does not expose public `Mutate` / `Retract`
wrappers.

Layer 2 lowering from contract operations to typed component commands
(`Register` to repository policy mutation, `Retire` to repository
record retraction, etc.) lives in the
`repository-ledger` daemon.

Layer 3 Sema classification is daemon-owned observation metadata:
component commands project to payloadless Sema labels via
`ToSemaOperation`.

## Owns

- Repository registration and retirement requests.
- Spool directory policy.
- Future mirror target policy records.
- Meta-signal request/reply variants declared with `signal_channel!`.

## Does Not Own

- The ordinary receive-hook notification relation.
- The Gitolite server installation.
- Runtime actors or sema-engine storage.

## Constraints

- Meta-signal operations are contract-local verbs in
  verb form on the wire (Layer 1). The daemon owns the typed
  Component Commands (Layer 2) that project to Sema classes
  (Layer 3 — `Mutate` / `Retract` / `Assert`) for observation only.
- This crate may reuse ordinary contract nouns when the same typed
  noun crosses the meta boundary, but meta authority lives here.
