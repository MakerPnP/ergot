//! Profiles describe how a single device interacts with the outside world
//!
//! Profiles have a couple of responsibilities, including managing all external interfaces,
//! as well as handling any routing outside of the device.

pub mod direct_edge;
pub mod null;

//#[cfg(any(feature = "tokio-std", feature = "embassy-net-v0_7"))]
#[cfg(any(feature = "tokio-std"))]
pub mod direct_router;
