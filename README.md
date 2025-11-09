# ProjectRigor

A high-performance robotics and animation engine in Rust for Apple Silicon, featuring Metal rendering, physics simulation, and Entity Component System architecture.

This project builds a comprehensive **robotics + animation engine** in Rust, targeting **Apple Silicon**, leveraging Metal for rendering.

## Current Status

✅ **ECS Architecture Complete**
- Full Entity Component System integration with `hecs`
- Component types: TransformComponent, PhysicsBodyComponent, RenderComponent
- Tag components for filtering: StaticTag, DynamicTag
- Query system for efficient entity iteration
- Complete separation of concerns: rendering, physics, and game logic
- 9 comprehensive ECS unit tests

✅ **Physics-Rendering Integration Complete**
- Real-time physics simulation with 5 falling cubes + static ground plane
- ECS-driven rendering with per-entity colors from RenderComponent
- Automatic transform updates for dynamic entities only
- Collision detection and gravity working seamlessly with visuals
- Physics bodies properly synchronized with ECS transforms

✅ **Rendering System Complete**
- 3D perspective camera with view/projection matrices
- Full cube rendering with depth testing
- Per-entity color system driven by ECS components
- Metal compute pipeline with runtime shader compilation
- Vertex attributes and uniforms system
- Efficient multi-object rendering with per-object colored vertex buffers
- 13 comprehensive renderer unit tests

✅ **Foundation Complete**
- Window management with `winit`
- Metal rendering backend with depth buffer
- Modular architecture (lib + bin structure)
- Core math module with `glam` (transforms, vectors, quaternions) - 6 unit tests
- Physics simulation with `rapier3d` (rigid bodies, collisions, gravity) - 5 unit tests
- Camera system with perspective projection - 4 unit tests
- **Total: 37 passing unit tests**

## Project Goals

* Render everything efficiently and safely
* Implement a modular and extensible control system and motion planning library

## TODO / Modules to Implement

### 1. Core Math Module ✅ **COMPLETED**

* [x] Integrated `glam` for vectors, matrices, quaternions, and transforms
* [x] Created `Transform` type with position, rotation, scale
* [x] Point and vector transformation methods
* [x] Matrix conversion utilities
* [x] Comprehensive unit tests
* [ ] Add FK/IK helpers and constraint math utilities (future)

### 2. Physics / Rigid-Body Module ✅ **COMPLETED**

* [x] Integrated `rapier3d` for rigid-body physics
* [x] Created `PhysicsWorld` wrapper managing simulation
* [x] Dynamic and static rigid body creation
* [x] Box and sphere collider helpers
* [x] Transform integration (glam ↔ rapier conversion)
* [x] Velocity get/set methods
* [x] Comprehensive unit tests (gravity, collisions, velocities)
* [ ] Wrap physics objects for robot skeleton integration (future)
* [ ] Implement articulated rigid bodies and joint constraints (future)

### 3. Robot / Skeleton Module

* [ ] Define joints, bones, and FK/IK routines
* [ ] Connect `rapier` rigid bodies to joints
* [ ] Enforce joint limits and actuator interfaces

### 4. Control Module

* [ ] Implement PID controllers and trajectory following
* [ ] Integrate control loops with physics and joints

### 5. Motion / Animation Module

* [ ] Keyframe interpolation and spline trajectory support
* [ ] IK-driven animation
* [ ] Collision-aware motion planning

### 6. Scene / World Module ✅ **COMPLETED**

* [x] Entity management using `hecs` ECS
* [x] Component system: TransformComponent, PhysicsBodyComponent, RenderComponent
* [x] Tag components: StaticTag, DynamicTag for entity filtering
* [x] Query system for iterating entities by component types
* [x] Physics stepping synchronized with ECS transform updates
* [x] Rendering system reads from ECS components
* [x] Comprehensive unit tests for ECS functionality
* [ ] Add more component types (VelocityComponent, MassComponent, etc.)
* [ ] Implement entity spawning/despawning at runtime
* [ ] Add entity archetypes for common object types

### 7. Rendering Module ✅ **COMPLETED**

* [x] Integrated `metal-rs` for GPU rendering
* [x] Metal layer attachment and drawable management
* [x] Runtime shader compilation (vertex + fragment shaders)
* [x] Vertex descriptor with position and color attributes
* [x] Camera system with perspective projection
* [x] Depth buffer and depth testing
* [x] Model-View-Projection matrix pipeline
* [x] Uniforms buffer for passing matrices to shaders
* [x] Full 3D cube rendering with 6 colored faces
* [x] Multi-object rendering with ECS-driven colors
* [x] Per-entity colored vertex buffers
* [x] Render method accepting (Transform, color) tuples from ECS
* [x] 13 comprehensive unit tests for renderer components
* [ ] Visual debug for forces, joint limits, and sensor rays
* [ ] Wireframe and debug rendering modes
* [ ] Instanced rendering for performance optimization
* [ ] Sphere rendering (RenderShape::Sphere support)
* [ ] Material system with lighting

### 8. Sensor / Perception Module

