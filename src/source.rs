mod source_info;
pub mod ui;
pub use source_info::{SourceInfo};

use std::collections::{HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use reqwest::{Url};

use serde_derive::{Deserialize, Serialize};

use crate::lang::{self, Lang, LangSelect, LangString, LangStrings};

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

trait SourceFile {
    const FILE: &'static str;

    fn file_path() -> PathBuf {
        [SOURCE_FOLDER, Self::FILE].iter().collect()
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Universe {
    names: LangStrings,
    id: u64,
    series: Vec<Series>,
    related_links: Vec<RelatedLink>,
}
impl Universe {
    /// Creates a new empty Universe with a single name
    pub fn new(name: LangString, id: u64) -> Universe {
        Universe {
            names: lang::new_lang_strings(name),
            id,
            series: Vec::new(),
            related_links: Vec::new(),
        }
    }
    pub fn name(&self, lang: LangSelect) -> &str {
        self.names.get(&lang).map_or("", |n| n.as_str())
    }
    pub fn id(&self) -> u64 { self.id }
    pub fn series(&self) -> &Vec<Series> { &self.series }
}
impl SourceFile for Universe {
    const FILE: &'static str = "universes.json";
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Series {
    names: LangStrings,
    id: u64,
    arcs: Vec<Arc>,
    related_links: Vec<RelatedLink>,
}
impl Series {
    /// Creates a new empty series with the name
    pub fn new(name: LangString, id: u64) -> Series {
        Series {
            names: lang::new_lang_strings(name),
            id,
            arcs: Vec::new(),
            related_links: Vec::new(),
        }
    }
    pub fn name(&self, lang: LangSelect) -> &str {
        self.names.get(&lang).map_or("", |n| n.as_str())
    }
    pub fn id(&self) -> u64 { self.id }
    pub fn arcs(&self) -> &Vec<Arc> { &self.arcs }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Arc {
    names: LangStrings,
    id: u64,
    related_links: Vec<RelatedLink>,
}
impl Arc {
    pub fn new(name: LangString, id: u64) -> Arc {
        Arc {
            names: lang::new_lang_strings(name),
            id,
            related_links: Vec::new(),
        }
    }
    pub fn name(&self, lang: LangSelect) -> &str {
        self.names.get(&lang).map_or("", |n| n.as_str())
    }
    pub fn id(&self) -> u64 { self.id }
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
    description: LangStrings,
    /// Release dates for this source in YYYY-MM-DD. Also includes notes about the release
    release_dates: Vec<ReleaseDate>,
    /// Other sources that are related to this one
    related_sources: Vec<Relation>,
    /// Related links to websites on this source
    related_links: Vec<RelatedLink>,
}
impl Source {
    /// Creates a new empty source
    pub fn new(name: LangString, media_type: MediaType, id: u64) -> Source {
        // Create the empty description strings
        let mut description = HashMap::new();
        for lang in LangSelect::all_langs() {
            description.insert(lang, String::new());
        }

        Source {
            names: lang::new_lang_strings(name),
            id,
            media_type,
            universe: None,
            series: None,
            arc: None,
            description,
            release_dates: Vec::new(),
            related_sources: Vec::new(),
            related_links: Vec::new(),
        }
    }
    pub fn name(&self, lang: LangSelect) -> &str {
        self.names.get(&lang).map_or("", |n| n.as_str())
    }
    pub fn id(&self) -> u64 { self.id }
    pub fn universe(&self) -> &Option<u64> { &self.universe }
    pub fn series(&self) -> &Option<u64> { &self.series }
    pub fn arc(&self) -> &Option<u64> { &self.arc }
    pub fn related_sources(&self) -> &Vec<Relation> { &self.related_sources }

    /// Links this source before the other
    pub fn relate_before(&mut self, other: &mut Source) {
        let (after, before) = Relation::relate(&self, &other);
        self.related_sources.push(after);
        other.related_sources.push(before);
    }
    /// Changes the ID of a related source
    pub fn modify_related_source_id(&mut self, index: usize, id: u64) {
        self.related_sources[index] = self.related_sources[index].change_related_id(id);
    }
}
impl SourceFile for Source {
    const FILE: &'static str = "sources.json";
}

/// These specify the relationship between sources (using IDs)
#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum Relation {
    /// Some other source that happens before
    Before(u64),
    /// Some other source that happens after
    After(u64),
}
impl Relation {
    /// Gives a pair of relations with the first source happening before the second
    pub fn relate(before_source: &Source, after_source: &Source) -> (Relation, Relation) {
        let before = Relation::Before(before_source.id());
        let after = Relation::After(after_source.id());
        (after, before)
    }

    /// Returns the ID in this relations
    pub fn related_id(&self) -> u64 {
        match self {
            Relation::Before(id) |
            Relation::After(id) => *id,
        }
    }

    /// Changes the internal related ID by returning a new Relation of the same kind
    pub fn change_related_id(&self, id: u64) -> Relation {
        match self {
            Relation::Before(_) => Relation::Before(id),
            Relation::After(_) => Relation::After(id),
        }
    }
}

#[derive(Copy, Clone, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum MediaType {
    Book_WebNovel,
}
impl MediaType {
    /// Returns all of the possible media types
    pub fn all_media_types() -> Vec<MediaType> {
        vec![
            MediaType::Book_WebNovel,
        ]
    }

    /// Converts the media type into a string in the given language
    pub fn lang_str(&self, lang: LangSelect) -> &'static str {
        match self {
            MediaType::Book_WebNovel => match lang {
                LangSelect::EN => "Book - Web Novel",
                LangSelect::JP => "本 - Web小説",
            },
        }
    }
    /// Gets the list of media types based on the language
    pub fn lang_list(lang: LangSelect) -> Vec<&'static str> {
        MediaType::all_media_types().into_iter()
            .map(|media_type| media_type.lang_str(lang))
            .collect()
    }
    /// Attempts to convert a string into the media type
    pub fn from_str(media_type_str: &str) -> Option<MediaType> {
        // Check each language
        for lang in LangSelect::all_langs() {
            // Now check each media type
            for (m_str, media_type) in MediaType::lang_list(lang).into_iter()
            .zip(MediaType::all_media_types()) {
                if m_str == media_type_str {
                    return Some(media_type);
                }
            }
        }
        None
    }
}

/// A release date with a description
#[derive(Clone, Deserialize, Serialize)]
pub struct ReleaseDate {
    year: u32,
    month: u8,
    day: u8,
    description: LangStrings,
}
impl ReleaseDate {
    /// Creates a new release date with just a single language description
    pub fn new(date: &str, description: LangString, lang: Lang) -> Result<ReleaseDate, String> {
        // Try to parse the date
        let year = date[0..4].parse().map_err(|_| lang.date_err(date))?;
        let month = date[5..7].parse().map_err(|_| lang.date_err(date))?;
        let day = date[8..10].parse().map_err(|_| lang.date_err(date))?;
        Ok(ReleaseDate {
            year,
            month,
            day,
            description: lang::new_lang_strings(description),
        })
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RelatedLink {
    /// The URL of the link
    url: String,
    /// A description of the link
    description: LangStrings,
}
impl RelatedLink {
    /// Tries to create a related link from the url and description.
    /// Fails if the url isn't well-formed.
    pub fn new(url: impl ToString, description: LangString) -> Result<RelatedLink, String> {
        // TODO Maybe make this error translatable
        let url = url.to_string();
        Url::parse(&url).map_err(|e| e.to_string())?;
        Ok(RelatedLink {
            url,
            description: lang::new_lang_strings(description),
        })
    }
}
