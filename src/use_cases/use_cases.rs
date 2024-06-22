use crate::entity::collections::{
    CollectionOfMissing, FriendCollections, MissingPhotos, PhotoCollection, Username,
};
use crate::interfaces::photo_provider::PhotoProvider;
use crate::use_cases::errors::UseCaseError;
use chrono::NaiveDate;
use std::collections::HashMap;

/// For simplicity lets assume that folder structure looks like this:
/// 2024-04-15 -> Lev   -> Photo1, Photo2, Photo3
///            -> My    -> Photo1, Photo5, Photo3
///            -> Denis -> Photo1, Photo2, Photo4
///
/// 2024-04-16 -> Lev   -> Photo1, Photo2, Photo3
///            -> My    -> Photo1, Photo5, Photo3
///            -> Denis -> Photo1, Photo2, Photo4

pub struct SynchronizeAllPhotosUseCase {
    my_name: Username,
    photo_provider: Box<dyn PhotoProvider>,
}

impl SynchronizeAllPhotosUseCase {
    pub fn new(my_name: &str, photo_provider: Box<dyn PhotoProvider>) -> Self {
        Self {
            my_name: Username::new(my_name),
            photo_provider,
        }
    }

    pub fn execute(&self) -> Result<CollectionOfMissing, UseCaseError> {
        let date_to_photo_collections: HashMap<NaiveDate, Vec<(Username, PhotoCollection)>> = self
            .photo_provider
            .get_date_to_photo_collections()
            .map_err(|e| UseCaseError::PhotoProvider(e.to_string()))?;

        let mut missing_for_all_time = CollectionOfMissing::new();

        for (day, username_to_collection) in date_to_photo_collections {
            let mut my_collection = PhotoCollection::default();
            let mut friend_collection = FriendCollections::default();

            for (username, user_collection) in username_to_collection {
                if username == self.my_name {
                    my_collection = user_collection
                } else {
                    friend_collection.insert(username, user_collection);
                }
            }

            missing_for_all_time.insert(
                day,
                FindMissingPhotoForDayUseCase.execute(&my_collection, &friend_collection, &day),
            );
        }

        Ok(missing_for_all_time)
    }
}

pub struct FindMissingPhotoForDayUseCase;

impl FindMissingPhotoForDayUseCase {
    pub fn execute(
        &self,
        my_collection: &PhotoCollection,
        friend_collections: &FriendCollections,
        for_day: &NaiveDate,
    ) -> MissingPhotos {
        let mut missing_photos =
            HashMap::with_capacity(my_collection.len() + friend_collections.len());

        for (friend_name, friend_collection) in friend_collections {
            if !my_collection.is_sync_needed_with(friend_collection) {
                continue;
            }

            let missing = friend_collection
                .difference(my_collection)
                .cloned()
                .collect::<PhotoCollection>();

            if !missing.is_empty() {
                log::debug!(
                    "For day: '{}', you missing: [{:?}] - we can find it in '{}' collection.",
                    for_day,
                    missing,
                    friend_name,
                );

                missing_photos.insert(friend_name.clone(), missing);
            }
        }

        missing_photos
    }
}

