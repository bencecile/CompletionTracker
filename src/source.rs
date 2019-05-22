pub mod character;
pub mod company;
pub mod err;
use self::err::{SourceError};
pub mod person;
mod source_db;
pub use self::source_db::{SourceDB};
pub mod ui;

use std::fs;
use std::path::{Path, PathBuf};

use base64;

use image::{self, ImageFormat, ImageOutputFormat};

use reqwest::{Url};

use serde_derive::{Deserialize, Serialize};

use crate::lang::{Lang, LangString, LangStrings};
use crate::source::err::{LinkError};
use crate::types::{Date, DateError};

/// This is the folder for all of the source information
const SOURCE_FOLDER: &'static str = "sources";
pub fn create_source_folder() -> Result<(), String> {
    let folder = Path::new(SOURCE_FOLDER);
    // Only try to create the folder if it isn't here
    if !folder.is_dir() {
        fs::create_dir(folder)
            .map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}
/// Creates a path to the source file inside the source folder
pub fn source_file_path(file_name: &str) -> PathBuf {
    Path::new(SOURCE_FOLDER).join(file_name)
}


/// An item that is related to sources
#[derive(Copy, Clone)]
pub enum SourceItem {
    Source,

    Universe,
    /// A series belongs to a universe (u_id)
    Series(u64),
    /// An arc belongs to a series and universe by extension (u_id, s_id)
    Arc(u64, u64),

    Character,
    Person,
    Company,
}
impl SourceItem {
    /// Gets the path to the related list file for this item
    pub fn list_file(&self) -> PathBuf {
        match self {
            SourceItem::Source => source_file_path("sources.json"),
            SourceItem::Universe |
            SourceItem::Series(_) |
            SourceItem::Arc(_, _) => source_file_path("universes.json"),
            SourceItem::Character => source_file_path("characters.json"),
            SourceItem::Person => source_file_path("people.json"),
            SourceItem::Company => source_file_path("companies.json"),
        }
    }

    /// Gets the image directory for the item
    pub fn image_dir(&self) -> PathBuf {
        match self {
            SourceItem::Source => source_file_path("imgSources"),
            SourceItem::Universe => source_file_path("imgUniverses"),
            // The universes will have a directory that is the id of the universe
            SourceItem::Series(u_id) => SourceItem::Universe.image_dir().join(format!("{}", u_id)),
            // Each series will have a directory with its ID
            SourceItem::Arc(u_id, s_id) => SourceItem::Series(u_id).image_dir()
                .join(format!("{}", s_id)),
            SourceItem::Character => source_file_path("imgCharacters"),
            SourceItem::Person => source_file_path("imgPeople"),
            SourceItem::Company => source_file_path("imgCompanies"),
        }
    }

    /// Finds the path to an image using the item's ID and image type
    pub fn image_path(&self, id: u64, image_type: ImageType) -> PathBuf {
        // The image will be directly underneath the directory
        self.image_dir().join(format!("{}.{}", id, image_type.extension()))
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Universe {
    names: LangStrings,
    id: u64,
    descriptions: LangStrings,
    /// The type of image for this universe
    image: Option<ImageType>,
    series: Vec<Series>,
    related_links: Vec<RelatedLink>,
}
impl Universe {
    /// Creates a new empty Universe with a single name
    pub fn new(name: LangString, id: u64) -> Universe {
        Universe {
            names: LangStrings::new(name),
            id,
            descriptions: LangStrings::new_empty(),
            image: None,
            series: Vec::new(),
            related_links: Vec::new(),
        }
    }
    pub fn name(&self, lang: Lang) -> &str { self.names.get_str(lang) }
    pub fn id_eq(&self, id: u64) -> bool { self.id == id }

    /// Adds an image using the image data
    pub fn add_image(&mut self, image_data: ImageData) -> Result<(), SourceError> {
        // Try to save the image before we add the image
        let image_type = image_data.save(SourceItem::Universe, self.id)?;
        let self.image = Some(image_type);
        Ok(())
    }

    /// Replaces all of the links
    pub fn replace_links(&mut self, links: Vec<RelatedLink>) {
        self.related_links = links;
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Series {
    names: LangStrings,
    id: u64,
    descriptions: LangStrings,
    /// The type of image for this series
    image: Option<ImageType>,
    arcs: Vec<Arc>,
    related_series: Vec<Relation>,
    related_links: Vec<RelatedLink>,
}
impl Series {
    /// Creates a new empty series with the name
    pub fn new(name: LangString, id: u64) -> Series {
        Series {
            names: LangStrings::new(name),
            id,
            descriptions: LangStrings::new_empty(),
            image: None,
            arcs: Vec::new(),
            related_series: Vec::new(),
            related_links: Vec::new(),
        }
    }
    pub fn name(&self, lang: Lang) -> &str { self.names.get_str(lang) }
    pub fn id_eq(&self, id: u64) -> bool { self.id == id }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Arc {
    names: LangStrings,
    id: u64,
    descriptions: LangStrings,
    /// The type of image for this arc
    image: Option<ImageType>,
    related_arcs: Vec<Relation>,
    related_links: Vec<RelatedLink>,
}
impl Arc {
    pub fn new(name: LangString, id: u64) -> Arc {
        Arc {
            names: LangStrings::new(name),
            id,
            descriptions: LangStrings::new_empty(),
            image: None,
            related_arcs: Vec::new(),
            related_links: Vec::new(),
        }
    }
    pub fn name(&self, lang: Lang) -> &str { self.names.get_str(lang) }
    pub fn id_eq(&self, id: u64) -> bool { self.id == id }
}

/// This is the definition of a source
#[derive(Clone, Deserialize, Serialize)]
pub struct Source {
    /// The name of the source
    names: LangStrings,
    /// An identifier for this source
    id: u64,
    /// The type of media that holds the source (the name)
    media_type: MediaType,
    /// The universe in which the source takes place (the name)
    universe: Option<u64>,
    /// The series in which the source takes place (the name)
    series: Option<u64>,
    /// The arc in which the source takes place (the name)
    arc: Option<u64>,
    /// A description of the source
    descriptions: LangStrings,
    /// The type of image for this source
    image: Option<ImageType>,
    /// Release dates for this source in YYYY-MM-DD. Also includes notes about the release
    release_dates: Vec<ReleaseDate>,
    /// Other sources that are related to this one
    related_sources: Vec<Relation>,
    /// Related links to websites on this source
    related_links: Vec<RelatedLink>,
    /// These are people who have contributed to this source
    people: Vec<u64>,
    /// These are fictional characters who appear in this source
    characters: Vec<u64>,
    /// These are any companies that are involved in the creation of this source
    companies: Vec<u64>,
}
impl Source {
    /// Creates a new empty source
    pub fn new(name: LangString, media_type: MediaType, id: u64) -> Source {
        Source {
            names: LangStrings::new(name),
            id,
            media_type,
            universe: None,
            series: None,
            arc: None,
            descriptions: LangStrings::new_empty(),
            image: None,
            release_dates: Vec::new(),
            related_sources: Vec::new(),
            related_links: Vec::new(),
            people: Vec::new(),
            characters: Vec::new(),
            companies: Vec::new(),
        }
    }
    pub fn name(&self, lang: Lang) -> &str { self.names.get_str(lang) }
    pub fn id_eq(&self, id: u64) -> bool { self.id == id }

    /// Links this source before the other
    pub fn relate_before(&mut self, other: &mut Source) {
        let (after, before) = Relation::relate_source(&self, &other);
        self.related_sources.push(after);
        other.related_sources.push(before);
    }
    /// Changes the ID of a related source
    pub fn modify_related_source_id(&mut self, index: usize, id: u64) {
        self.related_sources[index] = self.related_sources[index].change_related_id(id);
    }
}

/// These specify the relationship between things (using IDs)
#[derive(Copy, Clone, Deserialize, Serialize)]
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
    pub fn relate_source(before_source: &Source, after_source: &Source) -> (Relation, Relation) {
        let before = Relation::Before(before_source.id);
        let after = Relation::After(after_source.id);
        (after, before)
    }

    /// Returns the ID in this relations
    pub fn related_id(&self) -> u64 {
        match self {
            Relation::Before(id) |
            Relation::After(id) |
            Relation::Alternate(id) => *id,
        }
    }

    /// Changes the internal related ID by returning a new Relation of the same kind
    pub fn change_related_id(&self, id: u64) -> Relation {
        match self {
            Relation::Before(_) => Relation::Before(id),
            Relation::After(_) => Relation::After(id),
            Relation::Alternate(_) => Relation::Alternate(id),
        }
    }
}

#[derive(Copy, Clone, Deserialize, Serialize)]
// It makes sense to me to deliminate categories with an underscore
#[allow(non_camel_case_types)]
pub enum MediaType {
    Book_WebNovel,
}

/// A release date with a description
#[derive(Clone, Deserialize, Serialize)]
pub struct ReleaseDate {
    date: Date,
    descriptions: LangStrings,
}
impl ReleaseDate {
    /// Creates a new release date
    pub fn new(date: &str) -> Result<ReleaseDate, DateError> {
        let date = Date::parse_date(date)?;
        Ok(ReleaseDate {
            date,
            descriptions: LangStrings::new_empty(),
        })
    }

    /// Gets the language representation of this date (date, description)
    pub fn lang_str(&self, lang: Lang) -> (String, &str) {
        let description = self.descriptions.get_str(lang);
        (self.date.lang_str(lang), description)
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
    description: LangStrings,
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
            description: LangStrings::new_empty(),
        })
    }

    /// Adds a description from the language string
    pub fn add_description(&mut self, lang_string: LangString) {
        self.description.replace(lang_string);
    }
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
    /// Tries to parse the link type from the URL
    pub fn from_url(url: &str) -> Option<LinkType> {
        // Go through each variant to see if this URL belongs to a known site
        if url.contains("syosetu.com") {
            Some(LinkType::ShousetsukaNarou)
        } else if url.contains("wikipedia.org") {
            Some(LinkType::Wikipedia)
        } else {
            None
        }
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

/// A simple struct to hold the data for an image
pub struct ImageData {
    data: Vec<u8>,
    image_type: ImageType,
}
impl ImageData {
    /// Creates new image data
    pub fn new(data: Vec<u8>, image_type: ImageType) -> ImageData {
        ImageData { data, image_type }
    }
    /// Reads a Base64 string as an image. Uses the compressed data if it's a JPEG or PNG.
    /// Any other image type will be converted to PNG.
    fn read_base64_image(image_str: &str) -> Result<ImageData, SourceError> {
        let raw_data = base64::decode(image_str)
            .map_err(|_| Err(DataError::BadImageBase64))?;
        // Try to guess the format first
        let guessed_format = image::guess_format(&raw_data)
            .map_err(|_| Err(DataError::BadImageData))?;
        // If it's not PNG or JPG, read in the image and save it to a buffer
        match guessed_format {
            ImageFormat::JPEG => Ok( ImageData::new(raw_data, ImageType::Jpeg) ),
            ImageFormat::PNG => Ok( ImageData::new(raw_data, ImageType::Png) ),
            _ => {
                // Read in the image data
                let image = image::load_from_memory(&raw_data)
                    .map_err(|_| Err(DataError::BadImageData))?;
                // Save the image to a PNG buffer
                let mut png_buffer = Vec::new();
                image.write_to(&mut png_buffer, ImageOutputFormat::PNG)
                    // Although this error shouldn't happen since it's just to memory
                    .map_err(|_| Err(DataError::BadImageData))?;

                ImageData::new(png_buffer, ImageType::Png)
            },
        }
    }

    /// Saves the image to disk using a source item and its ID.
    /// Returns the image type.
    pub fn save(self, source_item: SourceItem, id: u64) -> Result<ImageType, SourceError> {
        // Get the path for the image
        let image_path = source_item.image_path(id, self.image_type);
        // Try to write the bytes to disk
        fs::write(image_path, self.data)
            .map_err(|_| Err(SourceError::FailedImageWrite( (source_item, id) )))?;
        // Return the image type
        Ok(self.image_type)
    }
}
