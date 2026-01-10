#!/bin/bash
set -e

# Fetch minimal test images for Cloud Hypervisor examples
# This script downloads Cirros (minimal test OS) and hypervisor-fw for testing

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IMAGE_DIR="$SCRIPT_DIR/images"

mkdir -p "$IMAGE_DIR"

echo "==> Downloading test images for Cloud Hypervisor SDK examples..."
echo "    Images will be stored in: $IMAGE_DIR"
echo ""

if [ ! -f "$IMAGE_DIR/hypervisor-fw" ]; then
    echo "==> Downloading hypervisor-fw..."
    HYPERVISOR_FW_URL="https://github.com/cloud-hypervisor/rust-hypervisor-firmware/releases/download/0.4.2/hypervisor-fw"
    curl -L "$HYPERVISOR_FW_URL" -o "$IMAGE_DIR/hypervisor-fw"
    chmod +x "$IMAGE_DIR/hypervisor-fw"
    echo "    ✓ Downloaded hypervisor-fw"
else
    echo "==> hypervisor-fw already exists, skipping"
fi

if [ ! -f "$IMAGE_DIR/cirros.qcow2" ]; then
    echo "==> Downloading Cirros cloud image (~15MB)..."
    CIRROS_VERSION="0.6.2"
    CIRROS_URL="https://download.cirros-cloud.net/${CIRROS_VERSION}/cirros-${CIRROS_VERSION}-x86_64-disk.img"

    curl -L "$CIRROS_URL" -o "$IMAGE_DIR/cirros.qcow2"
    echo "    ✓ Downloaded Cirros cloud image"
else
    echo "==> Cirros cloud image already exists, skipping"
fi

if [ ! -f "$IMAGE_DIR/cirros.raw" ]; then
    echo "==> Converting qcow2 to raw format..."
    if command -v qemu-img >/dev/null 2>&1; then
        qemu-img convert -f qcow2 -O raw "$IMAGE_DIR/cirros.qcow2" "$IMAGE_DIR/cirros.raw"
        echo "    ✓ Converted to raw format"
        echo "    You can delete cirros.qcow2 if you want to save space"
    else
        echo "    ⚠ qemu-img not found. You'll need to convert the image manually."
        echo "    Install qemu-img: sudo pacman -S qemu-img (Arch) or sudo apt install qemu-utils (Ubuntu)"
        echo "    Then run: qemu-img convert -f qcow2 -O raw $IMAGE_DIR/cirros.qcow2 $IMAGE_DIR/cirros.raw"
    fi
else
    echo "==> Cirros raw image already exists, skipping conversion"
fi

echo ""
echo "==> Setup complete!"
echo ""
echo "Available images:"
ls -lh "$IMAGE_DIR"
echo ""
echo "Run examples with:"
echo "  cargo run --example simple_vm_cirros"
