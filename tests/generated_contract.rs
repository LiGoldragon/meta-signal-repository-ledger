use meta_signal_repository_ledger::{
    Class, MirrorPolicy, MirrorPolicySet, OperationKind, Query, Registered, Registration,
    RequestUnimplemented, Response, Retired, Retirement, SpoolDirectoryPolicy,
    SpoolDirectoryPolicySet, UnimplementedReason,
};
use signal::{ByteViewable, Restorable, Signal, Signalizable};

fn registration() -> Registration {
    Registration {
        repository_name: String::from("repository-ledger"),
        repository_class: Class::RuntimeComponent,
    }
}

#[test]
fn a_registration_restores_from_fresh_peer_bytes() {
    let query = Query::Register(registration());
    let outgoing = query.signalize().expect("archive query");
    assert!(!outgoing.bytes().is_empty());
    let incoming = Signal::<Query>::from(outgoing.bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore query"), query);
}

#[test]
fn every_configuration_operation_restores_from_fresh_peer_bytes() {
    for query in [
        Query::Register(registration()),
        Query::Retire(Retirement {
            repository_name: String::from("repository-ledger"),
        }),
        Query::SetSpoolDirectory(SpoolDirectoryPolicy {
            spool_directory: String::from("/var/spool/repository-ledger"),
        }),
        Query::SetMirror(MirrorPolicy {
            repository_name: String::from("repository-ledger"),
            mirror_target: String::from("git@github.com:LiGoldragon/repository-ledger.git"),
            mirror_enabled: true,
        }),
    ] {
        let incoming = Signal::<Query>::from(query.signalize().expect("archive").bytes().to_vec());
        assert_eq!(incoming.restore().expect("restore"), query);
    }
}

#[test]
fn every_acknowledgement_restores_from_fresh_peer_bytes() {
    for response in [
        Response::Registered(Registered {
            repository_name: String::from("repository-ledger"),
        }),
        Response::Retired(Retired {
            repository_name: String::from("repository-ledger"),
        }),
        Response::SpoolDirectoryPolicySet(SpoolDirectoryPolicySet {
            spool_directory: String::from("/var/spool/repository-ledger"),
        }),
        Response::MirrorPolicySet(MirrorPolicySet {
            repository_name: String::from("repository-ledger"),
            mirror_target: String::from("git@github.com:LiGoldragon/repository-ledger.git"),
        }),
    ] {
        let incoming =
            Signal::<Response>::from(response.signalize().expect("archive").bytes().to_vec());
        assert_eq!(incoming.restore().expect("restore"), response);
    }
}

#[test]
fn malformed_peer_bytes_are_rejected() {
    assert!(Signal::<Query>::from(vec![0xff, 0, 1]).restore().is_err());
}

/// `OperationKind` and `UnimplementedReason` are tag enums. Ethos Zero gives a
/// bare variant the declared type of the same name as its payload, so a
/// contract that spelled one of their heads as a declared type would silently
/// make these carry data. Building each by its bare head and carrying it over
/// the wire is the witness that they stay bare.
#[test]
fn the_kind_tags_cross_the_wire_as_bare_heads() {
    for operation_kind in [
        OperationKind::Register,
        OperationKind::Retire,
        OperationKind::SetSpoolDirectory,
        OperationKind::SetMirror,
    ] {
        let response = Response::RequestUnimplemented(RequestUnimplemented {
            operation_kind,
            unimplemented_reason: UnimplementedReason::MirrorDispatchNotBuilt,
        });
        let incoming =
            Signal::<Response>::from(response.signalize().expect("archive").bytes().to_vec());
        assert_eq!(incoming.restore().expect("restore"), response);
    }
}

/// A disabled mirror policy is a different wire value from an enabled one.
/// The flag is the only field distinguishing them, so it is worth its own
/// witness.
#[test]
fn a_disabled_mirror_policy_is_distinct_from_an_enabled_one() {
    let enabled = MirrorPolicy {
        repository_name: String::from("repository-ledger"),
        mirror_target: String::from("git@github.com:LiGoldragon/repository-ledger.git"),
        mirror_enabled: true,
    };
    let mut disabled = enabled.clone();
    disabled.mirror_enabled = false;
    assert_ne!(enabled, disabled);
    let query = Query::SetMirror(disabled.clone());
    let incoming = Signal::<Query>::from(query.signalize().expect("archive").bytes().to_vec());
    assert_eq!(incoming.restore().expect("restore"), query);
}

#[cfg(feature = "datom")]
mod datom_text {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use meta_signal_repository_ledger::{Query, Response};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    fn budget() -> Budget {
        Budget {
            remaining: 65536,
            reader: ReaderBudget { remaining: 65536 },
            depth: 0,
            maximum_depth: 256,
        }
    }

    /// Every canonical line must actualize as a request or as a reply, and the
    /// text it actualizes from must be the text the codec writes for the value
    /// it yields. A line that no longer parses, or that round-trips to
    /// different text, is a wire change this file did not record.
    #[test]
    fn every_canonical_line_actualizes_and_re_renders_to_itself() {
        let canonical = include_str!("../examples/canonical.datom");
        let lines: Vec<&str> = canonical
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with(';'))
            .collect();
        assert_eq!(lines.len(), 9, "canonical file lost or gained a line");

        let mut as_query = 0;
        for line in lines {
            let rendered = match Potential::<Query>::from(line.to_owned()).actualize(&mut budget()) {
                Ok(query) => {
                    as_query += 1;
                    query.datomize(vec![]).protosize().textualize()
                }
                Err(_) => Potential::<Response>::from(line.to_owned())
                    .actualize(&mut budget())
                    .unwrap_or_else(|error| {
                        panic!("canonical line actualizes as neither request nor reply: {line}\n{error:?}")
                    })
                    .datomize(vec![])
                    .protosize()
                    .textualize(),
            };
            assert_eq!(
                rendered, line,
                "canonical line does not re-render to itself"
            );
        }
        assert_eq!(as_query, 4, "the request half of the canonical file moved");
    }
}
