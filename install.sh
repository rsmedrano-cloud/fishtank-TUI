#!/bin/bash

set -e

echo "🐟 Installing Fishtank TUI..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✓ Rust found"

# Build release binary
echo "🔨 Building optimized binary..."
cargo build --release

# Install location
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "📦 Installing to $INSTALL_DIR/fishtank..."
cp target/release/fishtank "$INSTALL_DIR/fishtank"
chmod +x "$INSTALL_DIR/fishtank"

echo ""
echo "✅ Installation complete!"
echo ""

# Check if PATH is already set
if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
    echo "✓ $INSTALL_DIR is already in your PATH"
    echo ""
    echo "🎮 You can now run: fishtank"
else
    echo "📝 Setting up PATH..."
    echo ""
    
    # Detect shell and config file
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
        SHELL_NAME="zsh"
    elif [ -n "$BASH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.bashrc"
        SHELL_NAME="bash"
    else
        # Default to bashrc
        SHELL_CONFIG="$HOME/.bashrc"
        SHELL_NAME="bash"
    fi
    
    # Check if already in config file
    if grep -q 'PATH.*\.local/bin' "$SHELL_CONFIG" 2>/dev/null; then
        echo "✓ PATH already configured in $SHELL_CONFIG"
    else
        echo "Adding PATH to $SHELL_CONFIG..."
        echo "" >> "$SHELL_CONFIG"
        echo "# Added by fishtank installer" >> "$SHELL_CONFIG"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_CONFIG"
        echo "✓ Updated $SHELL_CONFIG"
    fi
    
    echo ""
    echo "🎮 To use fishtank in this session, run:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo "  fishtank"
    echo ""
    echo "Or open a new terminal and run: fishtank"
fi

echo ""
echo "📁 Save files will be stored in:"
echo "   ~/.config/fishtank/"
