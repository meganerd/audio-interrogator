#!/bin/bash

# Audio Interrogator Installation Script
# This script installs the audio-interrogator tool system-wide

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="audio-interrogator"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check dependencies
check_dependencies() {
    print_status "Checking dependencies..."

    # Check for Rust/Cargo
    if ! command_exists cargo; then
        print_error "Cargo (Rust) is not installed!"
        print_status "Please install Rust from https://rustup.rs/"
        exit 1
    fi

    # Check for ALSA development libraries on Linux
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if ! pkg-config --exists alsa; then
            print_warning "ALSA development libraries not found!"
            print_status "Installing ALSA development libraries..."

            # Try different package managers
            if command_exists apt-get; then
                sudo apt-get update && sudo apt-get install -y libasound2-dev pkg-config
            elif command_exists dnf; then
                sudo dnf install -y alsa-lib-devel pkgconfig
            elif command_exists pacman; then
                sudo pacman -S --noconfirm alsa-lib pkgconf
            elif command_exists zypper; then
                sudo zypper install -y alsa-devel pkg-config
            else
                print_error "Could not install ALSA libraries automatically."
                print_status "Please install ALSA development libraries manually:"
                print_status "  Ubuntu/Debian: sudo apt-get install libasound2-dev pkg-config"
                print_status "  Fedora/RHEL:   sudo dnf install alsa-lib-devel pkgconfig"
                print_status "  Arch Linux:    sudo pacman -S alsa-lib pkgconf"
                exit 1
            fi
        fi
    fi

    print_success "All dependencies are satisfied!"
}

# Function to build the project
build_project() {
    print_status "Building audio-interrogator in release mode..."

    if cargo build --release; then
        print_success "Build completed successfully!"
    else
        print_error "Build failed!"
        exit 1
    fi
}

# Function to install the binary
install_binary() {
    print_status "Installing binary to $INSTALL_DIR..."

    # Check if install directory exists and is writable
    if [[ ! -d "$INSTALL_DIR" ]]; then
        print_status "Creating install directory: $INSTALL_DIR"
        sudo mkdir -p "$INSTALL_DIR"
    fi

    # Copy the binary
    if sudo cp "target/release/$BINARY_NAME" "$INSTALL_DIR/"; then
        sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
        print_success "Binary installed to $INSTALL_DIR/$BINARY_NAME"
    else
        print_error "Failed to install binary!"
        exit 1
    fi
}

# Function to verify installation
verify_installation() {
    print_status "Verifying installation..."

    if command_exists "$BINARY_NAME"; then
        local version
        version=$("$BINARY_NAME" --version 2>/dev/null | head -n1)
        print_success "Installation verified! $version"
        print_status "You can now run '$BINARY_NAME' from anywhere!"
    else
        print_warning "Binary installed but not found in PATH."
        print_status "You may need to add $INSTALL_DIR to your PATH or restart your shell."
    fi
}

# Function to show usage information
show_usage() {
    cat << EOF
Audio Interrogator Installation Script

Usage: $0 [OPTIONS]

OPTIONS:
    --help, -h          Show this help message
    --install-dir DIR   Set custom installation directory (default: $INSTALL_DIR)
    --skip-deps         Skip dependency checking
    --uninstall         Uninstall audio-interrogator

Examples:
    $0                                 # Standard installation
    $0 --install-dir ~/bin            # Install to custom directory
    $0 --uninstall                    # Remove installation

EOF
}

# Function to uninstall
uninstall() {
    print_status "Uninstalling audio-interrogator..."

    if [[ -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
        if sudo rm "$INSTALL_DIR/$BINARY_NAME"; then
            print_success "Audio-interrogator has been uninstalled!"
        else
            print_error "Failed to remove binary!"
            exit 1
        fi
    else
        print_warning "Audio-interrogator is not installed in $INSTALL_DIR"
    fi
}

# Main installation function
main() {
    local skip_deps=false
    local uninstall_mode=false

    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                show_usage
                exit 0
                ;;
            --install-dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --skip-deps)
                skip_deps=true
                shift
                ;;
            --uninstall)
                uninstall_mode=true
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done

    # Handle uninstall mode
    if [[ "$uninstall_mode" == true ]]; then
        uninstall
        exit 0
    fi

    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]] || ! grep -q "audio-interrogator" Cargo.toml; then
        print_error "This script must be run from the audio-interrogator project directory!"
        exit 1
    fi

    print_status "Starting audio-interrogator installation..."
    print_status "Install directory: $INSTALL_DIR"

    # Check dependencies unless skipped
    if [[ "$skip_deps" != true ]]; then
        check_dependencies
    fi

    # Build and install
    build_project
    install_binary
    verify_installation

    print_success "Installation complete!"
    print_status ""
    print_status "Quick start:"
    print_status "  $BINARY_NAME              # Show all audio devices"
    print_status "  $BINARY_NAME --verbose    # Show detailed device info"
    print_status "  $BINARY_NAME --json       # Output in JSON format"
    print_status "  $BINARY_NAME --help       # Show all options"
}

# Run main function
main "$@"
