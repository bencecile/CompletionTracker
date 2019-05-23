use rouille::{Request};
use rouille::input::{json_input};

use serde::de::{DeserializeOwned};
use serde_derive::{Deserialize};

use crate::lang::{Lang};
use crate::source::{
    Universe,
    RelatedLink,

    ImageData,
};
use crate::source::err::{LinkError, SourceError};
use super::{Api};

/// Tries to get the data from the request body
pub fn from_request<T: DeserializeOwned>(req: &Request) -> Result<T, ()> {
    json_input(req).map_err(|_| ())
}

/// This is the error for any data inconsistencies that don't allow us to convert the data.
/// The data error may reference its data.
pub enum DataError {
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
            Some(lang) => Ok(lang),
            None => Err(DataError::BadLang(lang_str)),
        },
        None => Err(DataError::MissingLanguage),
    }
}
fn convert_name(name: Option<String>) -> Result<String, DataError> {
    let name = name.ok_or(DataError::MissingName)?;
    // A name should also have some content to make it useful
    if name.is_empty() {
        Err(DataError::EmptyName)
    } else {
        Ok(name)
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
        let lang = convert_lang(data.lang)?;
        // We always need a name
        let name = convert_name(data.name)?;

        // Validate everything else before we start adding things to the DB
        let image = match data.image {
            // We can't .map the option since we need to be able to return immediately
            Some(image_str) => Some(
                ImageData::read_base64_image(&image_str)
                    .map_err(|e| DataError::SourceError(e))?
            ),
            None => None,
        };
        let related_links = {
            // Create a list of real RelatedLinks
            let mut links = Vec::new();
            for link_data in data.related_links {
                links.push(link_data.create_related_link(lang)?);
            }
            links
        };

        // Get the source database
        let mut source_db = self.lock_db();

        let u_id = source_db.add_universe((lang, name));
        source_db.modify_universe(u_id, |mut universe| {
            // Set the image if we have one
            if let Some(image) = image {
                universe.add_image(image)?;
            }
            // Set the links (we could also just not have any)
            universe.replace_links(related_links);

            Ok(universe)
        }).map_err(|e| DataError::SourceError(e))?;

        // Return the Universe ID so that it can be found again
        Ok(u_id)
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
        let url = self.url.ok_or(DataError::MissingUrl)?;
        
        let mut related_link = RelatedLink::new(&url)
            .map_err(|e| DataError::BadLink(e))?;
        if let Some(description) = self.description {
            // Set the description if we have any
            related_link.add_description((lang, description));
        }

        Ok(related_link)
    }
}
