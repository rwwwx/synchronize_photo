use std::collections::{HashMap, HashSet};
use std::fmt::Display;

pub type PhotoCollection = HashSet<PhotoId>;
pub type FriendCollections = HashMap<FriendName, PhotoCollection>;
pub type MissingPhotos = HashMap<FriendName, PhotoCollection>;
// pub type CollectionOfMissing = HashMap<NaiveDate, MissingPhotos>;

#[derive(Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct PhotoId(u64);

impl PhotoId {
    pub fn new<H: Into<u64>>(hash: H) -> Self {
        PhotoId(hash.into())
    }
}

impl Display for PhotoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct FriendName(String);

impl FriendName {
    pub fn new<H: Into<String>>(name: H) -> Self {
        FriendName(name.into())
    }
}

impl Display for FriendName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
