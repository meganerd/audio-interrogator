# Audio Interrogator Usage Guide

This guide provides comprehensive usage examples for the Audio Interrogator tool, demonstrating how to effectively query and filter audio devices on Linux systems.

## Table of Contents

1. [Basic Usage](#basic-usage)
2. [Filtering by Card](#filtering-by-card)
3. [Filtering by Device Name](#filtering-by-device-name)
4. [Output Formats](#output-formats)
5. [System Integration Options](#system-integration-options)
6. [Advanced Examples](#advanced-examples)
7. [Troubleshooting Common Issues](#troubleshooting-common-issues)

## Basic Usage

### Default Output (Recommended)

The default mode shows available cards and a filtered list of devices without duplicates:

```bash
audio-interrogator
```

This will display:
- Available audio cards detected by the system
- Summary of total devices found
- List of active audio devices with basic information

### Show All Devices (Including Duplicates)

To see every device including virtual and duplicate entries:

```bash
audio-interrogator --all
```

### List Cards Only

To quickly see what audio cards are available:

```bash
audio-interrogator --list
# or
audio-interrogator -l
```

## Filtering by Card

### By Card Number

Filter devices for a specific card using its number:

```bash
# Show devices for card 0 (usually built-in audio)
audio-interrogator --card 0

# Show devices for card 1 (often first USB/external device)
audio-interrogator --card 1

# Alternative syntax
audio-interrogator --card card2
```

### By Card Name

Filter using the card's short name as shown in the card listing:

```bash
# For HDMI audio outputs
audio-interrogator --card HDMI

# For generic HD Audio
audio-interrogator --card Generic

# For USB audio devices
audio-interrogator --card Audio
```

### Examples for Common Hardware

```bash
# Built-in laptop audio (usually card 0)
audio-interrogator --card 0

# USB audio interface (varies, check with --list first)
audio-interrogator --card Audio

# HDMI output devices
audio-interrogator --card HDMI

# Professional audio interfaces (by name)
audio-interrogator --card "Scarlett"  # Focusrite Scarlett series
audio-interrogator --card "Model"     # TASCAM Model series
```

## Filtering by Device Name

### Basic Name Filtering

Filter devices by partial name matching (case-insensitive):

```bash
# Show devices with "Audio" in the name
audio-interrogator --device Audio

# Show devices with "USB" in the name
audio-interrogator --device USB

# Show devices with "Model" in the name (for TASCAM Model series)
audio-interrogator --device Model
```

### Professional Audio Equipment Examples

```bash
# TASCAM interfaces
audio-interrogator --device "Model 12"
audio-interrogator --device "Model 24"

# Focusrite interfaces
audio-interrogator --device "Scarlett"
audio-interrogator --device "Clarett"

# PreSonus interfaces
audio-interrogator --device "AudioBox"
audio-interrogator --device "Studio"

# RME interfaces
audio-interrogator --device "Fireface"
audio-interrogator --device "Babyface"

# MOTU interfaces
audio-interrogator --device "UltraLite"
audio-interrogator --device "828"
```

## Output Formats

### Verbose Mode

Get detailed information about each device:

```bash
# Verbose output for all devices
audio-interrogator --verbose

# Verbose output for specific card
audio-interrogator --card Audio --verbose

# Verbose output for specific device
audio-interrogator --device "Model 12" --verbose
```

### JSON Output

Perfect for programmatic use and integration with other tools:

```bash
# JSON for all devices
audio-interrogator --json

# JSON for filtered results
audio-interrogator --card HDMI --json
audio-interrogator --device Audio --json

# Pretty-printed JSON (pipe through jq if available)
audio-interrogator --json | jq .
```

### Combining Flags

Most flags can be combined for specific output:

```bash
# Verbose JSON output
audio-interrogator --verbose --json --card Audio

# Show all devices with verbose output
audio-interrogator --all --verbose

# Filter and get JSON
audio-interrogator --device "Scarlett" --json
```

## System Integration Options

### Non-Intrusive Mode

When working with active audio streams or in production environments, use the `--no-proc` flag to prevent the tool from accessing `/proc/asound`, which could potentially interfere with running audio applications:

```bash
# Safe mode - won't interfere with active audio streams
audio-interrogator --no-proc

# Combined with other flags
audio-interrogator --no-proc --card Audio --verbose
audio-interrogator --no-proc --json --device "Scarlett"
```

**When to use `--no-proc`:**
- During live audio recording or playback sessions
- In production environments where audio stability is critical
- When investigating audio issues without disrupting current streams
- In automated monitoring scripts that run frequently

**Trade-offs:**
- Provides less detailed information about device usage status
- Some advanced device state information may not be available
- Still provides full device capability information through ALSA/CPAL APIs

### Production Environment Examples

```bash
# Monitoring script that won't disrupt audio
#!/bin/bash
audio-interrogator --no-proc --json > /var/log/audio-devices.json

# Check device availability without interrupting streams
audio-interrogator --no-proc --device "Model 12" --verbose

# Safe system health check
audio-interrogator --no-proc --all --json | jq '.total_devices'
```

## Advanced Examples

### Studio Setup Analysis

Analyze a complex studio setup with multiple audio interfaces:

```bash
# Get overview of all cards
audio-interrogator --list

# Check main audio interface
audio-interrogator --device "Scarlett" --verbose

# Check secondary interface
audio-interrogator --device "Model" --verbose

# Get complete setup in JSON for documentation
audio-interrogator --all --json > studio_audio_config.json
```

### Troubleshooting Audio Issues

```bash
# Check if specific device is detected
audio-interrogator --device "Model 12"

# Verify HDMI audio availability
audio-interrogator --card HDMI --verbose

# Check all available sample rates for troubleshooting
audio-interrogator --card Audio --verbose | grep "Sample Rates"

# Export full configuration for support
audio-interrogator --all --verbose > audio_debug_info.txt
```

### Automation and Scripting

```bash
# Get device count for monitoring
audio-interrogator --json | jq '.total_input_devices'

# Check if specific professional interface is connected
if audio-interrogator --device "Scarlett" --json | jq -e '.devices | length > 0'; then
    echo "Scarlett interface detected"
fi

# List all input devices with more than 2 channels
audio-interrogator --json | jq '.devices[] | select(.input_channels > 2) | .name'

# Find devices supporting high sample rates (96kHz+)
audio-interrogator --json | jq '.devices[] | select(.supported_sample_rates[] >= 96000) | .name'
```

### Performance and Latency Testing Setup

```bash
# Check buffer size capabilities for low-latency work
audio-interrogator --device "Scarlett" --verbose | grep "Buffer Sizes"

# Verify high sample rate support
audio-interrogator --card Audio --verbose | grep "192000"

# Get all devices supporting professional sample rates
audio-interrogator --all --verbose | grep -E "(96000|192000)"
```

## Troubleshooting Common Issues

### No Devices Found

If filtering returns no devices:

```bash
# First, check what cards are available
audio-interrogator --list

# Then check all devices to see naming
audio-interrogator --all | head -20

# Try broader device name searches
audio-interrogator --device USB
audio-interrogator --device Audio
```

### USB Audio Device Not Showing

For USB audio interfaces that appear in `--list` but not in device enumeration:

```bash
# Check if device is in use by another application
lsof /dev/snd/*

# Verify device permissions
ls -l /dev/snd/

# Try running with elevated privileges for testing
sudo audio-interrogator --device "Model 12"
```

### Finding Professional Audio Interfaces

Professional interfaces may use different naming conventions:

```bash
# Try various name patterns
audio-interrogator --device "18i20"    # Scarlett 18i20
audio-interrogator --device "Solo"     # Scarlett Solo
audio-interrogator --device "2i2"      # Scarlett 2i2
audio-interrogator --device "Mixer"    # Various mixer interfaces
audio-interrogator --device "Interface" # Generic interface naming
```

### Identifying Virtual Devices

To distinguish between real and virtual devices:

```bash
# Show all devices (including virtual)
audio-interrogator --all

# Compare with filtered output (virtual devices removed)
audio-interrogator

# Look for specific virtual device types
audio-interrogator --all | grep -E "(dmix|dsnoop|pulse)"
```

### System Integration Examples

```bash
# Generate system report (safe for production use)
{
    echo "Audio System Report - $(date)"
    echo "================================="
    audio-interrogator --list
    echo ""
    audio-interrogator --no-proc --all --verbose
} > audio_system_report.txt

# Check for changes in audio setup (non-intrusive)
audio-interrogator --no-proc --json > current_audio.json
# Compare with previous snapshot
diff previous_audio.json current_audio.json

# Monitoring script for production environments
#!/bin/bash
# Safe audio device monitoring that won't interfere with streams
while true; do
    audio-interrogator --no-proc --json > /tmp/audio-status.json
    sleep 60
done
```

This usage guide covers the most common scenarios for using Audio Interrogator. For additional help, use `audio-interrogator --help` or refer to the README.md file.