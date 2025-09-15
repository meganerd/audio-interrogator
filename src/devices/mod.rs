//! Device interrogation modules
//!
//! This module contains the core functionality for interrogating audio devices
//! across different platforms and audio systems.

pub mod types;
pub mod cpal_devices;
pub mod alsa_devices;

pub use types::*;
pub use cpal_devices::get_cpal_devices;

#[cfg(target_os = "linux")]
pub use alsa_devices::get_alsa_devices;

#[cfg(not(target_os = "linux"))]
pub fn get_alsa_devices() -> anyhow::Result<Vec<AudioDeviceInfo>> {
    Ok(Vec::new())
}
