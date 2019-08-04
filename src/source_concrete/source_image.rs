//! All of the image related things that we can do in the API

use std::fs;
use std::path::{Path, PathBuf};

use base64;
use image::{self, ImageFormat, ImageOutputFormat};

use super::{source_file_path};
use crate::source::{ImageType, SourceItem, Record, RecordInfo};
use crate::source::err::{SourceError};

/// Gets the image directory for the item
pub fn image_dir(source_item: SourceItem) -> PathBuf {
    match source_item {
        SourceItem::Character => source_file_path("imgCharacters"),
        SourceItem::Company => source_file_path("imgCompanies"),
        SourceItem::Person => source_file_path("imgPeople"),
        SourceItem::Source => source_file_path("imgSources"),
        SourceItem::UniverseTag => source_file_path("imgUniverseTags"),
    }
}

/// Finds the image path for the given image type
fn find_image_path<T: RecordInfo + Clone>(record: &Record<T>, image_type: ImageType) -> PathBuf {
    // The image will be directly underneath the directory
    let (source_item, id) = record.source_item_pair();
    image_dir(source_item)
        .join(format!("{}.{}", id, image_type.extension()))
}
/// Finds the path to the image for this record
pub fn image_path<T: RecordInfo + Clone>(record: &Record<T>)
-> Option< Result<PathBuf, SourceError> > {
    record.image_type
        .map(|image_type| find_image_path(record, image_type))
        // Check the path itself for existence
        .map(|image_path|
            if image_path.is_file() {
                Ok(image_path)
            } else {
                Err(SourceError::ImageMissing(record.source_item_pair()))
            }
        )
}


/// A simple struct to hold the data for an image
pub struct ImageData {
    data: Vec<u8>,
    image_type: ImageType,
}
impl ImageData {
    pub fn image_type(&self) -> ImageType { self.image_type }

    /// Creates new image data
    pub fn new(data: Vec<u8>, image_type: ImageType) -> ImageData {
        ImageData { data, image_type }
    }
    /// Reads a Base64 string as an image. Uses the compressed data if it's a JPEG or PNG.
    /// Any other image type will be converted to PNG.
    pub fn read_base64_image(image_str: &str) -> Result<ImageData, SourceError> {
        let raw_data = base64::decode(image_str)
            .map_err(|_| SourceError::ImageBadBase64)?;
        // Try to guess the format first
        let guessed_format = image::guess_format(&raw_data)
            .map_err(|_| SourceError::ImageBadData)?;
        // If it's not PNG or JPG, read in the image and save it to a buffer
        match guessed_format {
            ImageFormat::JPEG => Ok( ImageData::new(raw_data, ImageType::Jpeg) ),
            ImageFormat::PNG => Ok( ImageData::new(raw_data, ImageType::Png) ),
            _ => {
                // Read in the image data
                let image = image::load_from_memory(&raw_data)
                    .map_err(|_| SourceError::ImageBadData)?;
                // Save the image to a PNG buffer
                let mut png_buffer = Vec::new();
                image.write_to(&mut png_buffer, ImageOutputFormat::PNG)
                    // Although this error shouldn't happen since it's just to memory
                    .map_err(|_| SourceError::ImageBadData)?;

                Ok(ImageData::new(png_buffer, ImageType::Png))
            },
        }
    }

    /// Saves the image while modifying the record to reflect this
    pub fn save_and_record<T: RecordInfo + Clone>(self, record: &mut Record<T>)
    -> Result<(), SourceError> {
        // Take the image type out before we consume self by saving
        let image_type = self.image_type;
        self.save(find_image_path(&record, image_type))?;
        // We can now safely update the record since the save succeeded
        record.image_type = Some(image_type);
        Ok(())
    }

    /// Saves the image to disk using a source item and its ID.
    /// Returns the image type.
    pub fn save(self, image_path: impl AsRef<Path>) -> Result<(), SourceError> {
        let image_path = image_path.as_ref();
        // Try to write the bytes to disk
        fs::write(image_path, self.data)
            .map_err(|_| SourceError::ImageWriteFailed(image_path.to_path_buf()))?;
        // Return the image type
        Ok(())
    }
}
