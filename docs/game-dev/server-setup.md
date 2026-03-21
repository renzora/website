# Server Setup

Run a dedicated server for multiplayer games.

## Architecture

Renzora uses a **client-server model**. One authoritative server runs the game simulation, and clients connect to it. The server:

- Owns the game state (positions, health, scores)
- Validates client inputs
- Replicates state to all connected clients
- Runs server-side scripts

## Creating a server build

1. Open **Project → Export → Server**
2. Choose platform (Linux recommended for hosting)
3. The export produces a headless binary — no rendering, just simulation

## Server configuration

Create a `server.toml` in your project root:

```toml
[server]
port = 7777
max_players = 32
tick_rate = 60          # simulation updates per second
name = "My Game Server"
password = ""           # empty = no password

[network]
timeout_seconds = 30
max_packet_size = 1400  # bytes, MTU-safe default

[anti_cheat]
enabled = true
speed_check = true      # reject impossibly fast movement
teleport_threshold = 10 # max units per tick before flagging
```

## Running locally

For development, run the server and client on the same machine:

1. **Editor → Play → Host** — starts a local server and connects as a client
2. Or run the server binary separately: `./my_game --server --port 7777`
3. Connect from the editor: **Play → Connect → localhost:7777**

## Server script hooks

Server-side scripts can respond to player events:

```rhai
fn on_player_connect(player_id, player_name) {
    print(player_name + " joined the game");
    rpc_send("chat_message", "Server", player_name + " joined!");
}

fn on_player_disconnect(player_id, player_name) {
    print(player_name + " left the game");
    // Clean up player data, save progress, etc.
}

fn on_server_tick() {
    // Runs every server tick (60 times per second by default)
    // Game mode logic: check win conditions, spawn waves, etc.
}
```

## Server console commands

When running the server binary, these commands are available in the terminal:

| Command | Description |
|---------|-------------|
| `status` | Show connected players and server stats |
| `kick <player>` | Disconnect a player |
| `ban <player>` | Permanently ban a player |
| `say <message>` | Broadcast a chat message |
| `save` | Force-save world state |
| `shutdown` | Gracefully stop the server |
| `set tickrate <n>` | Change tick rate at runtime |
| `set maxplayers <n>` | Change max player count |

## Hosting options

### Self-hosted

Run the server binary on any machine with a public IP or port forwarding.

- **Linux VPS** (recommended) — DigitalOcean, Linode, Hetzner, AWS EC2
- **Windows Server** — works but Linux is lighter
- **Home server** — requires port forwarding (port 7777 UDP)

### Cloud hosting

- **Docker** — a `Dockerfile` is generated with the server export
- **Kubernetes** — scale multiple server instances behind a load balancer

### Renzora Hosting (coming soon)

One-click hosting from the Renzora dashboard. Auto-scaling, DDoS protection, global regions.

## Authentication

Clients authenticate when connecting:

1. **Token-based** — clients send a session token from your auth system
2. **Password** — server requires a password (set in `server.toml`)
3. **Open** — no authentication (for development or LAN)

## Tips

- **Tick rate 60** is good for action games. Turn-based or slower games can use 20–30
- **Always validate on the server** — never trust client data
- **Log everything** — server logs are your primary debugging tool for multiplayer issues
- **Test with simulated latency** — Editor → Network → Simulate Lag (50–200ms)
