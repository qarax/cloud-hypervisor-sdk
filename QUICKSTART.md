# Cloud Hypervisor SDK - Quick Start Guide

Get a VM running in under 2 minutes!

## Prerequisites

1. **Linux system** (Ubuntu, Arch, Fedora, etc.)
2. **Rust toolchain** (https://rustup.rs)
3. **qemu-img** for image conversion
   ```bash
   # Ubuntu/Debian
   sudo apt install qemu-utils
   
   # Arch Linux
   sudo pacman -S qemu-img
   
   # Fedora
   sudo dnf install qemu-img
   ```
4. **tmux** (usually pre-installed)

## 5-Step Quick Start

### 1. Get Cloud Hypervisor Binary

```bash
# Download latest static binary
wget https://github.com/cloud-hypervisor/cloud-hypervisor/releases/download/v41.0/cloud-hypervisor-static
chmod +x cloud-hypervisor-static
sudo mv cloud-hypervisor-static /usr/local/bin/cloud-hypervisor

# OR symlink in project root
ln -s cloud-hypervisor-static cloud-hypervisor
```

### 2. Clone and Build

```bash
git clone <your-repo-url>
cd cloud-hypervisor-sdk
cargo build
```

### 3. Download Test Images

```bash
./examples/fetch_test_images.sh
```

This downloads:
- `hypervisor-fw` (UEFI firmware, ~89KB)
- `cirros.raw` (minimal Linux, ~15MB)

### 4. Run Example

```bash
cargo run --example simple_vm_cirros
```

You'll see:
```
Starting Cloud Hypervisor VM with Cirros Linux...
Cloud Hypervisor started successfully in a tmux session, you can attach to it using:

 tmux attach -t vm_<some-uuid>

Creating VM...
Booting VM...
Getting VM info...
VM State: Running

Cirros login credentials:
  username: cirros
  password: gocubsgo
```

### 5. Connect to VM

```bash
# Use the session name from step 4
tmux attach -t vm_<uuid>
```

Login when prompted:
- **Username**: `cirros`
- **Password**: `gocubsgo`

You're now logged into a running VM! 🎉

```bash
$ uname -a
Linux cirros 5.15.0-71-generic x86_64 GNU/Linux

$ df -h
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda1       112M   18M   89M  17% /

$ free -m
             total       used       free     shared    buffers     cached
Mem:           495         32        462          0          0         13
-/+ buffers/cache:         19        475
Swap:            0          0          0
```

## Important: Serial Console & tmux Quirks

### The Display Issue

When you attach to the VM's tmux session with `tmux attach -t vm_<uuid>`, you might see the GRUB boot menu screen even though the VM has already booted to the login prompt. This happens because:

1. **tmux shows the screen buffer at your attach point** - not necessarily the latest output
2. **Serial console output has scrolled past** the visible area
3. **The display doesn't auto-refresh** to show new content

**What you'll see** (appears stuck):
```
*CirrOS
The highlighted entry will be executed automatically in 0s.
```

**What's actually there** (scrolled up):
```
login as 'cirros' user. default password: 'gocubsgo'. use 'sudo' for root.
cirros login:
```

### Solutions

#### Option 1: Use the Helper Script (Recommended)

We provide `examples/vm_console.sh` to handle the serial console quirks:

```bash
# List running VMs
./examples/vm_console.sh list

# Auto-login to a VM
./examples/vm_console.sh login vm_<uuid>

# Execute a command
./examples/vm_console.sh exec vm_<uuid> 'uname -a'

# View output without attaching
./examples/vm_console.sh view vm_<uuid>

# Attach properly (with instructions)
./examples/vm_console.sh attach vm_<uuid>

# Kill a VM
./examples/vm_console.sh kill vm_<uuid>

# Kill all VMs
./examples/vm_console.sh killall
```

**Example workflow:**
```bash
# Start VM
cargo run --example simple_vm_cirros

# Auto-login (handles the serial console quirks)
./examples/vm_console.sh login vm_936f56dd-58c1-4f58-974e-c3cd6a9af276

# Run commands
./examples/vm_console.sh exec vm_936f56dd-58c1-4f58-974e-c3cd6a9af276 'free -m'
./examples/vm_console.sh exec vm_936f56dd-58c1-4f58-974e-c3cd6a9af276 'df -h'

# View output
./examples/vm_console.sh view vm_936f56dd-58c1-4f58-974e-c3cd6a9af276
```

#### Option 2: Manual tmux Navigation

If you prefer to attach manually:

```bash
tmux attach -t vm_<uuid>

# Then immediately:
# 1. Press Ctrl+b then ] (enter copy mode)
# 2. Press Shift+G (jump to bottom)
# 3. Press q (exit copy mode)
# 4. Now you'll see the login prompt
```

#### Option 3: View Without Attaching

```bash
# See the latest output
tmux capture-pane -t vm_<uuid> -p -S -1000 | tail -30

# Watch for login prompt
tmux capture-pane -t vm_<uuid> -p -S -1000 | grep -A 5 "login"
```

### Why This Happens

Serial console over tmux has some quirks:

1. **Buffer position**: tmux maintains a scrollback buffer and attaching doesn't auto-scroll to the bottom
2. **Character echoing**: Sometimes input gets echoed when it shouldn't (especially with passwords)
3. **Input buffering**: If you press keys while GRUB is showing, those keypresses get buffered and can interfere with login
4. **No visual feedback**: The password field doesn't show typing, but characters might still echo due to serial console behavior

The helper script works around these by:
- Typing credentials character-by-character with delays
- Clearing input buffers before attempting login
- Checking for prompts before sending input
- Providing clear feedback about the login state

## Detach from tmux (VM keeps running)

Press: **Ctrl+b**, then **d**

## Shutdown VM

```bash
cargo run --example shutdown_vm
```

Or just kill the tmux session:
```bash
tmux kill-session -t vm_<uuid>
```

## Using the Makefile

Even simpler:

```bash
make run-example    # Download images + run VM
make doc            # View API documentation
make fmt            # Format code
make clippy         # Lint code
```

## What's Happening Under the Hood?

1. **Machine::start()** launches Cloud Hypervisor in a detached tmux session
2. **Machine::create_vm()** sends VM configuration via HTTP to Unix socket
3. **VM::boot()** boots the VM using the configured disk and firmware
4. Serial console output goes to the tmux terminal (Mode::Tty)
5. VM runs until explicitly shut down

## Next Steps

### Writing Your Own Code

```rust
use cloud_hypervisor_sdk::{
    machine::{Machine, MachineConfig},
    models::{ConsoleConfig, CpusConfig, DiskConfig, PayloadConfig, VmConfig, 
             memory_config::MemoryConfig, console_config::Mode},
};
use std::{borrow::Cow, path::PathBuf};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure machine
    let config = MachineConfig {
        vm_id: Uuid::new_v4(),
        socket_path: Cow::Owned(PathBuf::from("/tmp/ch.sock")),
        exec_path: Cow::Owned(PathBuf::from("./cloud-hypervisor")),
    };

    // Start Cloud Hypervisor
    let machine = Machine::start(config).await?;

    // Create VM config
    let vm_config = VmConfig {
        cpus: Some(Box::new(CpusConfig {
            boot_vcpus: 1,
            max_vcpus: 1,
            ..Default::default()
        })),
        memory: Some(Box::new(MemoryConfig::new(512 * 1024 * 1024))),
        payload: Box::new(PayloadConfig {
            kernel: Some("examples/images/hypervisor-fw".to_string()),
            ..Default::default()
        }),
        disks: Some(vec![DiskConfig {
            path: Some("examples/images/cirros.raw".to_string()),
            ..Default::default()
        }]),
        serial: Some(Box::new(ConsoleConfig {
            mode: Mode::Tty,
            ..Default::default()
        })),
        ..Default::default()
    };

    // Create and boot VM
    let mut vm = machine.create_vm(vm_config).await?;
    vm.boot().await?;

    // Get VM info
    let info = vm.get_info().await?;
    println!("VM State: {:?}", info.state);

    Ok(())
}
```

### Using Different Images

See `examples/README.md` for using:
- Ubuntu Cloud Images
- Alpine Linux
- Fedora Cloud
- Custom disk images

### Production Deployment

For production use:
1. Use proper VM orchestration (not tmux)
2. Configure logging via ConsoleConfig::File
3. Set up proper networking (not shown in basic example)
4. Use snapshot/restore for faster startup
5. Configure resource limits and security

## Troubleshooting

**Error: "cloud-hypervisor not found"**
- Ensure binary is in PATH or project root
- Check with: `which cloud-hypervisor`

**Error: "Failed to connect to Unix socket"**
- Old socket file may exist: `rm /tmp/cloud-hypervisor-*.sock`
- Check Cloud Hypervisor is running: `ps aux | grep cloud-hypervisor`

**"Appears stuck" at GRUB menu**
- The VM has likely already booted past this screen
- The tmux buffer is showing old output
- **Solution**: Use `./examples/vm_console.sh view <session>` to see current output
- Or use `./examples/vm_console.sh login <session>` to auto-login
- Or manually scroll in tmux: Ctrl+b then ] then Shift+G then q

**Login fails or password echoes**
- This is a serial console quirk where input echoes unexpectedly
- **Solution**: Use the helper script which handles timing and buffering
- `./examples/vm_console.sh login vm_<uuid>`
- Avoid typing during boot (wait for login prompt)

**No output in tmux**
- Ensure VM config includes serial console (Mode::Tty)
- Wait a few seconds for boot to complete
- Check scrollback: `tmux capture-pane -t <session> -p -S -1000 | tail -30`

**VM won't boot**
- Verify images exist and are readable
- Check tmux session for error messages
- Ensure disk image is raw format (not qcow2)

## More Information

- **Full Documentation**: See `CLAUDE.md`
- **Examples**: See `examples/README.md`
- **API Reference**: Run `cargo doc --open`
- **Cloud Hypervisor Docs**: https://github.com/cloud-hypervisor/cloud-hypervisor

## Clean Up

Remove all VMs and temporary files:

```bash
# Kill all Cloud Hypervisor processes
pkill cloud-hypervisor

# Clean up sockets
rm /tmp/cloud-hypervisor*.sock

# Remove test images (optional)
rm -rf examples/images/
```

---

**Happy virtualizing!** 🚀