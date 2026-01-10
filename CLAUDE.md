# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust SDK for Cloud Hypervisor, providing a high-level API for managing virtual machines through Cloud Hypervisor's HTTP API. The SDK was generated from OpenAPI specs but includes custom abstractions in `machine.rs` for easier VM lifecycle management.

## Building and Testing

```bash
# Build the library
cargo build
# or
make build

# Build with release optimizations
cargo build --release

# Setup test images and run example (easiest)
make run-example

# Or manually:
# 1. Setup test images (first time only)
./examples/fetch_test_images.sh
# or
make setup-examples

# 2. Run examples (requires cloud-hypervisor binary)
cargo run --example simple_vm_cirros  # Recommended: uses downloaded Cirros image (~15MB)
cargo run --example simple_vm         # Uses local images (hypervisor-fw, fedora.raw)
cargo run --example connect
cargo run --example shutdown_vm

# See examples/README.md for detailed instructions

# Generate and view documentation
cargo doc --open
# or
make doc

# Format code
cargo fmt
# or
make fmt

# Run clippy for linting
cargo clippy
# or
make clippy
```

## Regenerating from OpenAPI Spec

When Cloud Hypervisor's API is updated, regenerate the SDK:

```bash
# Regenerate from upstream OpenAPI spec
./regenerate.sh
# or
make regenerate
```

**What gets regenerated:**
- `src/models/*` - All model structs (VmConfig, DiskConfig, etc.)
- `docs/*` - API documentation

**What is preserved (protected in `.openapi-generator-ignore`):**
- `src/machine.rs` - High-level Machine/VM abstraction
- `src/client.rs` - HTTP client implementation
- `src/error.rs` - Error types
- `src/lib.rs` - Library exports
- `src/apis/**` - Generated API client (not used, we use custom machine.rs)
- `examples/*` - Example code
- `Cargo.toml` - Custom dependencies and metadata

**Requirements:**
- Java 8+ (the script auto-downloads the OpenAPI generator)

After regeneration, verify with `cargo build` and test examples.

## Architecture

### Core Components

**`machine.rs`** - High-level abstraction layer
- `Machine`: Manages a Cloud Hypervisor VMM instance
  - `Machine::start()`: Launches Cloud Hypervisor in a tmux session and establishes HTTP connection via Unix socket
  - `Machine::connect()`: Connects to an existing running VMM instance
  - `Machine::create_vm()`: Creates a VM and returns a `VM` handle
- `VM`: Represents a created VM instance
  - `VM::boot()`: Boots the VM
  - `VM::get_info()`: Retrieves VM information
  - `VM::shutdown()`: Shuts down the VM
- `MachineConfig`: Configuration struct containing:
  - `vm_id`: UUID for the VM
  - `socket_path`: Path to Unix socket for API communication
  - `exec_path`: Path to cloud-hypervisor binary

**`client.rs`** - HTTP client implementation
- `HttpClient`: Wrapper around hyper's HTTP/1.1 client over Unix sockets
- `TokioIo`: Adapter between tokio and hyper I/O traits for async Unix socket communication

**`models/`** - OpenAPI-generated data models
- All Cloud Hypervisor API request/response types
- Key models: `VmConfig`, `VmInfo`, `CpusConfig`, `MemoryConfig`, `DiskConfig`, `PayloadConfig`, etc.
- These mirror Cloud Hypervisor's API v1 schema

**`error.rs`** - Error types
- Unified error handling using thiserror
- `CloudHypervisorApiError` includes HTTP status code and error body for API failures

### Communication Flow

1. Cloud Hypervisor runs with `--api-socket <path>` flag (started via tmux)
2. HTTP client connects to the Unix socket using hyper over TokioIo adapter
3. All API calls are PUT/GET requests to `http://localhost/api/v1/vm.*` endpoints
4. Responses are parsed into model structs or error types

### Key Design Patterns

- **Builder pattern**: VM configuration uses nested Option types for flexibility
- **Async-first**: All I/O operations use tokio async runtime
- **Unix socket communication**: VMM API is accessed via Unix domain socket, not TCP
- **tmux integration**: VMMs are launched in detached tmux sessions for easy debugging (attach with `tmux attach -t vm_<uuid>`)

## Working with VMs

### Basic VM Lifecycle

```rust
// 1. Configure machine
let config = MachineConfig {
    vm_id: Uuid::new_v4(),
    socket_path: Cow::Owned(PathBuf::from("/tmp/cloud-hypervisor.sock")),
    exec_path: Cow::Owned(PathBuf::from("./cloud-hypervisor")),
};

// 2. Start VMM
let machine = Machine::start(config).await?;

// 3. Create VM with config
let mut vm = machine.create_vm(vm_config).await?;

// 4. Boot VM
vm.boot().await?;

// 5. Interact with VM
let info = vm.get_info().await?;

// 6. Shutdown
vm.shutdown().await?;
```

### Connecting to Existing VMM

```rust
// Connect to already-running Cloud Hypervisor instance
let machine = Machine::connect(config).await?;
let vm_info = machine.get_info().await?;
```

## Dependencies

- **hyper/http**: HTTP client stack (v1.x)
- **tokio**: Async runtime with full features
- **hyperlocal**: Unix socket support for hyper
- **serde/serde_json**: Serialization for API models
- **uuid**: VM identifiers
- **thiserror**: Error type derivation

## Testing and Examples

See `examples/README.md` for comprehensive testing instructions.

**Quick start:**
1. Download test images: `./examples/fetch_test_images.sh`
2. Get Cloud Hypervisor binary (if not installed)
3. Run example: `cargo run --example simple_vm_cirros`

The Cirros example boots a minimal test VM successfully with serial console output visible in tmux.

## Important Notes

- The SDK requires a cloud-hypervisor binary at the configured `exec_path`
- Socket paths must be cleaned up between runs (SDK handles this in `Machine::start()`)
- VM assets (kernel firmware, disk images) must exist at configured paths
- All API operations are async and require a tokio runtime
- Error responses from Cloud Hypervisor API include both status code and error message body
- Examples run VMs in tmux sessions for easy debugging (attach with `tmux attach -t vm_<uuid>`)
- Serial console output requires `ConsoleConfig` with `Mode::Tty` to be visible in tmux
- Cirros test image boots successfully without cloud-init (login: cirros/gocubsgo)
