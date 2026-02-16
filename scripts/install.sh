#!/usr/bin/env bash

# Rune Installation Script
# This script downloads and installs the latest binary release of Rune

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

function error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

function info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

function success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

function warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

function check_platform() {
    local kernel=$(uname -s)
    local machine=$(uname -m)

    if [[ "$kernel" == "Linux" ]]; then
        OS="linux"
    elif [[ "$kernel" == "Darwin" ]]; then
        OS="darwin"
    else
        error "Unsupported platform: $kernel"
        exit 1
    fi

    if [[ "$machine" == "x86_64" ]]; then
        ARCH="x86_64"
    elif [[ "$machine" == "aarch64" ]] || [[ "$machine" == "arm64" ]]; then
        ARCH="aarch64"
    else
        error "Unsupported architecture: $machine"
        exit 1
    fi

    info "Detected platform: $OS-$ARCH"
}

function check_dependencies() {
    local deps=("curl" "unzip")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            error "$dep is required but not installed."
            exit 1
        fi
    done
}

function get_latest_version() {
    if [[ -n "${VERSION:-}" ]]; then
        echo "$VERSION"
        return
    fi

    info "Fetching latest version from GitHub..." >&2

    # Use redirect approach to avoid API rate limits
    local latest_url="https://github.com/sagea-ai/rune/releases/latest"
    local version=$(curl -Ls -o /dev/null -w '%{url_effective}' "$latest_url" | grep -oE 'v[0-9]+\.[0-9]+\.[0-9]+$')

    if [[ -z "$version" ]]; then
        error "Failed to fetch latest version. Please check your internet connection or specify VERSION manually."
        error "Example: VERSION=v2.1.2 bash install.sh"
        exit 1
    fi

    echo "$version"
}

function download_and_install() {
    local version=$1
    # Strip 'v' prefix if present for filename construction if needed,
    # but based on the workflow, the zip name uses the version *without* v?
    # Wait, the workflow does: `steps.get_version_unix.outputs.version`.
    # `uv version` returns just the number (e.g. 0.1.0).
    # The release tag usually has 'v' (e.g. v0.1.0).
    # so we need to be careful.

    local clean_version="${version#v}"
    local filename="rune-${OS}-${ARCH}-${clean_version}.zip"
    local download_url="https://github.com/sagea-ai/rune/releases/download/${version}/${filename}"
    local tmp_dir=$(mktemp -d)

    info "Downloading $filename from $download_url..."
    if ! curl -L -f -o "$tmp_dir/$filename" "$download_url"; then
        error "Failed to download release asset."
        exit 1
    fi

    info "Extracting..."
    unzip -q "$tmp_dir/$filename" -d "$tmp_dir"

    # Determine install path
    local install_path=""
    if [[ -w "/usr/local/bin" ]]; then
        install_path="/usr/local/bin"
    else
        install_path="$HOME/.local/bin"
        mkdir -p "$install_path"
    fi

    info "Installing to $install_path..."

    # Check for sudo requirement if using /usr/local/bin and not root
    local use_sudo=false
    if [[ "$install_path" == "/usr/local/bin" && "$EUID" -ne 0 ]]; then
        use_sudo=true
        info "Elevated permissions required to install to /usr/local/bin"
    fi

    for binary in "rune" "rune-acp"; do
        if [[ -f "$tmp_dir/$binary" ]]; then
            if [[ "$use_sudo" == "true" ]]; then
                sudo mv "$tmp_dir/$binary" "$install_path/"
                sudo chmod +x "$install_path/$binary"
            else
                mv "$tmp_dir/$binary" "$install_path/"
                chmod +x "$install_path/$binary"
            fi
        else
             warning "Binary $binary not found in archive."
        fi
    done

    # Cleanup
    rm -rf "$tmp_dir"

    # Path check
    if [[ ":$PATH:" != *":$install_path:"* ]]; then
        warning "$install_path is not in your PATH."
        warning "Please add it to your PATH to run rune."
    fi
}

function main() {
    cat << 'EOF'
[38;2;147;197;253m ░ ░░  ░░░░░░   ░   ░ ░     ░   ░   ░   ░░          ░  ░░     ░  ░ ░  ░  ░  ░          ░         ░[0m
[38;2;147;197;253m    ░  ░░▒█▓░░ ░░     ░░░░░░░░░░░░░░░░░   ░░░░░░░   ░ ░░░░░░░░░░░░░░     ░░░░░░░ ░░░░░░░░░░░░░░░░░░░[0m
[38;2;96;165;250m     ░░▒▓▒██▒█▒░░  ░   ▒██████████████▒░░ ░████▓░ ░  ░░▓████░░▓████░░░   ░▒████▒ ▒████████████████▓░[0m
[38;2;96;165;250m   ░░▒▒██▒██▒█▓▒▒░     ▒████▓░    ░░█████░░████▓░  ░░░░▓████░░▓███████░  ░▒████▒ ▒████▓░░        ░[0m
[38;2;59;130;246m ░░░▓█▓▓█▒▒▒▒█▓▓█▒░░  ░▒████▓░     ░█████ ░████▓░░    ░▓████░░▓███████▓▓░░▒████▒ ▒████▓░  ░    ░░ ░[0m
[38;2;59;130;246m ░█▓▓█▒░▓▓░░▓▒░▓█▓▓▓░ ░▒████▓░   ▒▓▓█████ ░████▓░    ░░▓████░░▓████▓▒▓██▓▓▓████▒ ▒█████▓▓▓▓▓▓▓▓▓░░[0m
[38;2;37;99;235m ░▒▓▓██▒░▒█▓░░▒██▓▓█▒  ▒████▓░░░░▓██████▓ ░████▓░    ░░▓████░░▓████░ ░█████████▒ ▒██████████████░░[0m
[38;2;37;99;235m░▒█▓░▓██▓▒█▓▒███▒▒▓█▒  ▒████████████  ░   ░████▓░ ░ ░ ░▓████░░▓████░  ░░███████▒ ▒████▓░░░[0m
[38;2;29;78;216m ▒██░░░▓██████▓░░▒██▒ ░▒████▓░░███████▒   ░████▓░    ░░▓████░░▓████░ ░ ░░░▒████▒ ▒████▓░    ░[0m
[38;2;29;78;216m ░▓██▓░░░███▓░░▒███▒░  ▒████▓░░▒▒▓█████▓▒ ░▒▒▓██▓▓▓▓▓▓▓██▓▒▒░░▓████░     ░▒████▒ ▒█████▓▓▓▓▓▓▓▓▓▓▓▒░[0m
[38;2;30;64;175m  ░░▓██▓▒▒██▒▒███▓░░  ░░▓▓▓▓▒░░  ▒▓▓▓▓▓▓▒   ░░▓▓▓▓▓▓▓▓▓▓▓▒░░░ ▒▓▓▓▓░  ░  ░░▓▓▓▓▒ ░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░[0m
[38;2;30;64;175m ░  ░░░░░░░░░░░░░░       ░                       ░  ░ ░ ░        ░      ░  ░       ░  ░  ░ ░[0m

EOF
    echo
    echo "Starting Rune installation..."
    echo
    check_platform
    check_dependencies

    local version=$(get_latest_version)
    info "Latest version: $version"

    download_and_install "$version"

    success "Installation completed successfully!"
    echo
    echo "You can now run rune with:"
    echo "  rune"
    echo
    echo "Or for ACP mode:"
    echo "  rune-acp"
}

main
