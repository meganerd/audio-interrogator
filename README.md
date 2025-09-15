# Audio Interrogator

A Rust-based command-line tool for interrogating Linux audio devices and discovering their capabilities including input/output channels, supported sample rates, buffer sizes, and other audio hardware specifications.

## Features

- **Cross-platform audio device detection** using CPAL (Cross-Platform Audio Library)
- **Linux-specific ALSA integration** for detailed hardware information
- **Comprehensive device information** including:
  - Input and output channel counts
  - Supported sample rates
  - Buffer size capabilities
  - Default audio settings
  - Device types and drivers
- **Multiple output formats** (human-readable and JSON)
- **Verbose and summary modes** for different use cases
- **Enhanced device filtering** that searches both device names and card descriptions

## Installation

### Prerequisites

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

On Linux, you'll also need ALSA development libraries:

```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# Fedora/RHEL/CentOS
sudo dnf install alsa-lib-devel

# Arch Linux
sudo pacman -S alsa-lib
```

### Building from Source

```bash
git clone <repository-url>
cd audio-interrogator
cargo build --release
```

The compiled binary will be available at `target/release/audio-interrogator`.

## Usage

### Basic Usage

Run the tool to get a summary of all audio devices:

```bash
./audio-interrogator
```

### Command Line Options

- `-v, --verbose`: Display detailed information for each device
- `-j, --json`: Output results in JSON format for programmatic use
- `-a, --all`: Show all devices including duplicates and virtual devices
- `-c, --card <CARD_ID>`: Filter by specific card ID (e.g., card0, card1, or just 0, 1)
- `-d, --device <DEVICE_NAME>`: Filter by device name (partial match, case-insensitive)
- `-l, --list`: List available card IDs and exit (cards are shown by default in normal output)
- `-h, --help`: Show help information

### Examples

**Basic device listing (filtered, no duplicates):**
```bash
./audio-interrogator
```

**Show all devices including virtual/duplicate devices:**
```bash
./audio-interrogator --all
```

**Detailed device information:**
```bash
./audio-interrogator --verbose
```

**Filter by specific audio card:**
```bash
# By card number
./audio-interrogator --card 0
./audio-interrogator --card card1

# By card name
./audio-interrogator --card HDMI
./audio-interrogator --card Audio
```

**Filter by device name:**
```bash
# Show devices containing "Audio" in the name or description
./audio-interrogator --device Audio

# Show devices by manufacturer name (searches descriptions)
./audio-interrogator --device "Cubilux"
./audio-interrogator --device "TASCAM" 
./audio-interrogator --device "SPDIF"

# Show devices containing "Model 12" in the name or description
./audio-interrogator --device "Model 12"
```

**List available cards only:**
```bash
./audio-interrogator --list
# or
./audio-interrogator -l
```

**Combine filters with verbose output:**
```bash
./audio-interrogator --card 1 --verbose
./audio-interrogator --device Audio --json
```

**JSON output for integration with other tools:**
```bash
./audio-interrogator --json
```

## Sample Output

### Summary Mode
```
════════════════════════════════════════
        AVAILABLE AUDIO CARDS
════════════════════════════════════════
  0 [HDMI           ]: HDA-Intel - HDA ATI HDMI
  HDA ATI HDMI at 0xf6820000 irq 186
  1 [M12            ]: USB-Audio - Model 12
  TASCAM Model 12 at usb-0000:10:00.0-1, high speed
  2 [Generic        ]: HDA-Intel - HD-Audio Generic
  HD-Audio Generic at 0xf6680000 irq 188
  3 [Audio          ]: USB-Audio - USB Audio
  Generic USB Audio at usb-0000:12:00.0-6, high speed

════════════════════════════════════════
         SYSTEM AUDIO SUMMARY
════════════════════════════════════════
Total Devices Found: 8
Input Devices: 5
Output Devices: 6
Default Input: default
Default Output: default

════════════════════════════════════════
           DEVICE DETAILS
════════════════════════════════════════

1: default (CPAL) - In: 2, Out: 2, SR: 44100Hz
2: pulse (CPAL) - In: 2, Out: 2, SR: 44100Hz
3: hw:CARD=Audio,DEV=0 (CPAL) - In: 2, Out: 2, SR: 44100Hz
4: hw:CARD=Audio,DEV=1 (CPAL) - In: 2, Out: 2, SR: 44100Hz
5: sysdefault:CARD=Audio (CPAL) - In: 2, Out: 2, SR: 44100Hz
6: front:CARD=Audio,DEV=0 (CPAL) - In: 2, Out: 2, SR: 44100Hz
7: hw:CARD=ReceiverSolid,DEV=0 (CPAL) - In: 2, Out: 0, SR: 48000Hz
8: default (ALSA) - In: 32, Out: 32, SR: 44100Hz

Use --verbose flag for detailed device information
Use --card <id> to filter by card, --device <name> to filter by name
Use --all to show all devices including duplicates
```

