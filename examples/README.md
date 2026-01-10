# Cloud Hypervisor SDK Examples

This directory contains examples demonstrating how to use the Cloud Hypervisor SDK.

## Prerequisites

### 1. Install Cloud Hypervisor

Download the Cloud Hypervisor binary:

```bash
# Option 1: Download latest release
wget https://github.com/cloud-hypervisor/cloud-hypervisor/releases/latest/download/cloud-hypervisor
chmod +x cloud-hypervisor
mv cloud-hypervisor /usr/local/bin/  # or keep in project root

# Option 2: Build from source
git clone https://github.com/cloud-hypervisor/cloud-hypervisor.git
cd cloud-hypervisor
cargo build --release
cp target/release/cloud-hypervisor ..
```

### 2. Fetch Test Images

For testing, download a minimal OS image (Alpine Linux):

```bash
./examples/fetch_test_images.sh
```

This will download (~15MB):
- `hypervisor-fw` - UEFI firmware for VMs
- `cirros.raw` - Minimal Cirros Linux test image

## Available Examples

### `simple_vm_cirros.rs`
Creates and boots a VM with Cirros Linux using minimal resources (1 CPU, 512MB RAM).

```bash
cargo run --example simple_vm_cirros
```

**What it does:**
- Starts Cloud Hypervisor in a tmux session
- Creates a VM with Alpine Linux
- Boots the VM
- Shows VM state

**Login credentials:**
- Username: `cirros`
- Password: `gocubsgo`

### `simple_vm.rs`
Original example requiring local VM images (hypervisor-fw and disk image).

```bash
cargo run --example simple_vm
```

### `connect.rs`
Connects to an existing Cloud Hypervisor instance and retrieves VM info.

```bash
cargo run --example connect
```

### `shutdown_vm.rs`
Shuts down a running VM.

```bash
cargo run --example shutdown_vm
```

## Managing VMs

### Viewing VM Console

Each VM runs in a tmux session. **Important**: Due to serial console quirks, the tmux display may appear stuck at the GRUB menu even though the VM has booted. See solutions below.

#### Using the Helper Script (Recommended)

```bash
# List running VMs
./examples/vm_console.sh list

# Auto-login (handles serial console quirks)
./examples/vm_console.sh login vm_<uuid>

# Execute commands in the VM
./examples/vm_console.sh exec vm_<uuid> 'uname -a'

# View current output without attaching
./examples/vm_console.sh view vm_<uuid>

# Attach to console (with usage tips)
./examples/vm_console.sh attach vm_<uuid>

# Kill a VM session
./examples/vm_console.sh kill vm_<uuid>
```

#### Manual tmux Attachment

```bash
# The session name is printed when starting the VM
tmux attach -t vm_<uuid>

# If display appears stuck at GRUB:
# 1. Press Ctrl+b then ]
# 2. Press Shift+G to jump to bottom
# 3. Press q to exit copy mode

# List all tmux sessions
tmux ls

# Detach from tmux session (while attached)
# Press: Ctrl+b, then d

# View output without attaching
tmux capture-pane -t vm_<uuid> -p -S -1000 | tail -30
```

**Note**: To see VM output in the tmux session, the VM must be configured with serial console output. The `simple_vm_alpine` example includes this configuration:

```rust
let serial_config = ConsoleConfig {
    mode: cloud_hypervisor_sdk::models::console_config::Mode::Tty,
    file: None,
    socket: None,
    iommu: None,
};

let vm_config = VmConfig {
    serial: Some(Box::new(serial_config)),
    // ... other config
};
```

### Cleanup

To stop all VMs and clean up:

```bash
# Kill all Cloud Hypervisor processes
pkill cloud-hypervisor

# Clean up socket files
rm /tmp/cloud-hypervisor*.sock

# Remove downloaded images (optional)
rm -rf examples/images/
```

## Custom Images

You can use your own VM images:

1. **Get a cloud image** (e.g., Ubuntu, Fedora, Alpine)
2. **Convert to raw format** (if needed):
   ```bash
   qemu-img convert -f qcow2 -O raw image.qcow2 image.raw
   ```
3. **Update example code** to point to your image paths

### Popular Cloud Images

- **Cirros**: https://download.cirros-cloud.net/ (smallest, ~15MB, great for testing)
- **Alpine Linux**: https://alpinelinux.org/cloud/ (~50MB)
- **Ubuntu**: https://cloud-images.ubuntu.com/
- **Fedora**: https://fedoraproject.org/cloud/download

## Troubleshooting

**Error: "can't find crate for cloud_hypervisor_sdk"**
```bash
cargo build
```

**Error: "cloud-hypervisor not found"**
- Ensure `cloud-hypervisor` binary is in PATH or project root
- Download from: https://github.com/cloud-hypervisor/cloud-hypervisor/releases

**Error: "Failed to connect to Unix socket"**
- Check if Cloud Hypervisor is running: `ps aux | grep cloud-hypervisor`
- Ensure socket path is correct
- Clean up old sockets: `rm /tmp/cloud-hypervisor*.sock`

**VM doesn't boot**
- Check tmux session: `tmux attach -t vm_<uuid>`
- Verify image format is raw (not qcow2)
- Ensure sufficient permissions on image files

**Display appears stuck at GRUB menu**
- This is a serial console quirk - the VM has likely already booted
- **Solution**: Use `./examples/vm_console.sh view vm_<uuid>` to see current output
- Or use `./examples/vm_console.sh login vm_<uuid>` to auto-login
- Or manually scroll in tmux: Ctrl+b, ], Shift+G, q

**No output in tmux session**
- The VM config needs serial console enabled (see "Viewing VM Console" section)
- Ensure you're using the correct tmux session name (printed when VM starts)
- Check scrollback: `tmux capture-pane -t <session> -p -S -1000 | tail -30`

**Login fails or password echoes to screen**
- This is a serial console timing issue
- **Solution**: Use `./examples/vm_console.sh login vm_<uuid>` which handles the timing
- Avoid typing while GRUB menu is displayed (wait for full boot)

**VM boots to login prompt**
- Success! The `simple_vm_cirros` example boots Cirros successfully
- Login with username `cirros` and password `gocubsgo`
- You can interact with the VM through the tmux session

**Cloud images requiring cloud-init**
- Some cloud images (Alpine, Ubuntu Cloud) expect cloud-init configuration
- These images need metadata and user-data to boot properly
- **Solutions**:
  1. Use Cirros (doesn't require cloud-init, great for testing)
  2. Create a cloud-init ISO and attach it as a second disk
  3. Use direct kernel boot (extract kernel/initramfs)
  4. Use non-cloud versions of the OS
