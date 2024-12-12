use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Clone, Debug, Default)]
#[repr(u8)]
pub enum FileReleaseType {
    #[default]
    Release = 1,
    Beta = 2,
    Alpha = 3,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Clone, Debug, Default)]
#[repr(u8)]
pub enum FileStatus {
    #[default]
    PROCESSING = 1,
    ChangesRequired = 2,
    UnderReview = 3,
    Approved = 4,
    Rejected = 5,
    MalwareDetected = 6,
    Deleted = 7,
    Archived = 8,
    Testing = 9,
    Released = 10,
    ReadyForReview = 11,
    Deprecated = 12,
    Baking = 13,
    AwaitingPublishing = 14,
    FailedPublishing = 15,
}
