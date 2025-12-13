#[cfg(not(feature = "locking"))]
pub mod random;

#[cfg(feature = "locking")]
pub mod random_locking;

pub mod random_plugin;

#[cfg(feature = "locking")]
pub use random_locking as random;
