# Custom Blueprint Nodes

Extend the visual blueprint system with your own nodes.

## Blueprint architecture

The blueprint system is a **node graph interpreter**. Each frame, the engine:

1. Finds all entities with a `BlueprintComponent`
2. Fires event nodes (On Update, On Collision, etc.)
3. Walks the execution graph, evaluating each node
4. Collects `ScriptCommand`s and executes them

Nodes are either **execution nodes** (have flow pins, produce side effects) or **pure nodes** (stateless math/logic, evaluated on demand).

## The BlueprintNode trait

```rust
use renzora_blueprints::{BlueprintNode, NodeInput, NodeOutput, NodeContext, NodeResult, PinType};

pub trait BlueprintNode: Send + Sync {
    /// Display name in the node library
    fn name(&self) -> &'static str;

    /// Category for organization (Math, Physics, Audio, etc.)
    fn category(&self) -> &'static str;

    /// Input pin definitions
    fn inputs(&self) -> Vec<NodeInput>;

    /// Output pin definitions
    fn outputs(&self) -> Vec<NodeOutput>;

    /// Whether this node has execution flow pins (default: false = pure node)
    fn has_flow(&self) -> bool { false }

    /// Execute the node logic
    fn execute(&self, ctx: &mut NodeContext) -> NodeResult;

    /// Optional: tooltip description
    fn description(&self) -> &'static str { "" }
}
```

## Pin types

| PinType | Rust type | Color in editor |
|---------|-----------|----------------|
| `PinType::Float` | `f64` | Green |
| `PinType::Int` | `i64` | Cyan |
| `PinType::Bool` | `bool` | Red |
| `PinType::String` | `String` | Purple |
| `PinType::Vec3` | `(f64, f64, f64)` | Yellow |
| `PinType::Entity` | `Entity` | Blue |
| `PinType::Any` | dynamic | Gray |

## Creating a pure node

Pure nodes have no side effects and no flow pins:

```rust
pub struct LerpNode;

impl BlueprintNode for LerpNode {
    fn name(&self) -> &'static str { "Lerp" }
    fn category(&self) -> &'static str { "Math" }
    fn description(&self) -> &'static str { "Linear interpolation between A and B" }

    fn inputs(&self) -> Vec<NodeInput> {
        vec![
            NodeInput::new("A", PinType::Float).default(0.0),
            NodeInput::new("B", PinType::Float).default(1.0),
            NodeInput::new("T", PinType::Float).default(0.5),
        ]
    }

    fn outputs(&self) -> Vec<NodeOutput> {
        vec![NodeOutput::new("Result", PinType::Float)]
    }

    fn execute(&self, ctx: &mut NodeContext) -> NodeResult {
        let a = ctx.input_float("A");
        let b = ctx.input_float("B");
        let t = ctx.input_float("T").clamp(0.0, 1.0);
        ctx.set_output_float("Result", a + (b - a) * t);
        NodeResult::Done
    }
}
```

## Creating an execution node

Execution nodes have flow pins and produce side effects:

```rust
pub struct SpawnAtNode;

impl BlueprintNode for SpawnAtNode {
    fn name(&self) -> &'static str { "Spawn At Position" }
    fn category(&self) -> &'static str { "Entity" }
    fn has_flow(&self) -> bool { true }  // has execution pins

    fn inputs(&self) -> Vec<NodeInput> {
        vec![
            NodeInput::new("Name", PinType::String).default("Entity"),
            NodeInput::new("Position", PinType::Vec3),
        ]
    }

    fn outputs(&self) -> Vec<NodeOutput> {
        vec![NodeOutput::new("Spawned", PinType::Entity)]
    }

    fn execute(&self, ctx: &mut NodeContext) -> NodeResult {
        let name = ctx.input_string("Name");
        let pos = ctx.input_vec3("Position");

        ctx.push_command(ScriptCommand::SpawnEntity {
            name: name.clone(),
            position: pos,
        });

        // Output the entity reference (resolved after command execution)
        ctx.set_output_entity("Spawned", EntityRef::ByName(name));
        NodeResult::Continue  // flow continues to next node
    }
}
```

## NodeContext API

### Reading inputs

| Method | Returns | Description |
|--------|---------|-------------|
| `ctx.input_float(name)` | `f64` | Read a float input (or its default) |
| `ctx.input_int(name)` | `i64` | Read an integer input |
| `ctx.input_bool(name)` | `bool` | Read a boolean input |
| `ctx.input_string(name)` | `String` | Read a string input |
| `ctx.input_vec3(name)` | `(f64, f64, f64)` | Read a Vec3 input |
| `ctx.input_entity(name)` | `Option<Entity>` | Read an entity reference |

### Writing outputs

| Method | Description |
|--------|-------------|
| `ctx.set_output_float(name, value)` | Set a float output |
| `ctx.set_output_int(name, value)` | Set an integer output |
| `ctx.set_output_bool(name, value)` | Set a boolean output |
| `ctx.set_output_string(name, value)` | Set a string output |
| `ctx.set_output_vec3(name, x, y, z)` | Set a Vec3 output |
| `ctx.set_output_entity(name, entity)` | Set an entity output |

### World access

| Method | Description |
|--------|-------------|
| `ctx.entity()` | The entity running this blueprint |
| `ctx.delta()` | Frame delta time |
| `ctx.elapsed()` | Total elapsed time |
| `ctx.push_command(cmd)` | Queue a ScriptCommand |
| `ctx.get_variable(name)` | Read a blueprint variable |
| `ctx.set_variable(name, value)` | Write a blueprint variable |

## NodeResult

| Variant | Meaning |
|---------|---------|
| `NodeResult::Done` | Pure node completed (no flow) |
| `NodeResult::Continue` | Execution flows to the next connected node |
| `NodeResult::Branch(index)` | Flow to a specific output pin (for Branch/Switch) |
| `NodeResult::Halt` | Stop execution of this graph for this frame |

## Registering nodes

```rust
impl Plugin for MyBlueprintNodesPlugin {
    fn build(&self, app: &mut App) {
        app.register_blueprint_node::<LerpNode>()
           .register_blueprint_node::<SpawnAtNode>();
    }
}
```

## Node categories

Use standard categories for consistency:

| Category | For |
|----------|-----|
| `"Event"` | Entry points (On Ready, On Update, etc.) |
| `"Flow"` | Branch, Sequence, Loop, Delay |
| `"Math"` | Arithmetic, trigonometry, interpolation |
| `"Transform"` | Position, rotation, scale |
| `"Physics"` | Forces, raycasts, velocity |
| `"Entity"` | Spawn, destroy, find, components |
| `"Animation"` | Play, stop, blend, crossfade |
| `"Audio"` | Play sound, music, volume |
| `"UI"` | Show, hide, set text, progress |
| `"Material"` | Set color, property, swap |
| `"Variable"` | Get, set, local |
| `"Network"` | RPC, ownership, player info |

Custom categories appear as new sections in the node library.

## Testing custom nodes

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use renzora_blueprints::test::MockNodeContext;

    #[test]
    fn test_lerp_node() {
        let node = LerpNode;
        let mut ctx = MockNodeContext::new();
        ctx.set_input("A", 0.0);
        ctx.set_input("B", 10.0);
        ctx.set_input("T", 0.5);

        let result = node.execute(&mut ctx);
        assert_eq!(result, NodeResult::Done);
        assert_eq!(ctx.output_float("Result"), 5.0);
    }
}
```