#[cfg(test)]
mod test {
    use crate::entity::collections::{FriendCollections, PhotoCollection, PhotoId, Username};
    use crate::use_cases::use_cases::FindMissingPhotoForDayUseCase;
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
            Username::new("Lev"),
            for_lev
                .iter()
                .map(Clone::clone)
                .map(PhotoId::new)
                .collect::<PhotoCollection>(),
        );
        friend_collection.insert(
            Username::new("Denis"),
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

        let use_case = FindMissingPhotoForDayUseCase;
        let missing_photos = use_case.execute(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).get(&Username::new("Lev")).is_some());
        assert!(dbg!(&missing_photos)
            .get(&Username::new("Lev"))
            .unwrap()
            .contains(&PhotoId::new("6u64")));

        assert!(dbg!(&missing_photos).get(&Username::new("Denis")).is_some());
        assert!(dbg!(&missing_photos)
            .get(&Username::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("5u64")));
        assert!(dbg!(&missing_photos)
            .get(&Username::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("6u64")))
    }

    #[test]
    fn should_find_missing_photos_from_one_friend() {
        let friend_collection =
            get_friend_collection(&["4u64", "5u64", "6u64"], &["1u64", "2u64", "3u64"]);
        let my_collection = get_my_collection(&["1u64", "2u64", "3u64"]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let use_case = FindMissingPhotoForDayUseCase;
        let missing_photos = use_case.execute(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).get(&Username::new("Lev")).is_none());

        assert!(dbg!(&missing_photos).get(&Username::new("Denis")).is_some());
        assert!(dbg!(&missing_photos)
            .get(&Username::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("4u64")));
        assert!(dbg!(&missing_photos)
            .get(&Username::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("5u64")));
        assert!(dbg!(&missing_photos)
            .get(&Username::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("6u64")))
    }

    #[test]
    fn should_not_find_missing_photos() {
        let friend_collection =
            get_friend_collection(&["1u64", "2u64", "3u64"], &["1u64", "2u64", "3u64"]);
        let my_collection = get_my_collection(&["1u64", "2u64", "3u64"]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let use_case = FindMissingPhotoForDayUseCase;
        let missing_photos = use_case.execute(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).is_empty())
    }

    #[test]
    fn should_find_missing_photo_from_one_friend() {
        let friend_collection = get_friend_collection(&["1u64", "2u64", "4u64"], &[]);
        let my_collection = get_my_collection(&["1u64", "2u64", "3u64"]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let use_case = FindMissingPhotoForDayUseCase;
        let missing_photos = use_case.execute(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).get(&Username::new("Lev")).is_none());
        assert!(dbg!(&missing_photos)
            .get(&Username::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("4u64")));
    }

    #[test]
    fn should_not_contains_empty_element() {
        let friend_collection = get_friend_collection(&["1u64", "2u64", "3u64"], &["1u64"]);
        let my_collection = get_my_collection(&["1u64", "2u64"]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let use_case = FindMissingPhotoForDayUseCase;
        let missing_photos = use_case.execute(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).get(&Username::new("Lev")).is_none());
        assert!(dbg!(&missing_photos)
            .get(&Username::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new("3u64")));
    }

    #[test]
    fn should_skip_collection_ordered() {
        use get_my_collection as get_photo_collection;

        let friend_collection = get_photo_collection(&["1u64", "2u64", "3u64"]);
        let my_collection = get_photo_collection(&["1u64", "2u64", "3u64"]);

        let is_needed = my_collection.is_sync_needed_with(&friend_collection);

        assert!(!dbg!(is_needed));
    }

    #[test]
    fn should_skip_collection_disordered() {
        use get_my_collection as get_photo_collection;

        let friend_collection = get_photo_collection(&["3u64", "2u64", "1u64"]);
        let my_collection = get_photo_collection(&["1u64", "2u64", "3u64"]);

        let is_needed = my_collection.is_sync_needed_with(&friend_collection);

        assert!(!dbg!(is_needed));
    }

    #[test]
    fn should_not_skip_collection_ordered() {
        use get_my_collection as get_photo_collection;

        let friend_collection = get_photo_collection(&["1u64", "2u64", "3u64", "4u64"]);
        let my_collection = get_photo_collection(&["1u64", "2u64", "3u64"]);

        let is_needed = my_collection.is_sync_needed_with(&friend_collection);

        assert!(dbg!(is_needed));
    }

    #[test]
    fn should_not_skip_collection_disordered() {
        use get_my_collection as get_photo_collection;

        let friend_collection = get_photo_collection(&["3u64", "2u64", "1u64", "4u64"]);
        let my_collection = get_photo_collection(&["1u64", "2u64", "3u64"]);

        let is_needed = my_collection.is_sync_needed_with(&friend_collection);

        assert!(dbg!(is_needed));
    }
}
