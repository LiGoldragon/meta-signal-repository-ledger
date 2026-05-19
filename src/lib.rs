//! OwnerSignal contract for repository-ledger.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;
pub use signal_repository_ledger::{RepositoryLedgerPath, RepositoryName, RepositoryRegistration};

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
pub struct RetireRepository {
    pub repository_name: RepositoryName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SpoolDirectoryPolicy {
    pub path: RepositoryLedgerPath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MirrorPolicy {
    pub repository_name: RepositoryName,
    pub target: MirrorTarget,
    pub enabled: bool,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRegistered {
    pub repository_name: RepositoryName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRetired {
    pub repository_name: RepositoryName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SpoolDirectoryPolicySet {
    pub path: RepositoryLedgerPath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MirrorPolicySet {
    pub repository_name: RepositoryName,
    pub target: MirrorTarget,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerRepositoryLedgerOperationKind {
    RegisterRepository,
    RetireRepository,
    SetSpoolDirectoryPolicy,
    SetMirrorPolicy,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerRepositoryLedgerUnimplementedReason {
    StoreUnavailable,
    MirrorDispatchNotBuilt,
    NotInPrototypeScope,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OwnerRepositoryLedgerRequestUnimplemented {
    pub operation: OwnerRepositoryLedgerOperationKind,
    pub reason: OwnerRepositoryLedgerUnimplementedReason,
}

signal_channel! {
    channel OwnerRepositoryLedger {
        request OwnerRepositoryLedgerRequest {
            Mutate RegisterRepository(RepositoryRegistration),
            Retract RetireRepository(RetireRepository),
            Mutate SetSpoolDirectoryPolicy(SpoolDirectoryPolicy),
            Mutate SetMirrorPolicy(MirrorPolicy),
        }
        reply OwnerRepositoryLedgerReply {
            RepositoryRegistered(RepositoryRegistered),
            RepositoryRetired(RepositoryRetired),
            SpoolDirectoryPolicySet(SpoolDirectoryPolicySet),
            MirrorPolicySet(MirrorPolicySet),
            OwnerRepositoryLedgerRequestUnimplemented(OwnerRepositoryLedgerRequestUnimplemented),
        }
    }
}

pub type Frame = OwnerRepositoryLedgerFrame;
pub type FrameBody = OwnerRepositoryLedgerFrameBody;
pub type ChannelRequest = OwnerRepositoryLedgerChannelRequest;
pub type ChannelReply = OwnerRepositoryLedgerChannelReply;
pub type RequestBuilder = OwnerRepositoryLedgerRequestBuilder;

impl OwnerRepositoryLedgerRequest {
    pub fn operation_kind(&self) -> OwnerRepositoryLedgerOperationKind {
        match self {
            Self::RegisterRepository(_) => OwnerRepositoryLedgerOperationKind::RegisterRepository,
            Self::RetireRepository(_) => OwnerRepositoryLedgerOperationKind::RetireRepository,
            Self::SetSpoolDirectoryPolicy(_) => {
                OwnerRepositoryLedgerOperationKind::SetSpoolDirectoryPolicy
            }
            Self::SetMirrorPolicy(_) => OwnerRepositoryLedgerOperationKind::SetMirrorPolicy,
        }
    }
}
