#!/bin/bash
set -e

# VM Console Helper Script
# This script helps you properly interact with the VM serial console via tmux

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

show_usage() {
    echo "VM Console Helper"
    echo ""
    echo "Usage:"
    echo "  $0 list                    - List all running VM sessions"
    echo "  $0 attach <vm-uuid>        - Attach to a VM console"
    echo "  $0 login <vm-uuid>         - Auto-login to VM (cirros/gocubsgo)"
    echo "  $0 exec <vm-uuid> <cmd>    - Execute command in VM"
    echo "  $0 view <vm-uuid>          - View VM output without attaching"
    echo "  $0 kill <vm-uuid>          - Kill VM session"
    echo "  $0 killall                 - Kill all VM sessions"
    echo ""
    echo "Examples:"
    echo "  $0 list"
    echo "  $0 attach vm_abc123..."
    echo "  $0 login vm_abc123..."
    echo "  $0 exec vm_abc123... 'uname -a'"
    echo "  $0 view vm_abc123..."
}

list_vms() {
    echo "Running VM sessions:"
    tmux list-sessions 2>/dev/null | grep "^vm_" || echo "  (none)"
}

attach_vm() {
    local session=$1
    if [ -z "$session" ]; then
        echo "Error: No session specified"
        echo ""
        list_vms
        exit 1
    fi

    echo "Attaching to $session..."
    echo "Tip: The login prompt may be scrolled up. Press Ctrl+b then ] then Shift+G to scroll to bottom"
    echo "     To detach: Press Ctrl+b then d"
    echo ""
    sleep 2
    tmux attach -t "$session"
}

view_output() {
    local session=$1
    if [ -z "$session" ]; then
        echo "Error: No session specified"
        exit 1
    fi

    echo "=== Last 30 lines of VM output ==="
    tmux capture-pane -t "$session" -p -S -1000 | tail -30
}

auto_login() {
    local session=$1
    if [ -z "$session" ]; then
        echo "Error: No session specified"
        exit 1
    fi

    echo "Attempting auto-login to $session..."
    echo ""

    # Clear any buffered input
    tmux send-keys -t "$session" C-u C-c 2>/dev/null || true
    sleep 1

    # Send a newline to get fresh prompt
    tmux send-keys -t "$session" "" Enter
    sleep 2

    # Check current state
    local output=$(tmux capture-pane -t "$session" -p | tail -5)

    if echo "$output" | grep -q "login:"; then
        echo "Found login prompt, entering credentials..."

        # Type username character by character with small delays
        for char in c i r r o s; do
            tmux send-keys -t "$session" "$char"
            sleep 0.1
        done
        tmux send-keys -t "$session" Enter
        sleep 2

        # Type password character by character
        for char in g o c u b s g o; do
            tmux send-keys -t "$session" "$char"
            sleep 0.1
        done
        tmux send-keys -t "$session" Enter
        sleep 2

        # Check if logged in
        local result=$(tmux capture-pane -t "$session" -p | tail -3)
        if echo "$result" | grep -q '\$'; then
            echo ""
            echo "✓ Successfully logged in!"
            echo ""
            echo "Current shell prompt:"
            tmux capture-pane -t "$session" -p | tail -3
            echo ""
            echo "You can now:"
            echo "  - Run: $0 exec $session '<command>'"
            echo "  - Or attach: $0 attach $session"
        else
            echo ""
            echo "⚠ Login may have failed. Current output:"
            tmux capture-pane -t "$session" -p | tail -10
        fi
    else
        echo "Not at login prompt. Current output:"
        echo "$output"
    fi
}

exec_command() {
    local session=$1
    local cmd=$2

    if [ -z "$session" ] || [ -z "$cmd" ]; then
        echo "Error: Session and command required"
        echo "Usage: $0 exec <session> '<command>'"
        exit 1
    fi

    echo "Executing in $session: $cmd"

    # Send command
    tmux send-keys -t "$session" "$cmd" Enter
    sleep 2

    # Show output
    echo ""
    echo "=== Output ==="
    tmux capture-pane -t "$session" -p | tail -15
}

kill_vm() {
    local session=$1
    if [ -z "$session" ]; then
        echo "Error: No session specified"
        exit 1
    fi

    echo "Killing session: $session"
    tmux kill-session -t "$session" 2>/dev/null || echo "Session not found"

    # Clean up socket
    local socket_pattern="/tmp/cloud-hypervisor*.sock"
    if ls $socket_pattern 1> /dev/null 2>&1; then
        echo "Cleaning up sockets..."
        rm -f $socket_pattern
    fi
}

kill_all_vms() {
    echo "Killing all VM sessions..."
    tmux list-sessions 2>/dev/null | grep "^vm_" | cut -d: -f1 | while read session; do
        echo "  Killing $session"
        tmux kill-session -t "$session" 2>/dev/null || true
    done

    echo "Cleaning up sockets..."
    rm -f /tmp/cloud-hypervisor*.sock

    echo "Done!"
}

# Main script
case "${1:-}" in
    list)
        list_vms
        ;;
    attach)
        attach_vm "$2"
        ;;
    login)
        auto_login "$2"
        ;;
    exec)
        exec_command "$2" "$3"
        ;;
    view)
        view_output "$2"
        ;;
    kill)
        kill_vm "$2"
        ;;
    killall)
        kill_all_vms
        ;;
    ""|help|-h|--help)
        show_usage
        ;;
    *)
        echo "Unknown command: $1"
        echo ""
        show_usage
        exit 1
        ;;
esac