### Filtered Output (by card)
```bash
./audio-interrogator --card Audio
```
```
════════════════════════════════════════
         SYSTEM AUDIO SUMMARY
════════════════════════════════════════
Total Devices Found: 6
Input Devices: 5
Output Devices: 6
Default Input: default
Default Output: default

════════════════════════════════════════
           DEVICE DETAILS
════════════════════════════════════════

1: hw:CARD=Audio,DEV=0 (CPAL) - In: 2, Out: 2, SR: 44100Hz
2: hw:CARD=Audio,DEV=1 (CPAL) - In: 2, Out: 2, SR: 44100Hz
3: hw:CARD=Audio,DEV=2 (CPAL) - In: 2, Out: 2, SR: 44100Hz
4: hw:CARD=Audio,DEV=3 (CPAL) - In: 0, Out: 2, SR: 44100Hz
5: sysdefault:CARD=Audio (CPAL) - In: 2, Out: 2, SR: 44100Hz
6: front:CARD=Audio,DEV=0 (CPAL) - In: 2, Out: 2, SR: 44100Hz
```

### Verbose Mode
```
Device #1
┌─ Device: hw:CARD=Audio,DEV=0
├─ Type: Input/Output
├─ Driver: CPAL
├─ Input Channels: 2
├─ Output Channels: 2
├─ Default Sample Rate: 44100 Hz
├─ Default Buffer Size: 1024 samples
├─ Supported Sample Rates: [44100, 48000, 96000, 192000] Hz
└─ Supported Buffer Sizes: [64, 128, 256, 512, 1024, 2048, 4096] samples
```

### Card Listing Mode
```bash
./audio-interrogator --list-cards
```
```
Available Audio Cards:
═════════════════════
  0 [HDMI           ]: HDA-Intel - HDA ATI HDMI
  HDA ATI HDMI at 0xf6820000 irq 186
  1 [M12            ]: USB-Audio - Model 12
  TASCAM Model 12 at usb-0000:10:00.0-1, high speed
  2 [Generic        ]: HDA-Intel - HD-Audio Generic
  HD-Audio Generic at 0xf6680000 irq 188
  3 [Audio          ]: USB-Audio - USB Audio
  Generic USB Audio at usb-0000:12:00.0-6, high speed

Card Directories in /proc/asound/:
═══════════════════════════════════
  card0
  card1
  card2
  card3

Usage examples:
  audio-interrogator --card 0        # Show devices for card0
  audio-interrogator --card card1    # Show devices for card1
  audio-interrogator --device Audio  # Show devices matching 'Audio'
```

## Architecture

The tool uses two main audio libraries:

1. **CPAL (Cross-Platform Audio Library)**: Provides cross-platform audio device enumeration and works on Linux, Windows, and macOS.

2. **ALSA (Advanced Linux Sound Architecture)**: Linux-specific library that provides more detailed hardware information and direct access to ALSA devices.

### Key Components

- `AudioDeviceInfo`: Structure containing all relevant device information
- `SystemAudioInfo`: System-wide audio information including device lists and defaults
- `get_cpal_devices()`: Enumerates devices using CPAL
- `get_alsa_devices()`: Enumerates devices using ALSA (Linux only)
- `get_system_audio_info()`: Combines information from both sources

## Dependencies

- **alsa**: Direct interface to ALSA (Linux only)
- **cpal**: Cross-platform audio device enumeration
- **serde**: Serialization framework for JSON output
- **serde_json**: JSON serialization support
- **clap**: Command-line argument parsing
- **anyhow**: Error handling

## Platform Support

- **Linux**: Full support with both CPAL and ALSA integration
- **Windows/macOS**: CPAL-only support (can be extended)

## Use Cases

- **Audio application development**: Discover available audio interfaces before initialization
- **System administration**: Audit audio hardware capabilities
- **Troubleshooting**: Identify audio device configuration issues
- **Hardware testing**: Verify audio device specifications
- **CI/CD integration**: Automated audio hardware testing
- **Multi-card systems**: Filter and examine specific audio cards
- **Studio setups**: Identify professional audio interfaces by name or card ID
- **Automation**: JSON output for integration with audio management scripts

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Troubleshooting

### Common Issues

**"No devices found"**
- Ensure your user is in the `audio` group: `sudo usermod -a -G audio $USER`
- Check if audio devices are properly detected by the system: `aplay -l` and `arecord -l`

**"Permission denied" errors**
- Make sure you have proper permissions to access audio devices
- Try running with elevated privileges for testing: `sudo ./audio-interrogator`

**ALSA-related compilation errors**
- Install ALSA development libraries (see Installation section)
- Ensure pkg-config is installed: `sudo apt-get install pkg-config`

**"No devices found" when filtering**
- Use `--list` to see available card IDs and names
- Try using card names instead of numbers: `--card HDMI` vs `--card 0`
- Use `--all` flag to see all devices including virtual ones
- Device filtering searches both device names and card descriptions
- Some USB devices may be detected but not have active audio interfaces
- Try filtering by manufacturer: `--device Cubilux`, `--device TASCAM`

### Debug Mode

For additional debugging information, set the RUST_LOG environment variable:

```bash
RUST_LOG=debug ./audio-interrogator --verbose
```

## Roadmap

- [ ] PulseAudio integration
- [ ] JACK support detection
- [ ] Real-time capability testing
- [ ] Latency measurements
- [ ] Device stress testing
- [ ] Configuration file export
- [ ] GUI interface