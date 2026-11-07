#!/bin/bash
set -euo pipefail

# Regenerate the Cloud Hypervisor SDK models from the upstream OpenAPI spec.
#
# This is the same script the sync workflow runs, so a local run produces the
# same result as CI.
#
# Environment:
#   CH_TAG                     Cloud Hypervisor release to generate from
#                              (default: the latest release)
#   OPENAPI_GENERATOR_VERSION  openapi-generator version (default: the version
#                              pinned in openapitools.json)
#   GITHUB_TOKEN               optional, authenticates GitHub API requests
#
# Hand-written code (src/lib.rs, src/machine.rs, src/client.rs, src/error.rs,
# Cargo.toml, examples/) is protected by .openapi-generator-ignore.

cd "$(dirname "${BASH_SOURCE[0]}")"

UPSTREAM_REPO="cloud-hypervisor/cloud-hypervisor"

command_exists() {
	command -v "$1" >/dev/null 2>&1
}

# fetch URL [OUTPUT]: fail on HTTP errors instead of saving an error page.
fetch() {
	local auth=()
	if [ -n "${GITHUB_TOKEN:-}" ] && [[ "$1" == https://api.github.com/* ]]; then
		auth=(-H "Authorization: Bearer ${GITHUB_TOKEN}")
	fi
	if [ $# -gt 1 ]; then
		curl -fsSL "${auth[@]}" "$1" -o "$2"
	else
		curl -fsSL "${auth[@]}" "$1"
	fi
}

for cmd in curl java cargo; do
	if ! command_exists "$cmd"; then
		echo "ERROR: $cmd is required." >&2
		exit 1
	fi
done

if [ -z "${CH_TAG:-}" ]; then
	echo "==> Fetching latest Cloud Hypervisor release tag..."
	CH_TAG=$(fetch "https://api.github.com/repos/${UPSTREAM_REPO}/releases/latest" |
		sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' | head -1)
	if [ -z "$CH_TAG" ]; then
		echo "ERROR: Could not determine the latest release tag." >&2
		exit 1
	fi
fi
echo "    Cloud Hypervisor release: ${CH_TAG}"

# SDK version: vX.Y -> X.Y.0, a full vX.Y.Z is kept as is.
SDK_VERSION="${CH_TAG#v}"
if [[ "$SDK_VERSION" =~ ^[0-9]+\.[0-9]+$ ]]; then
	SDK_VERSION="${SDK_VERSION}.0"
fi
if ! [[ "$SDK_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
	echo "ERROR: Cannot derive an SDK version from tag ${CH_TAG}." >&2
	exit 1
fi
SDK_TAG="v${SDK_VERSION}"

# The generator version is pinned so a new generator release can't change the
# output (or break the build) on its own. Bump it in openapitools.json.
if [ -z "${OPENAPI_GENERATOR_VERSION:-}" ]; then
	OPENAPI_GENERATOR_VERSION=$(sed -n 's/.*"version": *"\([^"]*\)".*/\1/p' openapitools.json | head -1)
fi
if [ -z "$OPENAPI_GENERATOR_VERSION" ]; then
	echo "ERROR: No openapi-generator version pinned in openapitools.json." >&2
	exit 1
fi
echo "    openapi-generator: v${OPENAPI_GENERATOR_VERSION}"

GENERATOR_JAR=".openapi-generator/openapi-generator-cli-${OPENAPI_GENERATOR_VERSION}.jar"
if [ ! -f "$GENERATOR_JAR" ]; then
	echo "==> Downloading openapi-generator v${OPENAPI_GENERATOR_VERSION}..."
	mkdir -p .openapi-generator
	fetch "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/${OPENAPI_GENERATOR_VERSION}/openapi-generator-cli-${OPENAPI_GENERATOR_VERSION}.jar" \
		"${GENERATOR_JAR}.tmp"
	mv "${GENERATOR_JAR}.tmp" "$GENERATOR_JAR"
fi

TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT
SPEC_FILE="$TEMP_DIR/cloud-hypervisor.yaml"

echo "==> Downloading OpenAPI spec (${CH_TAG})..."
fetch "https://raw.githubusercontent.com/${UPSTREAM_REPO}/${CH_TAG}/vmm/src/api/openapi/cloud-hypervisor.yaml" "$SPEC_FILE"

echo "==> Generating SDK..."
java -jar "$GENERATOR_JAR" generate \
	-i "$SPEC_FILE" \
	-g rust \
	-o . \
	--additional-properties=packageName=cloud-hypervisor-sdk,packageVersion="${SDK_VERSION}"

echo "==> Updating version metadata..."
echo "$CH_TAG" >.cloud-hypervisor-version
sed -i "0,/^version = \".*\"/s//version = \"${SDK_VERSION}\"/" Cargo.toml

# README: shields.io badge and the install snippet.
sed -i "s|cloud--hypervisor-v[0-9.]*-blue|cloud--hypervisor-${SDK_TAG}-blue|g" README.md
sed -i "s|${UPSTREAM_REPO}/releases/tag/v[0-9.]*)|${UPSTREAM_REPO}/releases/tag/${CH_TAG})|g" README.md
sed -i "/<!-- BEGIN INSTALL -->/,/<!-- END INSTALL -->/{
	s|(Cloud Hypervisor v[0-9.]*)|(Cloud Hypervisor ${CH_TAG})|
	s|tag = \"v[0-9.]*\"|tag = \"${SDK_TAG}\"|
}" README.md

echo "==> Formatting code..."
cargo fmt

echo "==> Building to verify (including examples)..."
cargo build --all-targets

echo ""
echo "==> Regeneration complete: Cloud Hypervisor ${CH_TAG} -> SDK ${SDK_TAG}"
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Update src/lib.rs, src/machine.rs or examples/ if the API changed"
echo "  3. Test: cargo clippy --all-targets -- -D warnings && cargo test"
