use std::collections::HashMap;
use std::fs::read as fs_read;
use std::fs::{read_dir as fs_read_dir, DirEntry};
use std::path::PathBuf;

use chrono::NaiveDate;
use sha256::digest as sha256_digest;

use crate::entity::collections::{PhotoCollection, PhotoId, Username};
use crate::interfaces::errors::FsErrors;
use crate::interfaces::photo_provider::PhotoProvider;

pub struct PhotoProviderFs {
    path_to_photos: PathBuf,
}

impl Default for PhotoProviderFs {
    fn default() -> Self {
        Self {
            path_to_photos: PathBuf::from("./photo_example"),
        }
    }
}

impl PhotoProviderFs {
    pub fn new<T: Into<PathBuf>>(path_to_photos: T) -> Self {
        Self {
            path_to_photos: path_to_photos.into(),
        }
    }

    fn get_date(path_to_day_dir: &DirEntry) -> Result<NaiveDate, FsErrors> {
        path_to_day_dir
            .file_name()
            .to_str()
            .and_then(|date| NaiveDate::parse_from_str(date, "%Y-%m-%d").ok())
            .ok_or(FsErrors::DateParsingFailure)
    }

    fn process_day(path_to_day: PathBuf) -> Result<PhotoCollection, FsErrors> {
        let photo_dir =
            fs_read_dir(&path_to_day).map_err(|_| FsErrors::CannotReadDirectory(path_to_day))?;
        let mut photos_of_day = PhotoCollection::new();

        for photo in photo_dir {
            let photo_hash =
                Self::get_hash_of_photo(photo.map_err(|_| FsErrors::CannotGetDirEntry)?.path())?;
            photos_of_day.insert(PhotoId::new(photo_hash));
        }

        Ok(photos_of_day)
    }

    fn get_hash_of_photo(path: PathBuf) -> Result<String, FsErrors> {
        Ok(sha256_digest(
            fs_read(&path).map_err(|_| FsErrors::CannotReadFile(path))?,
        ))
    }
}

impl PhotoProvider for PhotoProviderFs {
    fn get_date_to_photo_collections(
        &self,
    ) -> Result<HashMap<NaiveDate, Vec<(Username, PhotoCollection)>>, FsErrors> {
        let mut res = HashMap::default();

        let days_dir = fs_read_dir(&self.path_to_photos)
            .map_err(|_| FsErrors::CannotReadDirectory(self.path_to_photos.clone()))?;

        for day_dir in days_dir {
            let day_dir_entry = day_dir.map_err(|_| FsErrors::CannotGetDirEntry)?;
            let date = Self::get_date(&day_dir_entry)?;
            let mut photo_collections_for_day = Vec::default();

            for user_dir in fs_read_dir(day_dir_entry.path())
                .map_err(|_| FsErrors::CannotReadDirectory(self.path_to_photos.clone()))?
            {
                let friend_dir = user_dir.map_err(|_| FsErrors::CannotGetDirEntry)?;
                let username = Username::new(
                    friend_dir
                        .file_name()
                        .to_str()
                        .map(ToOwned::to_owned)
                        .ok_or(FsErrors::CannotGetDirEntry)?,
                );
                let photo_collection_for_day = Self::process_day(friend_dir.path())?;
                photo_collections_for_day.push((username, photo_collection_for_day));
            }

            res.insert(date, photo_collections_for_day);
        }

        Ok(res)
    }
}
