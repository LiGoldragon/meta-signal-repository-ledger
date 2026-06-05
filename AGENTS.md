# meta-signal-repository-ledger — Agent Instructions

Read `~/primary/AGENTS.md`, then this file.

This repository is a pure MetaSignal contract crate. It declares the
meta-signal repository-ledger vocabulary and contains no daemon, storage,
actors, or runtime policy.

Do not put ordinary peer queries here. Peer-callable repository reads and event
submissions belong in `signal-repository-ledger`.
