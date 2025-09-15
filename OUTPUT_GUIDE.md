# Audio Interrogator Output Guide

This guide explains what each part of the Audio Interrogator output means, helping you understand your system's audio configuration.

## Understanding Device Output Format

### Basic Format
```
[Number]: [Device Name] ([Driver]) - In: [Input Channels], Out: [Output Channels], SR: [Sample Rate]Hz
```

### Example Breakdown
```
1: hw:CARD=ReceiverSolid,DEV=0 (CPAL) - In: 2, Out: 0, SR: 48000Hz
```

Let's break this down piece by piece:

#### 1. Device Number
- **`1:`** - Sequential number for this device in the list
- Used for reference and counting

#### 2. Device Name
- **`hw:CARD=ReceiverSolid,DEV=0`** - ALSA device identifier
- **Format**: `hw:CARD=[CardName],DEV=[DeviceNumber]`
- **`hw:`** - Hardware device (direct access)
- **`CARD=ReceiverSolid`** - Card name (from kernel driver)
- **`DEV=0`** - Device number on that card (0 is usually the main device)

#### 3. Driver/Backend
- **`(CPAL)`** - The audio library used to detect this device
- **CPAL** = Cross-Platform Audio Library
- **ALSA** = Advanced Linux Sound Architecture (Linux-specific)

#### 4. Input Channels
- **`In: 2`** - Number of input (recording) channels available
- **2** typically means stereo input
- **0** means no input capability (output-only device)

#### 5. Output Channels  
- **`Out: 0`** - Number of output (playback) channels available
- **0** means no output capability (input-only device)
- **2** typically means stereo output
- **8** might mean 7.1 surround sound

#### 6. Sample Rate
- **`SR: 48000Hz`** - Default sample rate in Hertz
- **48000Hz** (48kHz) - Professional audio standard
- **44100Hz** (44.1kHz) - CD audio standard

## Device Name Types Explained

### Hardware Devices (`hw:`)
```
hw:CARD=ReceiverSolid,DEV=0
```
- **Direct hardware access**
- **Exclusive access** (only one application can use it at a time)
- **Best for professional audio** (lowest latency)
- **Risk**: Can conflict with other applications

### System Default (`sysdefault:`)
```
sysdefault:CARD=ReceiverSolid
```
- **System-managed device**
- **Shared access** (multiple applications can use it)
- **Automatic sample rate conversion** if needed
- **Safe choice** for most applications

### Plugin Devices (`plughw:`)
```
plughw:CARD=Audio,DEV=0
```
- **Hardware with automatic format conversion**
- **ALSA handles** sample rate/format conversion automatically
- **More compatible** than raw `hw:` devices
- **Slightly higher latency** than `hw:` due to conversion

### Mixer Devices (`dmix:`, `dsnoop:`)
```
dmix:CARD=Audio,DEV=0     # Output mixing
dsnoop:CARD=Audio,DEV=0   # Input sharing
```
- **`dmix:`** - Digital mixer for **output** (playback)
- **`dsnoop:`** - Digital snoop for **input** (recording)
- **Allow multiple applications** to share the same hardware
- **Automatic mixing** of audio streams

### Surround Sound Devices
```
front:CARD=Audio,DEV=0        # Front speakers
surround51:CARD=Audio,DEV=0   # 5.1 surround
surround71:CARD=Audio,DEV=0   # 7.1 surround
```
- **`front:`** - Front stereo channels only
- **`surround51:`** - 5.1 surround sound (6 channels)
- **`surround71:`** - 7.1 surround sound (8 channels)

### Digital Output (`iec958:`)
```
iec958:CARD=Audio,DEV=0
```
- **Digital S/PDIF output** (optical/coaxial)
- **IEC 958** is the S/PDIF standard
- **Passes digital audio** to external DACs/receivers

## Interpreting Your Example

```
1: hw:CARD=ReceiverSolid,DEV=0 (CPAL) - In: 2, Out: 0, SR: 48000Hz
2: sysdefault:CARD=ReceiverSolid (CPAL) - In: 2, Out: 0, SR: 44100Hz  
3: front:CARD=ReceiverSolid,DEV=0 (CPAL) - In: 2, Out: 0, SR: 48000Hz
```

