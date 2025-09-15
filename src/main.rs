use std::collections::{HashSet, HashMap};
use anyhow::Result;
use clap::{Arg, Command};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AudioDeviceInfo {
    name: String,
    device_type: String,
    input_channels: u32,
    output_channels: u32,
    supported_sample_rates: Vec<u32>,
    supported_buffer_sizes: Vec<u32>,
    default_sample_rate: u32,
    default_buffer_size: u32,
    driver: String,
}

impl AudioDeviceInfo {
    fn new(name: String, driver: String) -> Self {
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

    fn update_device_type(&mut self) {
        self.device_type = match (self.input_channels > 0, self.output_channels > 0) {
            (true, true) => "Input/Output".to_string(),
            (true, false) => "Input".to_string(),
            (false, true) => "Output".to_string(),
            (false, false) => "Unknown".to_string(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemAudioInfo {
    devices: Vec<AudioDeviceInfo>,
    default_input: Option<String>,
    default_output: Option<String>,
    total_input_devices: usize,
    total_output_devices: usize,
}

fn get_cpal_devices() -> Result<Vec<AudioDeviceInfo>> {
    use cpal::traits::{DeviceTrait, HostTrait};

    let mut devices = Vec::new();

    // Get the default host
    let host = cpal::default_host();

    // Iterate through all available devices
    for device in host.devices()? {
        let device_name = device.name().unwrap_or_else(|_| "Unknown Device".to_string());

        // Get supported input configs
        let input_channels = match device.default_input_config() {
            Ok(config) => config.channels() as u32,
            Err(_) => 0,
        };

        // Get supported output configs
        let output_channels = match device.default_output_config() {
            Ok(config) => config.channels() as u32,
            Err(_) => 0,
        };

        // Get supported sample rates
        let mut supported_sample_rates = Vec::new();
        let mut default_sample_rate = 44100;
        let mut default_buffer_size = 1024;

        // Try to get input config ranges
        if let Ok(input_configs) = device.supported_input_configs() {
            for config in input_configs {
                supported_sample_rates.push(config.min_sample_rate().0);
                supported_sample_rates.push(config.max_sample_rate().0);
                if let Ok(default_config) = device.default_input_config() {
                    default_sample_rate = default_config.sample_rate().0;
                    default_buffer_size = 1024; // CPAL doesn't directly expose buffer size
                }
            }
        }

        // Try to get output config ranges if no input configs
        if supported_sample_rates.is_empty() {
            if let Ok(output_configs) = device.supported_output_configs() {
                for config in output_configs {
                    supported_sample_rates.push(config.min_sample_rate().0);
                    supported_sample_rates.push(config.max_sample_rate().0);
                    if let Ok(default_config) = device.default_output_config() {
                        default_sample_rate = default_config.sample_rate().0;
                    }
                }
            }
        }

        // Remove duplicates and sort
        supported_sample_rates.sort_unstable();
        supported_sample_rates.dedup();

        // Common buffer sizes (since CPAL doesn't expose this directly)
        let supported_buffer_sizes = vec![64, 128, 256, 512, 1024, 2048, 4096];

        let device_type = match (input_channels > 0, output_channels > 0) {
            (true, true) => "Input/Output".to_string(),
            (true, false) => "Input".to_string(),
            (false, true) => "Output".to_string(),
            (false, false) => "Unknown".to_string(),
        };

        devices.push(AudioDeviceInfo {
            name: device_name,
            device_type,
            input_channels,
            output_channels,
            supported_sample_rates,
            supported_buffer_sizes,
            default_sample_rate,
            default_buffer_size,
            driver: "CPAL".to_string(),
        });
    }

    Ok(devices)
}

#[cfg(target_os = "linux")]
fn get_alsa_devices() -> Result<Vec<AudioDeviceInfo>> {
    use alsa::{PCM, Direction};
    use alsa::pcm::{HwParams, Access, Format};

    let mut devices = Vec::new();

    // First get devices from /proc/asound to include in-use devices
    if let Ok(proc_devices) = get_proc_alsa_devices() {
        devices.extend(proc_devices);
    }

    // Common ALSA device names to check for additional devices
    let device_names = vec![
        "default",
        "hw:0,0", "hw:0,1", "hw:0,2", "hw:0,3",
        "hw:1,0", "hw:1,1", "hw:1,2", "hw:1,3",
        "hw:2,0", "hw:2,1", "hw:2,2", "hw:2,3",
        "plughw:0,0", "plughw:0,1", "plughw:1,0", "plughw:1,1",
    ];

    for device_name in device_names {
        // Try to open for playback (output)
        let mut output_channels = 0;
        let mut input_channels = 0;
        let mut supported_rates = Vec::new();

        if let Ok(pcm) = PCM::new(device_name, Direction::Playback, false) {
            if let Ok(hwp) = HwParams::any(&pcm) {
                if hwp.set_access(Access::RWInterleaved).is_ok() &&
                   hwp.set_format(Format::s16()).is_ok() {

                    // Get channel count range
                    if let Ok(max_ch) = hwp.get_channels_max() {
                        output_channels = max_ch;
                    }

                    // Get sample rate range
                    if let (Ok(min_rate), Ok(max_rate)) = (hwp.get_rate_min(), hwp.get_rate_max()) {
                        // Add common sample rates within the supported range
                        let common_rates = vec![8000, 11025, 22050, 44100, 48000, 88200, 96000, 176400, 192000];
                        for &rate in &common_rates {
                            if rate >= min_rate && rate <= max_rate {
                                supported_rates.push(rate);
                            }
                        }
                        if supported_rates.is_empty() {
                            supported_rates.push(min_rate);
                            supported_rates.push(max_rate);
                        }
                    }
                }
            }
        }

        // Try to open for capture (input)
        if let Ok(pcm) = PCM::new(device_name, Direction::Capture, false) {
            if let Ok(hwp) = HwParams::any(&pcm) {
                if hwp.set_access(Access::RWInterleaved).is_ok() &&
                   hwp.set_format(Format::s16()).is_ok() {

                    // Get channel count range
                    if let Ok(max_ch) = hwp.get_channels_max() {
                        input_channels = max_ch;
                    }

                    // Get sample rate range if not already populated
                    if supported_rates.is_empty() {
                        if let (Ok(min_rate), Ok(max_rate)) = (hwp.get_rate_min(), hwp.get_rate_max()) {
                            let common_rates = vec![8000, 11025, 22050, 44100, 48000, 88200, 96000, 176400, 192000];
                            for &rate in &common_rates {
                                if rate >= min_rate && rate <= max_rate {
                                    supported_rates.push(rate);
                                }
                            }
                            if supported_rates.is_empty() {
                                supported_rates.push(min_rate);
                                supported_rates.push(max_rate);
                            }
                        }
                    }
                }
            }
        }

        // Only add device if it has input or output capabilities
        if input_channels > 0 || output_channels > 0 {
            let device_type = match (input_channels > 0, output_channels > 0) {
                (true, true) => "Input/Output".to_string(),
                (true, false) => "Input".to_string(),
                (false, true) => "Output".to_string(),
                (false, false) => continue,
            };

            // Common buffer sizes for ALSA
            let supported_buffer_sizes = vec![64, 128, 256, 512, 1024, 2048, 4096, 8192];

            devices.push(AudioDeviceInfo {
                name: device_name.to_string(),
                device_type,
                input_channels,
                output_channels,
                supported_sample_rates: supported_rates.clone(),
                supported_buffer_sizes,
                default_sample_rate: 44100,
                default_buffer_size: 1024,
                driver: "ALSA".to_string(),
            });
        }
    }

    Ok(devices)
}

#[cfg(not(target_os = "linux"))]
fn get_alsa_devices() -> Result<Vec<AudioDeviceInfo>> {
    Ok(Vec::new()) // ALSA is Linux-specific
}

fn get_system_audio_info() -> Result<SystemAudioInfo> {
    let mut all_devices = Vec::new();

    // Get CPAL devices (cross-platform)
    match get_cpal_devices() {
        Ok(mut cpal_devices) => all_devices.append(&mut cpal_devices),
        Err(e) => eprintln!("Warning: Failed to get CPAL devices: {}", e),
    }

    // Get ALSA devices (Linux-specific)
    #[cfg(target_os = "linux")]
    match get_alsa_devices() {
        Ok(mut alsa_devices) => all_devices.append(&mut alsa_devices),
        Err(e) => eprintln!("Warning: Failed to get ALSA devices: {}", e),
    }

    let input_count = all_devices.iter().filter(|d| d.input_channels > 0).count();
    let output_count = all_devices.iter().filter(|d| d.output_channels > 0).count();

    // Try to determine default devices
    let default_input = all_devices.iter()
        .find(|d| d.input_channels > 0 && (d.name.contains("default") || d.name.contains("hw:0")))
        .map(|d| d.name.clone());

    let default_output = all_devices.iter()
        .find(|d| d.output_channels > 0 && (d.name.contains("default") || d.name.contains("hw:0")))
        .map(|d| d.name.clone());

    Ok(SystemAudioInfo {
        devices: all_devices,
        default_input,
        default_output,
        total_input_devices: input_count,
        total_output_devices: output_count,
    })
}

fn print_device_info(device: &AudioDeviceInfo) {
    println!("┌─ Device: {}", device.name);
    println!("├─ Type: {}", device.device_type);
    println!("├─ Driver: {}", device.driver);
    println!("├─ Input Channels: {}", device.input_channels);
    println!("├─ Output Channels: {}", device.output_channels);
    println!("├─ Default Sample Rate: {} Hz", device.default_sample_rate);
    println!("├─ Default Buffer Size: {} samples", device.default_buffer_size);

    if !device.supported_sample_rates.is_empty() {
        println!("├─ Supported Sample Rates: {:?} Hz", device.supported_sample_rates);
    }

    println!("└─ Supported Buffer Sizes: {:?} samples", device.supported_buffer_sizes);
    println!();
}

fn main() -> Result<()> {
    let matches = Command::new("Audio Interrogator")
        .version("0.1.0")
        .author("Audio Engineer")
        .about("Interrogates Linux audio devices for their capabilities")
        .arg(Arg::new("json")
            .short('j')
            .long("json")
            .action(clap::ArgAction::SetTrue)
            .help("Output results in JSON format"))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(clap::ArgAction::SetTrue)
            .help("Enable verbose output"))
        .arg(Arg::new("all")
            .short('a')
            .long("all")
            .action(clap::ArgAction::SetTrue)
            .help("Show all devices including duplicates and virtual devices"))
        .arg(Arg::new("card")
            .short('c')
            .long("card")
            .value_name("CARD_ID")
            .help("Filter by specific card ID (e.g., card0, card1, or just 0, 1)"))
        .arg(Arg::new("device")
            .short('d')
            .long("device")
            .value_name("DEVICE_NAME")
            .help("Filter by device name (partial match, case-insensitive)"))
        .arg(Arg::new("list-cards")
            .short('l')
            .long("list")
            .action(clap::ArgAction::SetTrue)
            .help("List available card IDs and exit (cards are shown by default)"))
        .get_matches();

    let json_output = matches.get_flag("json");
    let verbose = matches.get_flag("verbose");
    let show_all = matches.get_flag("all");
    let card_filter = matches.get_one::<String>("card");
    let device_filter = matches.get_one::<String>("device");
    let list_cards = matches.get_flag("list-cards");

    // Handle list-cards mode
    if list_cards {
        list_available_cards()?;
        return Ok(());
    }

    if verbose && !json_output {
        println!("🎵 Audio Interrogator - Scanning system audio devices...\n");
    }

    let mut system_info = get_system_audio_info()?;

    // Apply filters
    if !show_all {
        system_info.devices = filter_devices(system_info.devices, card_filter, device_filter, false);
    } else {
        system_info.devices = filter_devices(system_info.devices, card_filter, device_filter, true);
    }

    // Recalculate counts after filtering
    system_info.total_input_devices = system_info.devices.iter().filter(|d| d.input_channels > 0).count();
    system_info.total_output_devices = system_info.devices.iter().filter(|d| d.output_channels > 0).count();

    if json_output {
        println!("{}", serde_json::to_string_pretty(&system_info)?);
    } else {
        // Show card listing as part of default output
        println!("════════════════════════════════════════");
        println!("        AVAILABLE AUDIO CARDS");
        println!("════════════════════════════════════════");
        show_card_summary()?;

        println!("\n════════════════════════════════════════");
        println!("         SYSTEM AUDIO SUMMARY");
        println!("════════════════════════════════════════");
        println!("Total Devices Found: {}", system_info.devices.len());
        println!("Input Devices: {}", system_info.total_input_devices);
        println!("Output Devices: {}", system_info.total_output_devices);

        if let Some(ref default_input) = system_info.default_input {
            println!("Default Input: {}", default_input);
        }

        if let Some(ref default_output) = system_info.default_output {
            println!("Default Output: {}", default_output);
        }

        println!("\n════════════════════════════════════════");
        println!("           DEVICE DETAILS");
        println!("════════════════════════════════════════\n");

        for (i, device) in system_info.devices.iter().enumerate() {
            if verbose {
                println!("Device #{}", i + 1);
                print_device_info(device);
            } else {
                println!("{}: {} ({}) - In: {}, Out: {}, SR: {}Hz",
                    i + 1,
                    device.name,
                    device.driver,
                    device.input_channels,
                    device.output_channels,
                    device.default_sample_rate
                );
            }
        }

        if !verbose {
            println!("\nUse --verbose flag for detailed device information");
            println!("Use --card <id> to filter by card, --device <name> to filter by name");
            println!("Use --all to show all devices including duplicates");
        }
    }

    Ok(())
}

fn filter_devices(
    devices: Vec<AudioDeviceInfo>,
    card_filter: Option<&String>,
    device_filter: Option<&String>,
    show_all: bool
) -> Vec<AudioDeviceInfo> {
    let mut filtered = devices;

    // Apply card filter
    if let Some(card_id) = card_filter {
        let card_num = if card_id.starts_with("card") {
            card_id.strip_prefix("card").unwrap_or(card_id)
        } else {
            card_id
        };

        // Get card mapping from system
        let card_mapping = get_card_mapping().unwrap_or_default();
        let target_card_name = card_mapping.get(card_num).cloned();

        filtered = filtered.into_iter().filter(|device| {
            // Match by card number in various formats
            device.name.contains(&format!("hw:{}", card_num)) ||
            device.name.contains(&format!("card{}", card_num)) ||
            // Match by card name if we found it
            (target_card_name.as_ref().map_or(false, |name| device.name.contains(&format!("CARD={}", name)))) ||
            // Direct match for card name
            device.name.contains(&format!("CARD={}", card_id))
        }).collect();
    }

    // Apply device name filter
    if let Some(name_filter) = device_filter {
        let name_lower = name_filter.to_lowercase();
        let card_descriptions = get_card_descriptions().unwrap_or_default();

        filtered = filtered.into_iter().filter(|device| {
            // First check device name
            if device.name.to_lowercase().contains(&name_lower) {
                return true;
            }

            // Then check if any card description matches and this device belongs to that card
            for (card_name, description) in &card_descriptions {
                if description.to_lowercase().contains(&name_lower) &&
                   device.name.contains(&format!("CARD={}", card_name)) {
                    return true;
                }
            }

            false
        }).collect();
    }

    // If not showing all, remove common duplicates
    if !show_all {
        let mut seen_names = HashSet::new();
        filtered = filtered.into_iter().filter(|device| {
            // Skip obvious virtual/duplicate devices unless specifically requested
            if device.name.starts_with("dmix:") ||
               device.name.starts_with("dsnoop:") ||
               device.name.starts_with("surround") ||
               device.name.starts_with("iec958:") {
                return false;
            }

            // For similar devices, prefer the simpler name
            let simplified_name = if device.name.starts_with("plughw:") {
                device.name.replace("plughw:", "hw:")
            } else {
                device.name.clone()
            };

            if seen_names.contains(&simplified_name) {
                false
            } else {
                seen_names.insert(simplified_name);
                true
            }
        }).collect();
    }

    filtered
}

fn get_card_mapping() -> Result<HashMap<String, String>> {
    use std::fs;
    let mut mapping = HashMap::new();

    if let Ok(contents) = fs::read_to_string("/proc/asound/cards") {
        for line in contents.lines() {
            // Parse lines like " 0 [HDMI           ]: HDA-Intel - HDA ATI HDMI"
            if let Some(stripped) = line.strip_prefix(' ') {
                if let Some(bracket_start) = stripped.find('[') {
                    if let Some(bracket_end) = stripped.find(']') {
                        if let Some(card_num) = stripped.split_whitespace().next() {
                            let card_name = stripped[bracket_start+1..bracket_end].trim().to_string();
                            mapping.insert(card_num.to_string(), card_name);
                        }
                    }
                }
            }
        }
    }

    Ok(mapping)
}

fn get_card_descriptions() -> Result<HashMap<String, String>> {
    use std::fs;
    let mut descriptions = HashMap::new();

    if let Ok(contents) = fs::read_to_string("/proc/asound/cards") {
        for line in contents.lines() {
            // Parse lines like " 0 [HDMI           ]: HDA-Intel - HDA ATI HDMI"
            if let Some(stripped) = line.strip_prefix(' ') {
                if let Some(bracket_start) = stripped.find('[') {
                    if let Some(bracket_end) = stripped.find(']') {
                        if let Some(dash_pos) = stripped.find(" - ") {
                            let card_name = stripped[bracket_start+1..bracket_end].trim().to_string();
                            let description = stripped[dash_pos+3..].trim().to_string();
                            descriptions.insert(card_name, description);
                        }
                    }
                }
            }
        }
    }

    Ok(descriptions)
}

#[cfg(target_os = "linux")]
fn get_proc_alsa_devices() -> Result<Vec<AudioDeviceInfo>> {
    use std::fs;


    let mut devices = Vec::new();

    // Check /proc/asound/ for card directories
    if let Ok(entries) = fs::read_dir("/proc/asound/") {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                if let Some(name_str) = name.to_str() {
                    if name_str.starts_with("card") {
                        let card_num = &name_str[4..];
                        let card_path = format!("/proc/asound/{}", name_str);

                        // Check for PCM devices
                        if let Ok(card_entries) = fs::read_dir(&card_path) {
                            for card_entry in card_entries {
                                if let Ok(card_entry) = card_entry {
                                    let pcm_name = card_entry.file_name();
                                    if let Some(pcm_str) = pcm_name.to_str() {
                                        // Check for playback devices (pcmXp)
                                        if pcm_str.starts_with("pcm") && pcm_str.ends_with("p") {
                                            if let Some(device_info) = read_pcm_info(&card_path, pcm_str, "PLAYBACK", card_num) {
                                                devices.push(device_info);
                                            }
                                        }
                                        // Check for capture devices (pcmXc)
                                        if pcm_str.starts_with("pcm") && pcm_str.ends_with("c") {
                                            if let Some(device_info) = read_pcm_info(&card_path, pcm_str, "CAPTURE", card_num) {
                                                devices.push(device_info);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(devices)
}

#[cfg(target_os = "linux")]
fn read_pcm_info(card_path: &str, pcm_dir: &str, stream_type: &str, card_num: &str) -> Option<AudioDeviceInfo> {
    use std::fs;

    let info_path = format!("{}/{}/info", card_path, pcm_dir);
    let stream_path = format!("{}/stream0", card_path);

    if let Ok(info_content) = fs::read_to_string(&info_path) {
        // Parse device number from pcmXp or pcmXc
        let device_num = if pcm_dir.len() > 4 {
            &pcm_dir[3..pcm_dir.len()-1]
        } else {
            "0"
        };

        // Get card name from card mapping
        let card_mapping = get_card_mapping().unwrap_or_default();
        let card_name = card_mapping.get(card_num).cloned().unwrap_or_else(|| format!("card{}", card_num));

        let device_name = format!("hw:{},{}", card_num, device_num);
        let mut device = AudioDeviceInfo::new(device_name, "ALSA".to_string());

        // Determine channels from stream info if available
        if let Ok(stream_content) = fs::read_to_string(&stream_path) {
            if let Some(channels) = parse_stream_channels(&stream_content, stream_type) {
                match stream_type {
                    "PLAYBACK" => device.output_channels = channels,
                    "CAPTURE" => device.input_channels = channels,
                    _ => {}
                }
            }
        } else {
            // Fallback: assume stereo if we can't read stream info
            match stream_type {
                "PLAYBACK" => device.output_channels = 2,
                "CAPTURE" => device.input_channels = 2,
                _ => {}
            }
        }

        device.update_device_type();

        // Check if device is in use
        let in_use = info_content.contains("subdevices_avail: 0") &&
                    info_content.contains("subdevices_count: 1");

        if in_use {
            device.name = format!("{} (IN USE)", device.name);
        }

        return Some(device);
    }

    None
}

#[cfg(target_os = "linux")]
fn parse_stream_channels(stream_content: &str, stream_type: &str) -> Option<u32> {
    let section_start = if stream_type == "PLAYBACK" {
        "Playback:"
    } else {
        "Capture:"
    };

    if let Some(start_pos) = stream_content.find(section_start) {
        let section = &stream_content[start_pos..];
        for line in section.lines() {
            if line.trim().starts_with("Channels:") {
                if let Some(channels_str) = line.split(':').nth(1) {
                    if let Ok(channels) = channels_str.trim().parse::<u32>() {
                        return Some(channels);
                    }
                }
            }
        }
    }

    None
}

#[cfg(not(target_os = "linux"))]
fn get_proc_alsa_devices() -> Result<Vec<AudioDeviceInfo>> {
    Ok(Vec::new())
}

fn show_card_summary() -> Result<()> {
    use std::fs;

    // Check /proc/asound/cards for card information
    if let Ok(contents) = fs::read_to_string("/proc/asound/cards") {
        for line in contents.lines() {
            if let Some(card_line) = line.strip_prefix(' ') {
                if !card_line.trim().is_empty() && !card_line.starts_with('-') {
                    println!("  {}", card_line.trim());
                }
            }
        }
    } else {
        println!("  (Could not read /proc/asound/cards)");
    }

    Ok(())
}

fn list_available_cards() -> Result<()> {
    use std::fs;

    println!("Available Audio Cards:");
    println!("═════════════════════");

    show_card_summary()?;

    // Also check for card directories
    println!("\nCard Directories in /proc/asound/:");
    println!("═══════════════════════════════════");

    if let Ok(entries) = fs::read_dir("/proc/asound/") {
        let mut cards = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                if let Some(name_str) = name.to_str() {
                    if name_str.starts_with("card") {
                        cards.push(name_str.to_string());
                    }
                }
            }
        }

        cards.sort();
        for card in cards {
            println!("  {}", card);
        }
    }

    println!("\nUsage examples:");
    println!("  audio-interrogator --card 0        # Show devices for card0");
    println!("  audio-interrogator --card card1    # Show devices for card1");
    println!("  audio-interrogator --device Audio  # Show devices matching 'Audio'");

    Ok(())
}
