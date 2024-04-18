use crate::types::{
    CollectionOfMissing, FriendCollections, MissingPhotos, PhotoCollection, PhotoId,
};
use chrono::NaiveDate;
use clap::Parser;
use sha256::digest as sha256_digest;
use std::collections::HashMap;
use std::fs::read as fs_read;
use std::fs::{read_dir as fs_read_dir, DirEntry};
use std::path::PathBuf;
use thiserror::Error;

/// For simplicity lets assume that folder structure looks like this:
/// 2024-04-15 -> Lev   -> Photo1, Photo2, Photo3
///            -> My    -> Photo1, Photo5, Photo3
///            -> Denis -> Photo1, Photo2, Photo4
///
/// 2024-04-16 -> Lev   -> Photo1, Photo2, Photo3
///            -> My    -> Photo1, Photo5, Photo3
///            -> Denis -> Photo1, Photo2, Photo4

/// To be honest, these errors and error handling is redundant,
///  but I want show you that I can do it.
///
/// Also, this algorithm can be paralleled via `std::thread` threads and `std::sync::mpsc` channels.
#[derive(Error, Debug)]
pub enum Errors {
    #[error("Cannot read directory: {0}.")]
    CannotReadDirectory(PathBuf),

    #[error("Cannot read file: {0}.")]
    CannotReadFile(PathBuf),

    #[error("Cannot get dir entry.")]
    CannotGetDirEntry,

    #[error("Cannot parse date.")]
    DateParsingFailure,
}

#[derive(Debug, Parser)]
pub struct PhotoSyncCli {
    #[clap(default_value = "My")]
    my_folder_name: String,

    #[clap(default_value = "./photo_example")]
    path_to_photos: PathBuf,
}

impl PhotoSyncCli {
    pub fn sync_photos(&self) -> Result<CollectionOfMissing, Errors> {
        let days_dir = fs_read_dir(&self.path_to_photos)
            .map_err(|_| Errors::CannotReadDirectory(self.path_to_photos.clone()))?;

        let mut missing = CollectionOfMissing::new();

        for day_dir in days_dir {
            let day_dir_entry = day_dir.map_err(|_| Errors::CannotGetDirEntry)?;
            let date = Self::get_date(&day_dir_entry)?;

            let mut my_collection = PhotoCollection::new();
            let mut friend_collection = FriendCollections::new();

            for friend_dir in fs_read_dir(day_dir_entry.path())
                .map_err(|_| Errors::CannotReadDirectory(self.path_to_photos.clone()))?
            {
                let friend_dir = friend_dir.map_err(|_| Errors::CannotGetDirEntry)?;
                let friend_name = friend_dir
                    .file_name()
                    .to_str()
                    .map(ToOwned::to_owned)
                    .ok_or(Errors::CannotGetDirEntry)?;

                if friend_name == self.my_folder_name {
                    my_collection = Self::process_day(friend_dir.path())?;
                } else {
                    friend_collection
                        .insert(friend_name.into(), Self::process_day(friend_dir.path())?);
                }
            }

            missing.insert(
                date,
                find_missing_photos_for_day(&my_collection, &friend_collection, &date),
            );
        }

        Ok(missing)
    }

    fn process_day(path_to_day: PathBuf) -> Result<PhotoCollection, Errors> {
        let photo_dir =
            fs_read_dir(&path_to_day).map_err(|_| Errors::CannotReadDirectory(path_to_day))?;
        let mut photos_of_day = PhotoCollection::with_capacity(photo_dir.size_hint().0);

        for photo in photo_dir {
            let photo_hash =
                Self::get_hash_of_photo(photo.map_err(|_| Errors::CannotGetDirEntry)?.path())?;
            photos_of_day.insert(PhotoId::new(photo_hash));
        }

        Ok(photos_of_day)
    }

    fn get_date(path_to_day_dir: &DirEntry) -> Result<NaiveDate, Errors> {
        path_to_day_dir
            .file_name()
            .to_str()
            .and_then(|date| NaiveDate::parse_from_str(date, "%Y-%m-%d").ok())
            .ok_or(Errors::DateParsingFailure)
    }

    fn get_hash_of_photo(path: PathBuf) -> Result<String, Errors> {
        Ok(sha256_digest(
            fs_read(&path).map_err(|_| Errors::CannotReadFile(path))?,
        ))
    }
}

