use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to lock.")]
    FailedToLock,
    #[error("Duplicate BlockBehavior entry \"{0}\".")]
    DuplicateBlockEntry(String),
    #[error("Registry overflow, too many entries in BlockRegistry.")]
    RegistryOverflow,
    #[error("Block not found in registry \"{0}\".")]
    BlockNotFound(String),
    #[error("Invalid Property ID ({0}).")]
    InvalidPropertyId(u8),
    #[error("Invalid conversion.")]
    InvalidConversion,
}

pub type Result<T> = std::result::Result<T, Error>;