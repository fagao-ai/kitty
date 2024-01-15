#[cfg(feature = "hysteria")]
pub mod hysteria;
#[cfg(feature = "xray")]
pub mod xray;

#[cfg(feature = "hysteria")]
pub use hysteria::HysteriaManager;

#[cfg(feature = "xray")]
pub use xray::XrayManager;

pub mod traits;
mod types;

pub use traits::CommandManagerTrait;
