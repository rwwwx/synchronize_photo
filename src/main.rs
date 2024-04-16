mod domain;

use crate::domain::{FriendCollections, MissingPhotos, PhotoCollection};
use chrono::NaiveDate;
use std::collections::HashMap;

/// My Photos:
/// MyPhotos -> day1 -> photo_hash1
///          -> day2 -> photo_hash1, photo_hash2, photo_hash3
///          -> day3 -> photo_hash1
///
/// Photos of Friends:
/// MyFriendPhotos -> Friend1 -> day1 -> photo_hash1
///                           -> day2 -> photo_hash1, photo_hash2, photo_hash3
///                           -> day3 -> photo_hash1, photo_hash2
///                   Friend2 -> day1 -> photo_hash2
///                           -> day2 -> photo_hash1, photo_hash4
///                           -> day3 -> photo_hash1, photo_hash2

fn main() {
    env_logger::init()
}

pub fn find_missing_photos_for_day(
    my_collection: &PhotoCollection,
    friend_collections: &FriendCollections,
    day: &NaiveDate,
) -> (NaiveDate, MissingPhotos) {
    let mut missing_photos = HashMap::with_capacity(my_collection.len() + friend_collections.len());

    for (friend, friend_collection) in friend_collections {
        if is_different(my_collection, friend_collection) {
            let missing = friend_collection
                .difference(my_collection)
                .cloned()
                .collect::<PhotoCollection>();
            missing_photos.insert(friend.clone(), missing);
        }
    }

    missing_photos.iter().for_each(|(name, missing_photos)| {
        println!(
            "For day: '{}', you missing: [{:?}] - we can find it in '{}' collection.",
            day, missing_photos, name,
        )
    });

    (*day, missing_photos)
}

fn is_different(collection_a: &PhotoCollection, collection_b: &PhotoCollection) -> bool {
    collection_a.difference(collection_b).count().gt(&0)
}

#[cfg(test)]
mod test {
    use crate::domain::{FriendCollections, FriendName, PhotoCollection, PhotoId};
    use crate::find_missing_photos_for_day;
    use chrono::NaiveDate;
    use std::collections::HashMap;

    fn get_my_collection<T: Into<u64> + Clone>(of_elements: &[T]) -> PhotoCollection {
        of_elements
            .into_iter()
            .map(Clone::clone)
            .map(PhotoId::new)
            .collect::<PhotoCollection>()
    }

    fn get_friend_collection<T: Into<u64> + Clone>(
        for_denis: &[T],
        for_lev: &[T],
    ) -> FriendCollections {
        let mut friend_collection = HashMap::with_capacity(for_denis.len() + for_lev.len());

        friend_collection.insert(
            FriendName::new("Lev"),
            for_lev
                .into_iter()
                .map(Clone::clone)
                .map(PhotoId::new)
                .collect::<PhotoCollection>(),
        );
        friend_collection.insert(
            FriendName::new("Denis"),
            for_denis
                .into_iter()
                .map(Clone::clone)
                .map(PhotoId::new)
                .collect::<PhotoCollection>(),
        );

        friend_collection
    }

    #[test]
    fn should_find_missing_photos_from_both_friends() {
        let friend_collection = get_friend_collection(&[3u64, 5u64, 6u64], &[6u64]);
        let my_collection = get_my_collection(&[1u64, 2u64, 3u64]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let (_, missing_photos) =
            find_missing_photos_for_day(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).get(&FriendName::new("Lev")).is_some());
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Lev"))
            .unwrap()
            .contains(&PhotoId::new(6u64)));

        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .is_some());
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new(5u64)));
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new(6u64)))
    }

    #[test]
    fn should_find_missing_photos_from_one_friend() {
        let friend_collection = get_friend_collection(&[4u64, 5u64, 6u64], &[1u64, 2u64, 3u64]);
        let my_collection = get_my_collection(&[1u64, 2u64, 3u64]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let (_, missing_photos) =
            find_missing_photos_for_day(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).get(&FriendName::new("Lev")).is_none());

        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .is_some());
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new(4u64)));
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new(5u64)));
        assert!(dbg!(&missing_photos)
            .get(&FriendName::new("Denis"))
            .unwrap()
            .contains(&PhotoId::new(6u64)))
    }

    #[test]
    fn should_not_find_missing_photos() {
        let friend_collection = get_friend_collection(&[1u64, 2u64, 3u64], &[1u64, 2u64, 3u64]);
        let my_collection = get_my_collection(&[1u64, 2u64, 3u64]);
        let date = NaiveDate::parse_from_str("2024-04-15", "%Y-%m-%d").unwrap();

        let (_, missing_photos) =
            find_missing_photos_for_day(&my_collection, &friend_collection, &date);

        assert!(dbg!(&missing_photos).is_empty())
    }
}
