use rouille::{Request};
use rouille::input::{json_input};

use serde::de::{DeserializeOwned};
use serde_derive::{Deserialize};

use crate::lang::{Lang};
use crate::source::{
    Universe,
    RelatedLink,

    ImageData,
    SourceError,
};
use super::{Api};

/// Tries to get the data from the request body
pub fn from_request<T: DeserializeOwned>(req: &Request) -> Result<T, ()> {
    json_input(req).map_err(|_| ())
}

/// This is the error for any data inconsistencies that don't allow us to convert the data.
/// The data error may reference its data.
pub enum DataError {
    /// An image string has a bad Base64 representation
    BadImageBase64,
    /// The image data can't be read as an image
    BadImageData,
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

// ---- Define some generic data functions that we will be doing repeatedly ----
fn convert_lang(lang_str: Option<String>) -> Result<Lang, DataError> {
    match lang_str {
        Some(lang_str) => match Lang::from_short_str(&lang_str) {
            Some(lang) => lang,
            None => return Err(DataError::BadLang(lang_str)),
        },
        None => return Err(DataError::MissingLang),
    }
}

/// This structure holds the data required to create a new universe
#[derive(Deserialize)]
pub struct UniverseData {
    /// Have an optional ID if this is an edit request
    id: Option<u64>,
    /// The language string to determine the language of this request
    lang: Option<String>,
    /// The name of the universe
    name: Option<String>,
    /// A description of the universe
    description: Option<String>,
    /// The Base64'd image
    image: Option<String>,
    /// A list of related links
    related_links: Vec<RelatedLinkData>,
}
/// Implement the methods that deal with UniverseData
impl Api {
    /// Tries to insert a new universe into the DB.
    /// Returns the ID of the newly created Universe.
    pub fn create_new_universe(&self, data: UniverseData) -> Result<u64, DataError> {
        // Ignore the ID since we don't need one for creation
        // Fail on a missing language
        let lang = convert_lang(self.lang)?;
        // We always need a name
        let name = data.name.ok_or(DataError::MissingName)?;

        // Validate everything else before we start adding things to the DB
        let image = match data.image {
            // We can't .map the option since we need to be able to return immediately
            Some(image) => ImageData::read_base64_image(&image)?,
            None => None,
        };
        let related_links = match data.related_links {
            Some(link_data_list) => {
                // Create a list of real RelatedLinks
                let mut links = Vec::new();
                for link_data in link_data_list {
                    links.push(link_data.create_related_link()?);
                }
                links
            },
            None => None,
        };

        // Get the source database
        let source_db = self.lock_db();

        let u_id = source_db.add_universe((lang, name));
        source_db.modify_universe(|universe| {
            // Set the image if we have one
            if let Some(image) = image {
                universe.add_image(image)?;
            }
            // Set the links if we have any
            if let Some(related_links) = related_links {
                universe.replace_links(related_links);
            }
            Ok(())
        }).map_err(|e| DataError::SourceError(e))
    }
}

/// The data to create a RelatedLink
#[derive(Deserialize)]
pub struct RelatedLinkData {
    /// The url for the link
    url: Option<String>,
    /// The description of the url
    description: Option<String>,
}
impl RelatedLinkData {
    /// Tries to create a related link from this data and a language
    fn create_related_link(self, lang: Lang) -> Result<RelatedLink, DataError> {
        let url = match self.url {
            Some(url) => url,
            None => return Err(DataError::MissingUrl),
        };
        
        let mut related_link = RelatedLink::new(&url)
            .map_err(|e| Err(DataError::BadLink(e)))?;
        if let Some(description) = self.description {
            // Set the description if we have any
            related_link.add_description((lang, description));
        }

        Ok(related_link)
    }
}
