# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure
- Cross-platform audio device detection using CPAL
- Linux-specific ALSA integration for detailed hardware information
- Comprehensive device information collection including:
  - Input and output channel counts
  - Supported sample rates and buffer sizes
  - Device types and drivers
  - Default audio settings
- Command-line interface with multiple output formats
- JSON output support for programmatic integration
- Verbose mode for detailed device inspection
- Summary mode for quick device overview
- **Device filtering capabilities:**
  - Filter by card ID (`--card 0`, `--card card1`, `--card HDMI`)
  - Filter by device name (`--device "Model 12"`, `--device Audio`)
  - Show all devices including duplicates (`--all`)
  - List available cards (`--list` / `-l`)
- **Enhanced card visibility:**
  - Available cards shown by default in normal output
  - Separate card-only listing mode (`--list` / `-l`)
  - Card information excluded from JSON output for clean data structure
- Smart duplicate filtering (removes virtual/duplicate devices by default)
- Card mapping system that reads from `/proc/asound/cards`
- Automatic detection of default input/output devices
- Support for common ALSA device names and configurations
- Comprehensive error handling and warnings
- Documentation with usage examples and troubleshooting guide

### Technical Details
- Built with Rust 2021 edition
- Uses CPAL 0.15 for cross-platform audio device enumeration
- Uses ALSA 0.9 for Linux-specific audio hardware access
- Implements structured data serialization with serde
- Command-line parsing with clap 4.0
- Robust error handling with anyhow

## [0.1.0] - 2024-XX-XX

### Added
- Initial release of audio-interrogator
- Basic audio device interrogation functionality
- Support for Linux audio systems (ALSA, PulseAudio)
- Cross-platform compatibility through CPAL
- JSON and human-readable output formats
- Verbose device inspection mode
- Command-line interface with help system
- Device filtering by card ID and device name
- Card listing functionality with shortcut flag (`-l`)
- Card information displayed by default for better user experience
- Smart filtering to reduce duplicate/virtual device noise

### Known Issues
- JACK server connection warnings (non-critical)
- Some ALSA plugin warnings for unsupported configurations
- Limited to common sample rates and buffer sizes for ALSA devices
- Some USB audio devices may not be detected by CPAL if not actively used
- Card filtering works best with card names rather than numbers for some devices

### Planned Features for Future Releases
- [ ] PulseAudio direct integration
- [ ] JACK support detection and information
- [ ] Real-time capability testing
- [ ] Audio latency measurements
- [ ] Device stress testing capabilities
- [ ] Configuration file export/import
- [ ] GUI interface
- [ ] Windows WASAPI integration improvements
- [ ] macOS CoreAudio enhancements
- [ ] Plugin architecture for custom device interrogation
- [ ] Performance benchmarking tools
- [ ] Device health monitoring
- [ ] Historical device capability tracking