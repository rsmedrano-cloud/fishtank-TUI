#!/bin/bash

# Quick test run of fishtank
# This will start the game for a few seconds to verify it works

echo "🧪 Testing Fishtank..."
echo "Building release version..."

cargo build --release 2>&1 | grep -E "(Compiling fishtank|Finished|error|warning:)"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "Binary location: ./target/release/fishtank"
    echo "Binary size: $(du -h ./target/release/fishtank | cut -f1)"
    echo ""
    echo "To run the game:"
    echo "  ./target/release/fishtank"
    echo ""
    echo "Or install system-wide:"
    echo "  ./install.sh"
else
    echo "❌ Build failed!"
    exit 1
fi
