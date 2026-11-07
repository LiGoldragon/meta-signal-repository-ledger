# Upgrades

## 0.5.0 → 0.6.0

First Datom/Protos 0.33 adoption. Authored Ethos, library, build script and
canonical examples are unchanged. Four generated scalar aliases become tuple
structs with Datom derives. Fixtures construct those wrappers and the
ordinary contract's imported Name/FilesystemPath/RepositoryClass vocabulary;
Registration retains the ordinary contract's own RepositoryName identity.
The disabled-mirror assertion sets the same false value inside MirrorEnabled.
Reader budgets use checked construction. Assertions and values are unchanged.

The selected cohort is Datom eabead4 / Protos 3c7ade3 (0.33.0), Signal
7afb68b (10.3.0), ordinary repository-ledger 9139eda (0.6.0), and Ethos Zero
5a5daca (19.2.0). Registry packages/checksums remain unchanged. The ordinary
dependency is fetched from its declared GitHub URL; its separate local-origin
publication alone was insufficient. The same reviewed source and existing
tag were subsequently published to GitHub under separate guarded authority;
an immutable endpoint fetch matched all tracked source hashes before this
consumer's preparation resumed.

Historical c69d5c4 sources with their own tools and the candidate produce
byte-equal archives/frames for nine existing canonical values: four Query
and five Response samples. Each local frame restores and both directions of
typed binary/text reads succeed. Raw printed text is byte-equal 9/9; the
original textualize() equality assertion is retained without compact
normalization. These are bounded root samples, not exhaustive nested/value,
refusal, source-API, storage or runtime proof.

Checks cover x86_64-linux only; other declared systems are omitted.
No consumer or runtime rollout is performed by this source release.
