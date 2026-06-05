use meta_signal_repository_ledger::{
    ChannelRequest, FilesystemPath, Frame, FrameBody, MirrorPolicy, MirrorPolicySet, MirrorTarget,
    Operation, OperationKind, Registered, Registration, Reply, RequestUnimplemented, Retired,
    Retirement, SpoolDirectoryPolicy, SpoolDirectoryPolicySet, UnimplementedReason,
};
use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, SubReply,
};
use signal_repository_ledger::{Class, Name};

fn name() -> Name {
    Name::new("repository-ledger")
}

fn registration() -> Registration {
    Registration {
        repository_name: name(),
        repository_class: Class::RuntimeComponent,
    }
}

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_operation(operation: Operation) -> Operation {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: operation.into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_reply(reply: Reply) -> Reply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: FrameReply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            FrameReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected ok payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

#[test]
fn meta_operations_round_trip_through_signal_frame() {
    let register = Operation::Register(registration());
    assert_eq!(round_trip_operation(register.clone()), register);

    let retire = Operation::Retire(Retirement {
        repository_name: name(),
    });
    assert_eq!(round_trip_operation(retire.clone()), retire);

    let spool = Operation::SetSpoolDirectory(SpoolDirectoryPolicy {
        path: FilesystemPath::new("/var/lib/repository-ledger/spool"),
    });
    assert_eq!(round_trip_operation(spool.clone()), spool);

    let mirror = Operation::SetMirror(MirrorPolicy {
        repository_name: name(),
        target: MirrorTarget::new("git@github.com:LiGoldragon/repository-ledger"),
        enabled: true,
    });
    assert_eq!(round_trip_operation(mirror.clone()), mirror);
}

#[test]
fn meta_replies_round_trip_through_signal_frame() {
    let registered = Reply::Registered(Registered {
        repository_name: name(),
    });
    assert_eq!(round_trip_reply(registered.clone()), registered);

    let retired = Reply::Retired(Retired {
        repository_name: name(),
    });
    assert_eq!(round_trip_reply(retired.clone()), retired);

    let spool_set = Reply::SpoolDirectoryPolicySet(SpoolDirectoryPolicySet {
        path: FilesystemPath::new("/var/lib/repository-ledger/spool"),
    });
    assert_eq!(round_trip_reply(spool_set.clone()), spool_set);

    let mirror_set = Reply::MirrorPolicySet(MirrorPolicySet {
        repository_name: name(),
        target: MirrorTarget::new("git@github.com:LiGoldragon/repository-ledger"),
    });
    assert_eq!(round_trip_reply(mirror_set.clone()), mirror_set);

    let unimplemented = Reply::RequestUnimplemented(RequestUnimplemented {
        operation: OperationKind::SetMirror,
        reason: UnimplementedReason::MirrorDispatchNotBuilt,
    });
    assert_eq!(round_trip_reply(unimplemented.clone()), unimplemented);
}

#[test]
fn meta_operations_encode_as_contract_local_nota_heads() {
    let operation = Operation::Register(registration());
    let mut encoder = Encoder::new();
    operation
        .into_request()
        .encode(&mut encoder)
        .expect("encode");
    let text = encoder.into_string();

    assert!(text.starts_with("(Register "));
    assert!(!text.contains("Mutate"));
    assert!(!text.contains("Retract"));

    let mut decoder = Decoder::new(&text);
    let decoded = ChannelRequest::decode(&mut decoder).expect("decode");
    assert_eq!(decoded.payloads().head().kind(), OperationKind::Register);
}
