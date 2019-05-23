use std::fs;
use std::path::{Path, PathBuf};

use base64;
use image::{self, ImageFormat, ImageOutputFormat};

use serde_derive::{Deserialize, Serialize};

use crate::source::{Record, RecordInfo, SourceError};

/// All of the image related methods for the Record
impl <T: RecordInfo> Record<T> {
    /// Finds the path to the image for this record
    pub fn image_path(&self, image_type: ImageType) -> PathBuf {
        // The image will be directly underneath the directory
        let source_item = self.record_info.source_item();
        source_item.image_dir().join(format!("{}.{}", self.id, image_type.extension()))
    }
    /// Checks if the image for this record exists
    pub fn does_image_exist(&self) -> Option< Result<(), SourceError> > {
        if let Some(image_type) = self.image_type {
            let image_path = self.image_path(image_type);
            if image_path.is_file() {
                Some(Ok(()))
            } else {
                Some(Err(SourceError::ImageMissing(self.source_item_pair())))
            }
        } else {
            None
        }
    }
    /// Adds an image to the record
    pub fn add_image(&mut self, image_data: ImageData) -> Result<(), SourceError> {
        // Get the image type before we save the image to disk
        let image_type = image_data.image_type;
        image_data.save(self.image_path(image_type))?;
        self.image_type = Some(image_type);
        Ok(())
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
