# cloud-hypervisor-sdk

[![Cloud Hypervisor](https://img.shields.io/badge/cloud--hypervisor-v51.1.0-blue)](https://github.com/cloud-hypervisor/cloud-hypervisor/releases/tag/v51.1)
[![CI](https://github.com/qarax/cloud-hypervisor-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/qarax/cloud-hypervisor-sdk/actions/workflows/ci.yml)

A Rust SDK for [Cloud Hypervisor](https://github.com/cloud-hypervisor/cloud-hypervisor), providing a high-level API for managing virtual machines through Cloud Hypervisor's HTTP API.

The models are generated from Cloud Hypervisor's OpenAPI spec and kept in sync automatically. The high-level `Machine` and `VM` abstractions in `src/machine.rs` are hand-written for ergonomic VM lifecycle management.

## Version Convention

SDK tags mirror the Cloud Hypervisor release they were generated from:

| SDK tag | Cloud Hypervisor release |
|---------|--------------------------|
| `v51.0.0` | `v51.0` |
| `v50.0.0` | `v50.0` |
| `v49.0.0` | `v49.0` |

The pattern is: strip the `v`, append `.0` → `vX.Y` becomes `vX.Y.0`.

This means if you know which Cloud Hypervisor version you're running, you can derive the SDK tag directly without looking anything up.

## Installation

<!-- BEGIN INSTALL -->
**Pin to the latest release (Cloud Hypervisor v51.1):**
```toml
[dependencies]
cloud-hypervisor-sdk = { git = "https://github.com/qarax/cloud-hypervisor-sdk", tag = "v51.1.0" }
```

**Always use latest main:**
```toml
[dependencies]
cloud-hypervisor-sdk = { git = "https://github.com/qarax/cloud-hypervisor-sdk", branch = "main" }
```
<!-- END INSTALL -->

## Usage

### Basic VM Lifecycle

```rust
use cloud_hypervisor_sdk::{Machine, MachineConfig};
use std::borrow::Cow;
use std::path::PathBuf;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = MachineConfig {
        vm_id: Uuid::new_v4(),
        socket_path: Cow::Owned(PathBuf::from("/tmp/cloud-hypervisor.sock")),
        exec_path: Cow::Owned(PathBuf::from("/usr/bin/cloud-hypervisor")),
    };

    // Start the VMM process
    let machine = Machine::start(config).await?;

    // Create and boot a VM
    let mut vm = machine.create_vm(vm_config).await?;
    vm.boot().await?;

    // Interact
    let info = vm.get_info().await?;
    println!("{info:?}");

    // Shutdown
    vm.shutdown().await?;
    Ok(())
}
```

### Connecting to an Existing VMM

```rust
let machine = Machine::connect(config).await?;
let info = machine.get_info().await?;
```

## Examples

```bash
# Download test images (first time only)
./examples/fetch_test_images.sh

# Run the Cirros example (recommended)
cargo run --example simple_vm_cirros

# Run with local images
cargo run --example simple_vm

# Connect to a running VMM
cargo run --example connect
```

VMs are launched in detached tmux sessions. Attach with:
```bash
tmux attach -t vm_<uuid>
```

See [`examples/README.md`](examples/README.md) for full instructions.

## Building

```bash
cargo build
cargo clippy
cargo fmt
cargo doc --open
```

## Regenerating from OpenAPI Spec

The SDK models are generated from Cloud Hypervisor's OpenAPI spec. To regenerate against the latest Cloud Hypervisor release:

```bash
./regenerate.sh
```

This will:
1. Fetch the latest Cloud Hypervisor release tag from GitHub
2. Download the corresponding OpenAPI spec
3. Download the latest OpenAPI Generator
4. Regenerate `src/models/` and `docs/`
5. Apply patches for known generator bugs
6. Build and verify

Regeneration also runs automatically via GitHub Actions whenever a new Cloud Hypervisor release is published.

## Architecture

| File | Description |
|------|-------------|
| `src/machine.rs` | High-level `Machine` and `VM` abstractions |
| `src/client.rs` | HTTP client over Unix socket (hyper + tokio) |
| `src/error.rs` | Unified error types |
| `src/models/` | OpenAPI-generated request/response types |

Communication flow: Cloud Hypervisor exposes an HTTP API over a Unix domain socket (`--api-socket`). All requests are `PUT`/`GET` to `http://localhost/api/v1/vm.*` endpoints.

## License

Apache 2.0