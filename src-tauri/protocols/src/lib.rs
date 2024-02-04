mod kitty_command;
mod traits;
mod utils;

#[cfg(feature = "hysteria")]
mod hysteria;
#[cfg(feature = "xray")]
mod xray;

#[cfg(feature = "xray")]
pub use xray::XrayCommandGroup;

#[cfg(feature = "hysteria")]
pub use hysteria::HysteriaCommandGroup;

pub use crate::traits::KittyCommandGroupTrait;
