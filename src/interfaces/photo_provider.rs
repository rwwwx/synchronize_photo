use crate::entity::collections::{PhotoCollection, Username};
use crate::interfaces::errors::FsErrors;
use chrono::NaiveDate;
use std::collections::HashMap;

pub trait PhotoProvider {
    fn get_date_to_photo_collections(
        &self,
    ) -> Result<HashMap<NaiveDate, Vec<(Username, PhotoCollection)>>, FsErrors>;
}
