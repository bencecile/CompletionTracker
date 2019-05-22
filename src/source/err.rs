use super::{SourceItem};

/// We need the type of item with the ID of that item
pub type SourceErrorType = (SourceItem, u64);

/// Various errors can occur with source-related operations.
/// There may be more than 1 error type for a given "error", but this will help with translations.
pub enum SourceError {
    /// An image failed to write to disk
    FailedImageWrite(SourceErrorType),

    /// The source ID doesn't match its location
    IDNotMatching(SourceErrorType),

    /// A series can't be added if a universe isn't added on a Source
    MissingUniverse,
    /// An arc can't be added if the series is missing on a Source
    MissingSeries,

    /// The type that can't be found
    NotFound(SourceErrorType),
    /// The related source ID can't be found on the type
    NotFoundSourceRelation(SourceErrorType, u64),
}

/// The error type when trying to create a link
pub enum LinkError {
    /// The structure of the URL is somehow incorrect
    MalformedUrl(String),
    /// The LinkType for the Url cannot be found
    UnknownLink(String),
}
