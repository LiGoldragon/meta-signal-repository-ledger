# INTENT — meta-signal-repository-ledger

*The meta-signal authority contract for `repository-ledger`. Defines the typed
request/reply channel the ledger's meta authority uses to register and retire repositories,
set spool-directory policy, and set future mirror policy.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this meta-signal
`meta-signal-repository-ledger` contract. Workspace-shape intent stays in the
primary workspace `primary/INTENT.md`. Component daemon intent stays in
`repository-ledger/INTENT.md`. The ordinary event/query traffic stays in
`signal-repository-ledger/INTENT.md`.

## Why this repo exists

`meta-signal-repository-ledger` is the **meta-signal authority contract** for
`repository-ledger`. It exists because privileged repository policy must be
separated by contract and socket from ordinary repository event/query traffic.
The daemon runs one meta listener actor for this contract and one ordinary
listener actor for `signal-repository-ledger`.

## The channel shape

The meta channel carries (Layer 1 — contract-local verbs on the wire):

- **Requests:** `Register(Registration)` (register a repository),
  `Retire(Retirement)` (retract a repository record),
  `SetSpoolDirectory(SpoolDirectoryPolicy)` (set spool-directory policy),
  `SetMirror(MirrorPolicy)` (future mirror target policy).
- **Replies:** the meta-signal request/reply variants emitted by
  `signal_channel!`.

Layer 2 lowering (`Register` to repository policy mutation, `Retire` to record
retraction) lives in the `repository-ledger` daemon. Layer 3 Sema classification
is daemon-owned observation metadata: component commands project to payloadless
Sema labels via `ToSemaOperation`.

## Constraints

- Meta-signal operations are contract-local verbs in verb form on the wire
  (Layer 1). The daemon owns the typed Component Commands (Layer 2) that project
  to Sema classes (Layer 3 — `Mutate` / `Retract` / `Assert`) for observation
  only; there is no public `Mutate` / `Retract` wrapper on the wire.
- This crate may reuse ordinary-contract nouns when the same typed noun crosses
  the meta boundary, but meta authority lives here.
- Wire enums are closed. No `Unknown` escape hatch.
- This crate depends on `signal-frame` for request/reply frames and carries only
  typed wire vocabulary, NOTA codecs, and round-trip witnesses — no runtime
  actors or sema-engine storage.

## Non-ownership

This crate does not own:

- the ordinary receive-hook notification relation;
- the Gitolite server installation;
- runtime actors or sema-engine storage;
- spool-directory or mirror execution (only the policy vocabulary).

## See also

- `ARCHITECTURE.md` — meta surface, three-layer model, and constraints.
- `../repository-ledger/INTENT.md` — daemon-side intent (ledger, spool, mirror runtime).
- `../signal-repository-ledger/INTENT.md` — ordinary event/query contract.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