fn find_missing_photos_for_day(
    my_collection: &PhotoCollection,
    friend_collections: &FriendCollections,
    day: &NaiveDate,
) -> MissingPhotos {
    let mut missing_photos = HashMap::with_capacity(my_collection.len() + friend_collections.len());

    for (friend_name, friend_collection) in friend_collections {
        if friend_collection.is_empty() || !is_different(my_collection, friend_collection) {
            continue;
        }

        let missing = friend_collection
            .difference(my_collection)
            .cloned()
            .collect::<PhotoCollection>();

        if !missing.is_empty() {
            missing_photos.insert(friend_name.clone(), missing);
        }
    }

    missing_photos.iter().for_each(|(name, missing_photos)| {
        log::debug!(
            "For day: '{}', you missing: [{:?}] - we can find it in '{}' collection.",
            day,
            missing_photos,
            name,
        )
    });

    missing_photos
}

fn is_different(collection_a: &PhotoCollection, collection_b: &PhotoCollection) -> bool {
    collection_a
        .symmetric_difference(collection_b)
        .count()
        .gt(&0)
}

#[cfg(test)]
mod test {
    use super::find_missing_photos_for_day;
    use crate::types::{FriendCollections, FriendName, PhotoCollection, PhotoId};
    use chrono::NaiveDate;
    use std::collections::HashMap;

    fn get_my_collection<T: Into<String> + Clone>(of_elements: &[T]) -> PhotoCollection {
        of_elements
            .iter()
            .map(Clone::clone)
            .map(PhotoId::new)
            .collect::<PhotoCollection>()
    }

    fn get_friend_collection<T: Into<String> + Clone>(
        for_denis: &[T],
        for_lev: &[T],
    ) -> FriendCollections {
        let mut friend_collection = HashMap::with_capacity(for_denis.len() + for_lev.len());

        friend_collection.insert(
            FriendName::new("Lev"),
            for_lev
                .iter()
                .map(Clone::clone)
                .map(PhotoId::new)
                .collect::<PhotoCollection>(),
        );
        friend_collection.insert(
            FriendName::new("Denis"),
            for_denis
                .iter()
                .map(Clone::clone)
                .map(PhotoId::new)
                .collect::<PhotoCollection>(),
        );

        friend_collection
    }

    #[test]
    fn should_find_missing_photos_from_both_friends() {
        let friend_collection = get_friend_collection(&["3u64", "5u64", "6u64"], &["6u64"]);
        let my_collection = get_my_collection(&["1u64", "2u64", "3u64"]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let missing_photos = find_missing_photos_for_day(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).get(&FriendName::new("Lev")).is_some());
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Lev"))
            .unwrap()
            .contains(&PhotoId::new("6u64")));

        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .is_some());
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("5u64")));
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("6u64")))
    }

    #[test]
    fn should_find_missing_photos_from_one_friend() {
        let friend_collection =
            get_friend_collection(&["4u64", "5u64", "6u64"], &["1u64", "2u64", "3u64"]);
        let my_collection = get_my_collection(&["1u64", "2u64", "3u64"]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let missing_photos = find_missing_photos_for_day(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).get(&FriendName::new("Lev")).is_none());

        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .is_some());
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("4u64")));
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("5u64")));
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("6u64")))
    }

    #[test]
    fn should_not_find_missing_photos() {
        let friend_collection =
            get_friend_collection(&["1u64", "2u64", "3u64"], &["1u64", "2u64", "3u64"]);
        let my_collection = get_my_collection(&["1u64", "2u64", "3u64"]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let missing_photos = find_missing_photos_for_day(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).is_empty())
    }

    #[test]
    fn should_find_missing_photo_from_one_friend() {
        let friend_collection = get_friend_collection(&["1u64", "2u64", "4u64"], &[]);
        let my_collection = get_my_collection(&["1u64", "2u64", "3u64"]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let missing_photos = find_missing_photos_for_day(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).get(&FriendName::new("Lev")).is_none());
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("4u64")));
    }

    #[test]
    fn should_not_contains_empty_element() {
        let friend_collection = get_friend_collection(&["1u64", "2u64", "3u64"], &["1u64"]);
        let my_collection = get_my_collection(&["1u64", "2u64"]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let missing_photos = find_missing_photos_for_day(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).get(&FriendName::new("Lev")).is_none());
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("3u64")));
    }
}
