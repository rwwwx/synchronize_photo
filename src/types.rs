use chrono::NaiveDate;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Display;

pub type PhotoCollection = BTreeSet<PhotoId>;
pub type FriendCollections = HashMap<FriendName, PhotoCollection>;
pub type MissingPhotos = HashMap<FriendName, PhotoCollection>;
pub type CollectionOfMissing = BTreeMap<NaiveDate, MissingPhotos>;

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
pub struct FriendName(String);

#[allow(dead_code)]
impl FriendName {
    pub fn new<H: Into<String>>(name: H) -> Self {
        FriendName(name.into())
    }
}

impl From<String> for FriendName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for FriendName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
