use reqwest::{Url};

use serde_derive::{Deserialize, Serialize};

use super::{SourceLangStrings};
use super::err::{LinkError};
use crate::types::{Date, DateError};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum Country {
    Japan,
}

/// These specify the relationship between things (using IDs)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum Relation {
    /// Some other thing that happens before
    Before(u64),
    /// Some other thing that happens after
    After(u64),
    /// An alternate version of the same-ish content
    Alternate(u64),
}
impl Relation {
    /// Gives a pair of relations with the first source happening before the second
    pub fn relate_source(before_id: u64, after_id: u64) -> (Relation, Relation) {
        let before = Relation::Before(before_id);
        let after = Relation::After(after_id);
        (after, before)
    }

    /// Returns the ID in this relation
    pub fn related_id(&self) -> u64 {
        match self {
            Relation::Before(id) |
            Relation::After(id) |
            Relation::Alternate(id) => *id,
        }
    }

    /// Modifies the related ID
    pub fn change_related_id(&mut self, new_id: u64) {
        match self {
            Relation::Before(ref mut id) |
            Relation::After(ref mut id) |
            Relation::Alternate(ref mut id) => *id = new_id,
        }
    }
}

/// A release date with a description
#[derive(Clone, Deserialize, Serialize)]
pub struct ReleaseDate {
    date: Date,
    pub descriptions: SourceLangStrings,
}
impl ReleaseDate {
    /// Creates a new release date
    pub fn new(date: &str) -> Result<ReleaseDate, DateError> {
        let date = Date::parse_date(date)?;
        Ok(ReleaseDate {
            date,
            descriptions: SourceLangStrings::new_empty(),
        })
    }
}

/// A URL that will have related information
#[derive(Clone, Deserialize, Serialize)]
pub struct RelatedLink {
    /// The URL of the link
    url: String,
    /// The type of the link (ie. which website are we linking to)
    link_type: LinkType,
    /// A description of the link
    pub description: SourceLangStrings,
}
impl RelatedLink {
    /// Tries to create a related link from the url and description.
    /// Fails if the url isn't well-formed.
    pub fn new(url: &str) -> Result<RelatedLink, LinkError> {
        // See if the URL checks out syntactically
        if let Err(_) = Url::parse(url) {
            return Err(LinkError::MalformedUrl(url.to_string()));
        }
        // Tries to find the LinkType from the URL
        let link_type = if let Some(link_type) = LinkType::from_url(url) {
            link_type
        } else {
            return Err(LinkError::UnknownLink(url.to_string()));
        };

        Ok(RelatedLink {
            url: url.to_string(),
            link_type,
            description: SourceLangStrings::new_empty(),
        })
    }
    pub fn url(&self) -> &str { &self.url }
}


/// The type of resource that is being linked.
/// This can be useful for providing link text and icons (like the 'W' for Wikipedia).
#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum LinkType {
    /// This is for https://syosetu.com/ and the novels that it contains
    ShousetsukaNarou,
    /// Any part of wikipedia.org
    Wikipedia,
}
impl LinkType {
    /// Pairs with a URL and the matching link type
    fn url_pairs() -> &'static [(&'static str, LinkType)] {
        &[
            ("syosetu.com", LinkType::ShousetsukaNarou),
            ("wikipedia.org", LinkType::Wikipedia),
        ]
    }
    /// Tries to parse the link type from the URL
    pub fn from_url(url: &str) -> Option<LinkType> {
        // Go through each variant to see if this URL belongs to a known site
        for (url_part, link_type) in LinkType::url_pairs().iter() {
            if url.contains(url_part) { return Some(*link_type); }
        }
        None
    }
}


/// The type (codec) of an image
#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum ImageType {
    Jpeg,
    Png,
}
impl ImageType {
    /// Gets the extension for the image type (excluding the prefix period '.')
    pub fn extension(&self) -> &'static str {
        match self {
            ImageType::Jpeg => "jpg",
            ImageType::Png => "png",
        }
    }
}
