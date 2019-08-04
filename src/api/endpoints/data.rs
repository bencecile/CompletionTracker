//! This module specializes in taking the JSON data sent from the client, and turning
//! it into the data structures used in the actual implementation.
//!
//! Each data object has a lot of options so that we can determine exactly what goes wrong
//! when deserializing a structure. The user will then see more specific errors.

use std::collections::{BTreeMap};
use std::ops::{Deref, DerefMut};

use rouille::{Request};
use rouille::input::{json_input};

use serde::de::{DeserializeOwned};
use serde_derive::{Deserialize};

use crate::source::{
    RelatedLink,
    SourceLang, SourceLangStrings,
};
use crate::source_concrete::source_image::{ImageData};
use crate::source::err::{LinkError, SourceError};

/// Tries to get the data from the request body
pub fn from_request<T: DeserializeOwned>(req: &Request) -> DataResult<T> {
    json_input(req).map_err(|e| DataError::BadJson(e.to_string()))
}

/// This is the error for any data inconsistencies that don't allow us to convert the data.
/// The data error may reference its data.
pub enum DataError {
    BadJson(String),
    /// A bad language short form
    BadLang(String),
    /// A bad link with the error
    BadLink(LinkError),
    
    /// A name can't be empty
    EmptyName,

    /// A required ID is missing
    MissingId,
    /// A required language selection is missing
    MissingLanguage,
    /// A name is missing but a name is always required
    MissingName,
    /// A required URL is missing
    MissingUrl,

    /// An error occured during a source operation
    SourceError(SourceError),
}
impl From<SourceError> for DataError {
    fn from(source_err: SourceError) -> DataError { DataError::SourceError(source_err) }
}

pub type DataResult<T> = Result<T, DataError>;

// ---- Define some generic data functions that we will be doing repeatedly ----
fn convert_names(name: Option<SourceLangStrings>) -> Result<SourceLangStrings, DataError> {
    let name = name.ok_or(DataError::MissingName)?;
    // A name should also have some content to make it useful
    if name.is_empty() {
        Err(DataError::EmptyName)
    } else {
        Ok(name)
    }
}
fn convert_image(image: Option<String>) -> Option< Result<ImageData, DataError> > {
    image.map(|image_str| ImageData::read_base64_image(&image_str)
        .map_err(|e| DataError::SourceError(e)))
}

/// This structure holds the data required for a record
#[derive(Deserialize)]
pub struct RecordData<T> {
    id: Option<u64>,
    names: Option<SourceLangStrings>,
    pub aliases: BTreeMap<SourceLang, Vec<String>>,
    pub descriptions: Option<SourceLangStrings>,
    /// The Base64'd image
    image: Option<String>,
    related_links: Vec<RelatedLinkData>,
    /// Any sub data that may be needed for a specific type
    sub_data: T,
}
impl <T> RecordData<T> {
    pub fn convert_names(&mut self) -> DataResult<SourceLangStrings> {
        convert_names(self.names.take())
    }
    pub fn convert_image(&mut self) -> Option< DataResult<ImageData> > {
        convert_image(self.image.take())
    }
    pub fn create_related_links(&mut self) -> DataResult< Vec<RelatedLink> > {
        // Create a list of real RelatedLinks
        let mut links = Vec::with_capacity(self.related_links.len());
        for link_data in self.related_links.drain(..) {
            links.push(link_data.create_related_link()?);
        }
        Ok(links)
    }
}
impl <T> Deref for RecordData<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.sub_data }
}
impl <T> DerefMut for RecordData<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.sub_data }
}

/// The data to create a RelatedLink
#[derive(Deserialize)]
pub struct RelatedLinkData {
    /// The url for the link
    url: Option<String>,
    /// The description of the url
    description: Option<SourceLangStrings>,
}
impl RelatedLinkData {
    /// Tries to create a related link from this data and a language
    pub fn create_related_link(self) -> DataResult<RelatedLink> {
        let url = self.url.ok_or(DataError::MissingUrl)?;
        
        let mut related_link = RelatedLink::new(&url)
            .map_err(|e| DataError::BadLink(e))?;
        if let Some(description) = self.description {
            // Set the description if we have any
            related_link.description = description;
        }

        Ok(related_link)
    }
}

/// If the record data doesn't need anything else specific, use this.
/// It's an option so that it doesn't need to exist in the JSON.
pub type EmptySubData = Option<()>;
