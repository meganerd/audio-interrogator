# Contributing to Audio Interrogator

Thank you for your interest in contributing to Audio Interrogator! This document provides guidelines and information for contributors.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [Making Contributions](#making-contributions)
5. [Coding Guidelines](#coding-guidelines)
6. [Testing](#testing)
7. [Documentation](#documentation)
8. [Submitting Changes](#submitting-changes)

## Code of Conduct

This project follows a simple code of conduct:
- Be respectful and constructive in all interactions
- Focus on what is best for the community
- Show empathy towards other community members
- Accept constructive criticism gracefully

## Getting Started

### Types of Contributions

We welcome several types of contributions:

- **Bug Reports**: Help us identify and fix issues
- **Feature Requests**: Suggest new functionality
- **Code Contributions**: Implement new features or fix bugs
- **Documentation**: Improve or add documentation
- **Testing**: Test on different hardware configurations
- **Hardware Support**: Add support for new audio devices

### Good First Issues

Look for issues tagged with `good first issue` or `help wanted` if you're new to the project.

## Development Setup

### Prerequisites

1. **Rust**: Install Rust using [rustup](https://rustup.rs/)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Linux Development Libraries** (Linux only):
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libasound2-dev pkg-config
   
   # Fedora/RHEL
   sudo dnf install alsa-lib-devel pkgconfig
   
   # Arch Linux
   sudo pacman -S alsa-lib pkgconf
   ```

3. **Audio Hardware**: Various audio devices for testing (optional but helpful)

### Setting Up the Development Environment

1. **Fork and Clone**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/audio-interrogator.git
   cd audio-interrogator
   ```

2. **Build and Test**:
   ```bash
   # Build the project
   cargo build
   
   # Run tests
   cargo test
   
   # Run the tool
   cargo run -- --help
   ```

3. **Development Build**:
   ```bash
   # For faster iteration during development
   cargo build
   
   # For production testing
   cargo build --release
   ```

## Making Contributions

### Before You Start

1. **Check existing issues** to see if your bug/feature is already reported
2. **Open an issue** to discuss large changes before implementing
3. **Search existing PRs** to avoid duplicate work

### Branching Strategy

- `main` branch contains stable releases
- Create feature branches from `main`
- Use descriptive branch names: `feature/device-latency-measurement`, `fix/alsa-crash-on-disconnect`

## Coding Guidelines

### Rust Style

- Follow standard Rust formatting: `cargo fmt`
- Use `cargo clippy` for linting
- Write idiomatic Rust code
- Document public APIs with doc comments

### Code Organization

```
src/
├── main.rs              # CLI interface and main application logic
├── devices/             # Device detection and management
│   ├── mod.rs          # Module exports
│   ├── types.rs        # Data structures
│   ├── cpal_devices.rs # CPAL backend
│   └── alsa_devices.rs # ALSA backend
└── utils/              # Utility functions (future)
```

### Error Handling

- Use `anyhow::Result` for error propagation
- Provide meaningful error messages
- Handle edge cases gracefully
- Don't panic in production code paths

### Audio Device Support

When adding support for new audio devices:

1. **Test on Real Hardware**: Ensure changes work with actual devices
2. **Handle Edge Cases**: Devices may behave unexpectedly
3. **Backward Compatibility**: Don't break existing functionality
4. **Documentation**: Update device compatibility lists

## Testing

### Manual Testing

1. **Test with Multiple Devices**:
   ```bash
   # Test basic functionality
   cargo run
   
   # Test filtering
   cargo run -- --card 0
   cargo run -- --device USB
   
   # Test output formats
   cargo run -- --json
   cargo run -- --verbose
   ```

2. **Test Edge Cases**:
   - Disconnecting devices while running
   - Systems with no audio devices
   - Systems with many audio devices
   - Devices in use by other applications

### Hardware Testing

If you have access to professional audio equipment, test with:
- USB audio interfaces (Focusrite, PreSonus, MOTU, etc.)
- Multiple card setups
- HDMI audio devices
- Bluetooth audio devices
- Various sample rates and bit depths

### Cross-Platform Testing

While primarily focused on Linux, test on:
- Different Linux distributions
- Various kernel versions
- Different desktop environments (GNOME, KDE, etc.)
- Both PulseAudio and JACK setups

## Documentation

### Code Documentation

- Document all public functions and structs
- Include examples in doc comments
- Explain complex algorithms or audio concepts

### User Documentation

- Update README.md for new features
- Add examples to USAGE.md
- Update OUTPUT_GUIDE.md for new output formats
- Keep CHANGELOG.md updated

### Documentation Format

```rust
/// Interrogates audio devices using the CPAL backend
/// 
/// This function enumerates all available audio devices through CPAL
/// and returns detailed information about their capabilities.
/// 
/// # Examples
/// 
/// ```rust
/// let devices = get_cpal_devices()?;
/// for device in devices {
///     println!("Device: {}", device.name);
/// }
/// ```
/// 
/// # Errors
/// 
/// Returns an error if the audio subsystem is not available or
/// if there are permission issues accessing audio devices.
pub fn get_cpal_devices() -> Result<Vec<AudioDeviceInfo>> {
    // Implementation...
}
```

## Submitting Changes

### Pull Request Process

1. **Create a Branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Your Changes**:
   - Write code following the guidelines above
   - Add tests if applicable
   - Update documentation

3. **Test Your Changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Commit Your Changes**:
   ```bash
   git add .
   git commit -m "Add support for XYZ audio interface
   
   - Implement detection for XYZ brand interfaces
   - Add channel count parsing for multi-channel devices  
   - Update documentation with XYZ examples"
   ```

5. **Push and Create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```

### Pull Request Guidelines

**Title**: Use descriptive titles
- ✅ "Add support for MOTU audio interfaces"
- ✅ "Fix crash when ALSA device is disconnected"
- ❌ "Update code"
- ❌ "Fix bug"

**Description**: Include:
- What changes you made
- Why you made them
- How to test the changes
- Any breaking changes
- Related issue numbers

**Example PR Description**:
```markdown
## Summary
Add support for detecting MOTU USB audio interfaces and their channel configurations.

## Changes
- Enhanced ALSA device parsing to handle MOTU-specific naming
- Added channel count detection for MOTU interfaces
- Updated device filtering to work with MOTU card names

## Testing
Tested with MOTU UltraLite-mk5 and 828es interfaces.

## Related Issues
Fixes #42
Related to #38

## Breaking Changes
None
```

### Review Process

1. **Automated Checks**: CI will run tests and linting
2. **Code Review**: Maintainers will review your changes
3. **Testing**: Changes may be tested on different hardware
4. **Merge**: Once approved, changes will be merged

## Development Tips

### Debugging Audio Issues

1. **Use Verbose Logging**:
   ```bash
   RUST_LOG=debug cargo run -- --verbose
   ```

2. **Check ALSA State**:
   ```bash
   cat /proc/asound/cards
   aplay -l
   arecord -l
   ```

3. **Monitor Device Changes**:
   ```bash
   # Watch for device changes
   udevadm monitor --subsystem-match=sound
   ```

### Common Pitfalls

- **Permissions**: Audio device access requires proper permissions
- **Device States**: Devices may be in use by other applications
- **USB Timing**: USB devices may take time to enumerate
- **Driver Differences**: Different devices may expose capabilities differently

### Getting Help

- **GitHub Issues**: Ask questions in issues
- **Code Comments**: Leave detailed comments in complex code
- **Documentation**: Reference existing documentation and examples

## Recognition

Contributors will be recognized in:
- CHANGELOG.md for significant contributions
- README.md contributors section
- Git commit history

Thank you for contributing to Audio Interrogator! 🎵