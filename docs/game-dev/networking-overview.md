# Multiplayer Overview

Build multiplayer games with dedicated server support.

## Architecture

Renzora uses a **dedicated server** model powered by [Lightyear](https://github.com/cBournhonesque/lightyear). The server runs the authoritative game simulation, and clients send input and receive state updates.

This approach:
- Prevents cheating (server validates everything)
- Ensures consistency (all players see the same state)
- Supports high player counts

## Transport Options

| Transport | Best For | Latency |
|-----------|----------|---------|
| **UDP** | Desktop games, lowest latency | ~1-5ms |
| **WebTransport** | Modern browsers | ~5-15ms |
| **WebSocket** | Maximum compatibility | ~10-30ms |

## Configuration

Network settings in `project.toml`:

```toml
[network]
server_addr = "127.0.0.1"
port = 7636
transport = "udp"
tick_rate = 64
max_clients = 32
```

- **server_addr** — IP address of the server
- **port** — network port (default 7636)
- **transport** — `"udp"`, `"websocket"`, or `"webtransport"`
- **tick_rate** — server simulation rate in Hz (higher = smoother but more bandwidth)
- **max_clients** — maximum simultaneous players

## State Replication

Mark components for network replication in the editor. The server automatically syncs their state to all connected clients. The engine handles:

- **Interpolation** — smooth movement between server updates
- **Client-side prediction** — immediate response to local input
- **Server reconciliation** — corrects client prediction errors

## Scripting API

```rhai
fn on_ready() {
    // Check connection status
    if net_is_connected() {
        print("Connected to server!");
        print("Client ID: " + net_get_client_id());
    }
}

fn on_update() {
    // Get connection state
    let state = net_get_connection_state();
    // Returns: "Disconnected", "Connecting", "Connected", or "Error"

    // Dynamic connect/disconnect
    if is_key_just_pressed("C") {
        net_connect("127.0.0.1", 7636);
    }
    if is_key_just_pressed("X") {
        net_disconnect();
    }
}
```

## Running a Dedicated Server

Export your game with the server option enabled. The export includes a separate server binary alongside the game client:

```bash
# Start the server
./my-game-server --port 7636

# Players connect via the game client
./my-game --connect 123.45.67.89:7636
```

## Tips

- Start with UDP for development (lowest latency)
- Use WebSocket for web builds
- Keep tick_rate at 64 for most games (matches physics rate)
- Test with artificial latency to catch prediction issues early
