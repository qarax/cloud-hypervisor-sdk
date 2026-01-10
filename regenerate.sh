#!/bin/bash
set -e

# Regenerate cloud-hypervisor SDK from OpenAPI spec
# This script downloads the OpenAPI spec and generator, then regenerates the SDK

SPEC_URL="https://raw.githubusercontent.com/cloud-hypervisor/cloud-hypervisor/main/vmm/src/api/openapi/cloud-hypervisor.yaml"
SPEC_FILE="cloud-hypervisor.yaml"
GENERATOR_VERSION="7.10.0"
GENERATOR_JAR="openapi-generator-cli.jar"
GENERATOR_URL="https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/${GENERATOR_VERSION}/openapi-generator-cli-${GENERATOR_VERSION}.jar"
TEMP_DIR=$(mktemp -d)

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get openapi-generator command
get_generator_cmd() {
    # Check if openapi-generator-cli is available via npm
    if command_exists openapi-generator-cli; then
        echo "openapi-generator-cli"
        return 0
    fi

    # Check if we have the JAR file locally
    if [ -f ".openapi-generator/$GENERATOR_JAR" ]; then
        echo "java -jar .openapi-generator/$GENERATOR_JAR"
        return 0
    fi

    # Download the JAR file
    echo "==> OpenAPI Generator not found. Downloading version $GENERATOR_VERSION..."
    mkdir -p .openapi-generator

    if command_exists curl; then
        curl -sSL "$GENERATOR_URL" -o ".openapi-generator/$GENERATOR_JAR"
    elif command_exists wget; then
        wget -q "$GENERATOR_URL" -O ".openapi-generator/$GENERATOR_JAR"
    else
        echo "ERROR: Neither curl nor wget found. Please install one of them."
        exit 1
    fi

    echo "java -jar .openapi-generator/$GENERATOR_JAR"
}

# Check for Java
if ! command_exists java; then
    echo "ERROR: Java is not installed. Please install Java JRE/JDK (version 8 or later)."
    echo ""
    echo "Installation options:"
    echo "  - Arch Linux: sudo pacman -S jre-openjdk"
    echo "  - Ubuntu/Debian: sudo apt install default-jre"
    echo "  - Fedora: sudo dnf install java-latest-openjdk"
    echo ""
    echo "Or install openapi-generator-cli via npm:"
    echo "  npm install -g @openapitools/openapi-generator-cli"
    exit 1
fi

echo "==> Downloading OpenAPI spec from GitHub..."
if command_exists curl; then
    curl -sSL "$SPEC_URL" -o "$TEMP_DIR/$SPEC_FILE"
elif command_exists wget; then
    wget -q "$SPEC_URL" -O "$TEMP_DIR/$SPEC_FILE"
else
    echo "ERROR: Neither curl nor wget found."
    exit 1
fi

GENERATOR_CMD=$(get_generator_cmd)

echo "==> Generating SDK with openapi-generator..."
echo "    Using: $GENERATOR_CMD"
$GENERATOR_CMD generate \
    -i "$TEMP_DIR/$SPEC_FILE" \
    -g rust \
    -o . \
    --additional-properties=packageName=cloud-hypervisor-sdk,packageVersion=0.1.0

echo "==> Cleaning up..."
rm -rf "$TEMP_DIR"

echo "==> Formatting code..."
cargo fmt

echo "==> Building to verify..."
cargo build

echo ""
echo "==> Regeneration complete!"
echo "    Custom files (machine.rs, client.rs, error.rs, examples/) were preserved."
echo "    Generated files (models/, apis/) have been updated."
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Update src/lib.rs if needed to export new models"
echo "  3. Test: cargo test"
echo "  4. Run examples to verify: cargo run --example simple_vm"
