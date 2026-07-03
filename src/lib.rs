//! Meta Signal contract for repository-ledger.

#[cfg(not(feature = "nota-text"))]
use nota::{Block, NotaDecodeError};
use nota::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_repository_ledger::{FilesystemPath, Name, Registration};

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Retirement {
    pub repository_name: Name,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SpoolDirectoryPolicy {
    pub path: FilesystemPath,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MirrorPolicy {
    pub repository_name: Name,
    pub target: MirrorTarget,
    pub enabled: bool,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Registered {
    pub repository_name: Name,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Retired {
    pub repository_name: Name,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SpoolDirectoryPolicySet {
    pub path: FilesystemPath,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MirrorPolicySet {
    pub repository_name: Name,
    pub target: MirrorTarget,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum UnimplementedReason {
    StoreUnavailable,
    MirrorDispatchNotBuilt,
    NotInPrototypeScope,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestUnimplemented {
    pub operation: OperationKind,
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel Meta {
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

#[cfg(not(feature = "nota-text"))]
impl OperationKind {
    const fn as_nota_atom(self) -> &'static str {
        match self {
            Self::Register => "Register",
            Self::Retire => "Retire",
            Self::SetSpoolDirectory => "SetSpoolDirectory",
            Self::SetMirror => "SetMirror",
        }
    }

    fn from_nota_atom(atom: &str) -> Result<Self, NotaDecodeError> {
        match atom {
            "Register" => Ok(Self::Register),
            "Retire" => Ok(Self::Retire),
            "SetSpoolDirectory" => Ok(Self::SetSpoolDirectory),
            "SetMirror" => Ok(Self::SetMirror),
            variant => Err(NotaDecodeError::UnknownVariant {
                enum_name: "OperationKind",
                variant: variant.to_owned(),
            }),
        }
    }
}

#[cfg(not(feature = "nota-text"))]
impl NotaEncode for OperationKind {
    fn to_nota(&self) -> String {
        self.as_nota_atom().to_owned()
    }
}

#[cfg(not(feature = "nota-text"))]
impl NotaDecode for OperationKind {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let atom = block
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom {
                type_name: "OperationKind",
            })?;
        Self::from_nota_atom(atom)
    }
}

pub type ChannelRequest = signal_frame::Request<Operation>;
pub type ChannelReply = signal_frame::Reply<Reply>;

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}
