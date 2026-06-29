//! Filesystem infrastructure: spray scanning, VTF thumbnails and atomic apply.

pub mod applier;
pub mod spray_scanner;
pub mod vtf;

pub use applier::FsSprayApplier;
pub use spray_scanner::FsSprayRepository;
