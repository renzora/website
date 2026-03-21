# State Replication

Synchronize game state between the server and all connected clients.

## What gets replicated

By default, the server replicates:

- **Transform** — position, rotation, scale
- **Physics state** — velocity, angular velocity
- **Animation state** — current clip, playback position
- **Custom replicated components** — any data you mark for sync

Clients receive updates every server tick and interpolate between them for smooth visuals.

## Ownership model

Every networked entity has an **owner**:

| Owner | Behavior |
|-------|----------|
| **Server** | Server controls the entity. Clients receive replicated state. Default for NPCs, world objects. |
| **Client** | A specific client controls the entity. Client sends inputs, server validates and replicates to others. Default for player characters. |

```rhai
fn on_update() {
    if is_owner() {
        // Only the owner runs movement logic
        position_x += input_x * speed * delta;
        position_z += input_y * speed * delta;
    }
}
```

## Marking components for replication

In the Inspector, toggle **Replicate** on any component to sync it across the network.

From code (developer guide):

```rust
app.replicate::<Health>();
app.replicate::<Score>();
app.replicate::<Inventory>();
```

Only replicated components are sent over the network. Keep replication minimal — only sync what other clients need to see.

## RPCs (Remote Procedure Calls)

Send one-off messages between clients and server.

### Client → Server

```rhai
fn on_update() {
    if is_key_just_pressed("E") {
        // Ask the server to use an item
        rpc_send("use_item", "health_potion");
    }
}
```

### Server → Client(s)

```rhai
fn on_rpc_use_item(item_name) {
    // Server validates and processes the request
    if is_server() {
        if inventory_has(item_name) {
            health += 50;
            rpc_send("show_effect", "heal");  // tell all clients to show VFX
        }
    }
}

fn on_rpc_show_effect(effect_name) {
    // All clients play the visual effect
    if effect_name == "heal" {
        play_sound("self", "sounds/heal.ogg");
    }
}
```

### RPC modes

| Mode | Description |
|------|-------------|
| `rpc_send(name, ...)` | Send to server (if client) or broadcast to all clients (if server) |
| `rpc_send_to(target_id, name, ...)` | Send to a specific client |
| `rpc_send_except(exclude_id, name, ...)` | Send to all clients except one |

## Network variables

For data that changes frequently and needs continuous sync (not one-off messages):

```rhai
// These variables are automatically synced to all clients
fn props() {
    #{
        health: #{ default: 100.0, replicate: true },
        team: #{ default: "none", replicate: true },
        score: #{ default: 0, replicate: true }
    }
}
```

When a replicated property changes on the owner, the new value is sent to all clients automatically.

## Interpolation

Clients render entities between the two most recent server snapshots. This hides network jitter and makes movement appear smooth.

- **Interpolation delay** — clients render ~2 ticks behind the server (configurable)
- **Extrapolation** — if a packet is late, clients predict forward briefly

The engine handles interpolation automatically for Transform and Physics components. Custom components use latest-value (no interpolation) by default.

## Client-side prediction

For the local player, waiting for the server creates input lag. Prediction solves this:

1. Client applies input locally (instant feedback)
2. Client sends input to server
3. Server validates and sends back authoritative state
4. Client corrects if its prediction was wrong (reconciliation)

Prediction is enabled by default for client-owned entities. The correction is smoothed to avoid visual snapping.

## Lag compensation

For hit detection in fast-paced games, the server rewinds time:

1. Client fires a shot at what they see (which is slightly in the past due to latency)
2. Server receives the shot with a timestamp
3. Server rewinds entity positions to that timestamp
4. Server checks if the shot hit
5. Server applies damage based on the rewound state

This is automatic for raycast-based attacks. Configure in `server.toml`:

```toml
[lag_compensation]
enabled = true
max_rewind_ms = 200  # maximum rewind window
```

## Bandwidth optimization

- **Delta compression** — only changed values are sent, not full snapshots
- **Priority** — nearby entities update more frequently than distant ones
- **Relevancy** — entities beyond a distance threshold are not replicated at all
- **Quantization** — positions are compressed to reduce packet size

### Monitoring bandwidth

In the editor: **Window → Network Stats** shows:

- Bytes sent/received per second
- Packet loss percentage
- Round-trip time (ping)
- Entity count and replication overhead

## Tips

- **Replicate as little as possible** — visual-only effects (particles, decals) don't need replication
- **Use RPCs for events**, replicated variables for continuous state
- **Test with 100+ ms latency** — use the built-in network simulator
- **Server authority is non-negotiable** — the server always has the final say
