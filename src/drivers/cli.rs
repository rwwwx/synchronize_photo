use std::path::PathBuf;

use clap::Parser;

use crate::drivers::errors::DriverError;
use crate::drivers::photo_provider::PhotoProviderFs;
use crate::entity::collections::CollectionOfMissing;
use crate::use_cases::use_cases::SynchronizeAllPhotosUseCase;

#[derive(Debug, Parser)]
pub struct PhotoSyncCli {
    #[clap(default_value = "My")]
    my_folder_name: String,
    #[clap(default_value = "./photo_example")]
    path_to_photos: PathBuf,
}

impl PhotoSyncCli {
    pub fn sync_photos(&self) -> Result<CollectionOfMissing, DriverError> {
        Ok(SynchronizeAllPhotosUseCase::new(
            &self.my_folder_name,
            Box::new(PhotoProviderFs::new(&self.path_to_photos)),
        )
        .execute()?)
    }
}