### What This Tells Us:

1. **Device Type**: Cubilux SPDIF ReceiverSolid (USB audio device)
2. **Capability**: **Input-only device** (In: 2, Out: 0)
3. **Input**: Stereo input (2 channels) - can record in stereo
4. **Output**: No output capability - this is a recording/capture device
5. **Multiple Access Methods**:
   - `hw:` = Direct hardware access (48kHz default)
   - `sysdefault:` = System-managed (44.1kHz default) 
   - `front:` = Front channel access (48kHz default)

### Practical Usage:
- **For recording**: Use any of these three interfaces
- **Recommended**: `sysdefault:CARD=ReceiverSolid` for most applications
- **Professional use**: `hw:CARD=ReceiverSolid,DEV=0` for lowest latency
- **Sample rate**: Device prefers 48kHz but can handle 44.1kHz

## Common Device Patterns

### Built-in Audio Card
```
1: hw:CARD=Audio,DEV=0 (CPAL) - In: 2, Out: 2, SR: 44100Hz
2: front:CARD=Audio,DEV=0 (CPAL) - In: 2, Out: 2, SR: 44100Hz
3: surround51:CARD=Audio,DEV=0 (CPAL) - In: 0, Out: 6, SR: 44100Hz
```
- **Full duplex** (can record and playback simultaneously)
- **Multiple output configurations** (stereo, 5.1 surround)

### HDMI Output
```
1: hw:CARD=HDMI,DEV=3 (CPAL) - In: 0, Out: 2, SR: 44100Hz
2: hdmi:CARD=HDMI,DEV=0 (CPAL) - In: 0, Out: 2, SR: 44100Hz
```
- **Output-only** (In: 0, Out: 2)
- **Multiple HDMI ports** (DEV=0, DEV=3, etc.)

### Professional Audio Interface
```
1: hw:CARD=Scarlett,DEV=0 (CPAL) - In: 8, Out: 8, SR: 48000Hz
2: sysdefault:CARD=Scarlett (CPAL) - In: 8, Out: 8, SR: 48000Hz
```
- **Multi-channel** (8 inputs, 8 outputs)
- **Professional sample rate** (48kHz default)

### USB Headset
```
1: hw:CARD=Headset,DEV=0 (CPAL) - In: 1, Out: 2, SR: 44100Hz
```
- **Mono microphone** (In: 1) 
- **Stereo headphones** (Out: 2)

## Troubleshooting with Output Information

### No Output Channels (Out: 0)
- Device is **input/recording only**
- Cannot be used for playback/speakers
- Examples: USB microphones, audio capture devices

### No Input Channels (In: 0)  
- Device is **output/playback only**
- Cannot be used for recording
- Examples: HDMI output, some USB speakers

### High Channel Counts
- **In: 32, Out: 32** - Likely a virtual/system device
- **In: 8, Out: 8** - Professional audio interface
- **In: 6, Out: 6** - 5.1 surround sound card

### Sample Rate Differences
- **48000Hz** - Professional/broadcast standard
- **44100Hz** - Consumer/CD standard  
- **96000Hz+** - High-resolution audio
- Different rates between devices = automatic conversion may occur

## Best Practices

### For Recording Applications:
1. Look for devices with **In: > 0**
2. Prefer **professional sample rates** (48kHz, 96kHz)
3. Use **`hw:`** devices for lowest latency
4. Use **`sysdefault:`** for compatibility

### For Playback Applications:
1. Look for devices with **Out: > 0** 
2. Match **channel count** to your speakers (2=stereo, 6=5.1, 8=7.1)
3. Use **`sysdefault:`** for most applications
4. Use **`hw:`** for exclusive access

### For Professional Audio:
1. Look for **high channel counts** (In: 8+, Out: 8+)
2. Prefer **48kHz or higher** sample rates
3. Use **`hw:`** devices for direct hardware access
4. Avoid **`dmix:`** and **`dsnoop:`** for critical applications

This output format gives you complete information about how your audio hardware is configured and what capabilities each device offers.