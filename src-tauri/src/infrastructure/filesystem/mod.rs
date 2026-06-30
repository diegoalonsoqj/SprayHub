//! Filesystem infrastructure: spray scanning, VTF thumbnails and atomic apply.

pub mod applier;
pub mod spray_scanner;
pub mod spray_writer;
pub mod vtf;
pub mod vtf_encode;

pub use applier::FsSprayApplier;
pub use spray_scanner::FsSprayRepository;
pub use spray_writer::FsSprayWriter;
