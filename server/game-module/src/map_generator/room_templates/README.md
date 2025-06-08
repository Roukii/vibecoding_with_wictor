# Room Templates

This directory contains the room template system for dungeon generation. Instead of generating rooms procedurally, rooms are now created from predefined templates with ASCII art-style layouts.

## Template Format

Room templates use the following characters:

- `#` = Wall
- `.` = Floor
- `C` = Connection Point (wall that can become a door when connecting to other rooms)
- `D` = Door (pre-placed door)

## Template Structure

Each `RoomTemplate` has the following properties:

```rust
pub struct RoomTemplate {
    pub name: &'static str,        // Unique identifier
    pub weight: u32,               // Weight for random selection (higher = more likely)
    pub template: &'static str,    // ASCII art layout
    pub connection_points: Vec<Position>, // Relative positions where doors can be placed
    pub is_central: bool,          // Whether this is a central room template
}
```

## Current Templates

### Basic Rooms (`basic_rooms.rs`)

1. **Small Square** (weight: 10)

   - Simple 5x5 room with connection points on each side
   - Good for basic chambers

2. **Large Hall** (weight: 5)

   - Rectangular 10x5 room with multiple connection points
   - Good for main corridors or gathering areas

3. **L-Shaped** (weight: 7)

   - Irregular L-shaped room
   - Creates interesting layout variations

4. **Central Chamber** (weight: 3, central: true)

   - Large 11x9 room with many connection points
   - Designed for central hub areas

5. **Narrow Corridor** (weight: 8)

   - 8x3 corridor perfect for connecting rooms
   - High weight since corridors are often needed

6. **Cross-Shaped** (weight: 4)
   - Plus-shaped room with 4 arms
   - Creates interesting junction points

## Using the Room System

### RoomManager

The `RoomManager` handles loading and managing all room templates:

```rust
use crate::dungeon_generation::{RoomManager, Room};
use spacetimedb::rand::{rngs::StdRng, SeedableRng};

let room_manager = RoomManager::new();
let mut rng = StdRng::seed_from_u64(42);

// Create a specific room
let room = Room::from_template_name(&room_manager, "small_square", 10, 10, &mut rng)?;

// Create a random room
let room = Room::random_from_templates(&room_manager, 20, 20, false, &mut rng)?;

// Create a random central room
let central_room = Room::random_from_templates(&room_manager, 30, 30, true, &mut rng)?;
```

### Integration with Existing Code

The new system maintains backward compatibility. The old `Room::new()` method still works for procedural generation, while the new template-based methods are available as alternatives.

## Adding New Templates

To add new room templates:

1. Create a new constant in `basic_rooms.rs` (or create a new file)
2. Define the template string with proper ASCII art
3. Specify connection points as relative coordinates
4. Set appropriate weight and central room flag
5. Add the template to the `ALL_TEMPLATES` array

Example:

```rust
pub const MY_ROOM: RoomTemplate = RoomTemplate {
    name: "my_room",
    weight: 6,
    is_central: false,
    connection_points: vec![
        Position { x: 3, y: 0 },    // Top
        Position { x: 0, y: 2 },    // Left
    ],
    template: "
#######
#.....#
C.....C
#.....#
#######",
};
```

## Connection System

Connection points (`C` in templates) mark where doors can be placed when connecting rooms. The dungeon generator will:

1. Identify connection points between adjacent rooms
2. Create doors at matching connection points
3. Generate corridors between non-adjacent rooms if needed

The connection system is flexible and handles various room sizes and shapes automatically.

## Weighted Selection

Rooms are selected randomly based on their weights. Higher weight means higher probability of being chosen. This allows fine-tuning of room distribution:

- Common rooms (corridors, small chambers): weight 8-10
- Standard rooms: weight 5-7
- Special rooms: weight 3-4
- Rare rooms: weight 1-2

## Examples

See `template_example.rs` for complete examples of how to use the room template system, including a new `TemplateMapGenerator` that demonstrates integration with the existing dungeon generation pipeline.
