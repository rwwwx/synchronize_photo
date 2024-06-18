use chrono::NaiveDate;
use std::collections::btree_set::Difference;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};

// pub type PhotoCollection = BTreeSet<PhotoId>;
pub type FriendCollections = HashMap<Username, PhotoCollection>;
pub type MissingPhotos = HashMap<Username, PhotoCollection>;
pub type CollectionOfMissing = BTreeMap<NaiveDate, MissingPhotos>;

#[derive(Default, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct PhotoCollection(BTreeSet<PhotoId>);

impl PhotoCollection {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, value: PhotoId) -> bool {
        self.0.insert(value)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<PhotoId> {
        self.0.difference(&other.0)
    }

    #[cfg(test)]
    pub fn contains(&self, value: &PhotoId) -> bool {
        self.0.contains(value)
    }

    pub fn is_sync_needed_with(&self, other: &Self) -> bool {
        !self.0.is_empty() && !self.is_collection_hashes_eq(other)
    }

    fn is_collection_hashes_eq(&self, other: &Self) -> bool {
        self.hash().eq(&other.hash())
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.iter().for_each(|element| element.hash(&mut hasher));
        hasher.finish()
    }
}

impl FromIterator<PhotoId> for PhotoCollection {
    fn from_iter<I: IntoIterator<Item = PhotoId>>(iter: I) -> Self {
        Self(BTreeSet::from_iter(iter))
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct PhotoId(String);

impl PhotoId {
    pub fn new<H: Into<String>>(hash: H) -> Self {
        PhotoId(hash.into())
    }
}

impl Display for PhotoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Username(String);

#[allow(dead_code)]
impl Username {
    pub fn new<H: Into<String>>(name: H) -> Self {
        Username(name.into())
    }
}

impl From<String> for Username {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
