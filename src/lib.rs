//! Owner Signal contract for repository-ledger.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_repository_ledger::{FilesystemPath, Name, Registration};

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct MirrorTarget(String);

impl MirrorTarget {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Retirement {
    pub repository_name: Name,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SpoolDirectoryPolicy {
    pub path: FilesystemPath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MirrorPolicy {
    pub repository_name: Name,
    pub target: MirrorTarget,
    pub enabled: bool,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Registered {
    pub repository_name: Name,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Retired {
    pub repository_name: Name,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SpoolDirectoryPolicySet {
    pub path: FilesystemPath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MirrorPolicySet {
    pub repository_name: Name,
    pub target: MirrorTarget,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum UnimplementedReason {
    StoreUnavailable,
    MirrorDispatchNotBuilt,
    NotInPrototypeScope,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub operation: OperationKind,
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel Owner {
        operation Register(Registration),
        operation Retire(Retirement),
        operation SetSpoolDirectory(SpoolDirectoryPolicy),
        operation SetMirror(MirrorPolicy),
    }
    reply Reply {
        Registered(Registered),
        Retired(Retired),
        SpoolDirectoryPolicySet(SpoolDirectoryPolicySet),
        MirrorPolicySet(MirrorPolicySet),
        RequestUnimplemented(RequestUnimplemented),
    }
}

pub type ChannelRequest = signal_frame::Request<Operation>;
pub type ChannelReply = signal_frame::Reply<Reply>;

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}
