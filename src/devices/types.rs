//! Audio device type definitions
//!
//! This module contains the core data structures used to represent
//! audio device information across different platforms and audio systems.

use serde::{Deserialize, Serialize};

/// Information about a single audio device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDeviceInfo {
    /// Human-readable device name
    pub name: String,
    /// Type of device (Input, Output, or Input/Output)
    pub device_type: String,
    /// Number of input channels available
    pub input_channels: u32,
    /// Number of output channels available
    pub output_channels: u32,
    /// List of supported sample rates in Hz
    pub supported_sample_rates: Vec<u32>,
    /// List of supported buffer sizes in samples
    pub supported_buffer_sizes: Vec<u32>,
    /// Default sample rate in Hz
    pub default_sample_rate: u32,
    /// Default buffer size in samples
    pub default_buffer_size: u32,
    /// Audio driver/system name (CPAL, ALSA, etc.)
    pub driver: String,
}

/// System-wide audio information
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemAudioInfo {
    /// List of all detected audio devices
    pub devices: Vec<AudioDeviceInfo>,
    /// Name of the default input device, if any
    pub default_input: Option<String>,
    /// Name of the default output device, if any
    pub default_output: Option<String>,
    /// Total number of devices with input capabilities
    pub total_input_devices: usize,
    /// Total number of devices with output capabilities
    pub total_output_devices: usize,
}

impl AudioDeviceInfo {
    /// Create a new AudioDeviceInfo with default values
    pub fn new(name: String, driver: String) -> Self {
        Self {
            name,
            device_type: "Unknown".to_string(),
            input_channels: 0,
            output_channels: 0,
            supported_sample_rates: Vec::new(),
            supported_buffer_sizes: vec![64, 128, 256, 512, 1024, 2048, 4096],
            default_sample_rate: 44100,
            default_buffer_size: 1024,
            driver,
        }
    }

    /// Check if this device has input capabilities
    pub fn has_input(&self) -> bool {
        self.input_channels > 0
    }

    /// Check if this device has output capabilities
    pub fn has_output(&self) -> bool {
        self.output_channels > 0
    }

    /// Update the device type based on input/output capabilities
    pub fn update_device_type(&mut self) {
        self.device_type = match (self.has_input(), self.has_output()) {
            (true, true) => "Input/Output".to_string(),
            (true, false) => "Input".to_string(),
            (false, true) => "Output".to_string(),
            (false, false) => "Unknown".to_string(),
        };
    }

    /// Check if a given sample rate is supported
    pub fn supports_sample_rate(&self, rate: u32) -> bool {
        if self.supported_sample_rates.is_empty() {
            // If no specific rates are listed, assume common rates are supported
            matches!(rate, 8000 | 11025 | 22050 | 44100 | 48000 | 88200 | 96000 | 176400 | 192000)
        } else {
            self.supported_sample_rates.contains(&rate)
        }
    }

    /// Check if a given buffer size is supported
    pub fn supports_buffer_size(&self, size: u32) -> bool {
        self.supported_buffer_sizes.contains(&size)
    }
}

impl SystemAudioInfo {
    /// Create a new SystemAudioInfo from a list of devices
    pub fn from_devices(devices: Vec<AudioDeviceInfo>) -> Self {
        let input_count = devices.iter().filter(|d| d.has_input()).count();
        let output_count = devices.iter().filter(|d| d.has_output()).count();

        // Try to determine default devices
        let default_input = devices.iter()
            .find(|d| d.has_input() && (d.name.contains("default") || d.name.contains("hw:0")))
            .map(|d| d.name.clone());

        let default_output = devices.iter()
            .find(|d| d.has_output() && (d.name.contains("default") || d.name.contains("hw:0")))
            .map(|d| d.name.clone());

        Self {
            devices,
            default_input,
            default_output,
            total_input_devices: input_count,
            total_output_devices: output_count,
        }
    }

    /// Get all input devices
    pub fn input_devices(&self) -> impl Iterator<Item = &AudioDeviceInfo> {
        self.devices.iter().filter(|d| d.has_input())
    }

    /// Get all output devices
    pub fn output_devices(&self) -> impl Iterator<Item = &AudioDeviceInfo> {
        self.devices.iter().filter(|d| d.has_output())
    }

    /// Find a device by name
    pub fn find_device(&self, name: &str) -> Option<&AudioDeviceInfo> {
        self.devices.iter().find(|d| d.name == name)
    }

    /// Get devices by driver type
    pub fn devices_by_driver(&self, driver: &str) -> impl Iterator<Item = &AudioDeviceInfo> {
        self.devices.iter().filter(move |d| d.driver == driver)
    }
}