* [ ] Implement virtual cameras, LiDAR, and depth sensors
* [ ] Perform raycasting and collision queries with `rapier`
* [ ] Optional ML perception integration

### 9. Event / Input Module

* [x] Integrated `winit` for windowing and input
* [x] Basic event handling (close, resize, redraw)
* [ ] Feed input to robots, adjust trajectories, and interact with simulations

### 10. AI / Planning Module (Optional)

* [ ] Task planning and pathfinding for autonomous agents
* [ ] Implement behavior trees or state machines
* [ ] Integrate `petgraph` for graph-based planning

## Roadmap / Suggested Implementation Order

1. ✅ **Core Math** (`glam`) - Transform types, vectors, quaternions
2. ✅ **Rendering Foundation** (`metal-rs`) - Window and basic rendering
3. ✅ **Input Handling** (`winit`) - Event loop and window management
4. ✅ **Physics** (`rapier3d`) - Rigid body simulation, collisions, gravity
5. ✅ **3D Camera System** - Perspective projection, view matrices, depth testing
6. ✅ **Physics-Rendering Integration** - Render rapier bodies dynamically with colors
7. ✅ **ECS Architecture** (`hecs`) - Entity-Component-System for object management
8. **Skeleton & FK/IK layer** - Articulated structures with joints - **NEXT**
9. Control system (PID, trajectory following)
10. Enhanced rendering (instancing, materials, lighting, sphere geometry)
11. Motion / Animation integration (keyframes, splines)
12. Sensors & perception (virtual cameras, raycasting)
13. User input and interactive controls
14. Optional AI / planning

---

## Notes

* Focus on building reusable, modular systems
* Ensure compatibility and performance on Apple Silicon
* Prioritize correctness in physics and control before adding advanced animation or AI features


## Structure

```
src/
├── lib.rs          # Library root, public API exports
├── main.rs         # Application entry point (minimal)
├── app.rs          # Application state with ECS World and event handling
├── camera.rs       # Camera system: view/projection matrices (4 tests)
├── ecs.rs          # ECS components and tags (9 tests)
├── math.rs         # Math module: Transform, vectors, quaternions (6 tests)
├── physics.rs      # Physics module: PhysicsWorld, rigid bodies, colliders (5 tests)
└── renderer.rs     # Metal rendering: shaders, pipeline, uniforms (13 tests)
shaders/
└── cube.metal      # Metal shader code (vertex + fragment)
examples/
├── math_test.rs    # Interactive math module demonstration
└── physics_test.rs # Physics simulation demonstration

Total: 37 unit tests across all modules
```

## Building

```bash
# Build in debug mode
cargo build

# Build optimized for release
cargo build --release

# Run tests
cargo test
```

## Running

```bash
# Run the main application (shows 5 colored cubes falling with physics)
cargo run

# Run the math test example
cargo run --example math_test

# Run the physics simulation example
cargo run --example physics_test
```

## What's Working Now

- **ECS Architecture**: Complete Entity Component System with hecs managing all game objects
- **Component-Based Design**: TransformComponent, PhysicsBodyComponent, RenderComponent with color
- **Entity Filtering**: StaticTag and DynamicTag for separating static/dynamic objects
- **Physics-ECS Integration**: Physics updates only dynamic entities, static ground remains fixed
- **Rendering-ECS Integration**: Renderer reads transforms and colors directly from ECS components
- **Multi-Object Scene**: 1 static gray ground plane (50x0.1x50) + 5 colored dynamic cubes
- **Real-Time Simulation**: Physics steps at 60Hz, updates ECS, extracts render data each frame
- **3D Rendering**: Fully functional Metal-based renderer with depth testing
- **Camera System**: Perspective projection from position (8, 4, 12) looking at scene center
- **Physics**: Complete rapier3d integration with gravity, collisions, and rigid bodies
- **Math**: Transform types with glam for vectors, quaternions, and matrices
- **Comprehensive Testing**: 37 unit tests covering all core systems

## Next Steps

1. **Robot Skeleton Module**: Build articulated structures with joints, bones, and FK/IK
2. **Joint Constraints**: Implement hinge, ball-socket, and prismatic joints with limits
3. **Control Systems**: PID controllers for robot actuators and trajectory following
4. **Enhanced Rendering**: Sphere geometry, instanced rendering, materials, and lighting
5. **User Input**: Keyboard/mouse controls to interact with physics objects and camera
6. **Animation System**: Keyframe interpolation and spline-based trajectories
7. **Sensors**: Virtual cameras, raycasting, and collision queries for perception

## Dependencies

- `winit` - Cross-platform windowing and input
- `metal` - Apple Metal graphics API bindings
- `glam` - High-performance 3D math library (SIMD-optimized)
- `rapier3d` - 3D physics engine for rigid body dynamics
- `hecs` - Fast and minimal Entity Component System
- `objc`, `cocoa` - Objective-C runtime and macOS frameworks
- `raw-window-handle`, `core-graphics-types` - Platform integration

## Requirements

- macOS with Apple Silicon
- Rust 1.70+
