use std::path::{PathBuf};

use super::{SourceItemIDPair};

/// Various errors can occur with source-related operations.
/// There may be more than 1 error type for a given "error", but this will help with translations.
pub enum SourceError {
    /// The source ID doesn't match its location
    IDNotMatching(SourceItemIDPair),

    /// An image string has a bad Base64 representation
    ImageBadBase64,
    /// The image data can't be read as an image
    ImageBadData,
    /// An image is missing where it should exist
    ImageMissing(SourceItemIDPair),
    /// An image failed to write to disk
    ImageWriteFailed(PathBuf),

    /// A series can't be added if a universe isn't added on a Source
    MissingUniverse,
    /// An arc can't be added if the series is missing on a Source
    MissingSeries,

    /// The type that can't be found
    NotFound(SourceItemIDPair),
    /// A relation of any kind can't find the ID of the same type
    NotFoundRelation(SourceItemIDPair, u64),

    /// A save operation failed
    FailedSave,
}

/// The error type when trying to create a link
pub enum LinkError {
    /// The structure of the URL is somehow incorrect
    MalformedUrl(String),
    /// The LinkType for the Url cannot be found
    UnknownLink(String),
}
