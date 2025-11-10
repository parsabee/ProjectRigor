# ProjectRigor

A high-performance robotics and animation engine in Rust for Apple Silicon, featuring Metal rendering, physics simulation, and Entity Component System architecture.

This project builds a comprehensive **robotics + animation engine** in Rust, targeting **Apple Silicon**, leveraging Metal for rendering.

## Current Status

✅ **Phong Lighting Complete**
- Vertex normals for cubes and spheres (proper outward-facing normals per face)
- Phong shading model (ambient + diffuse + specular components)
- Directional light with configurable direction, color, and intensities
- Blinn-Phong specular highlights with adjustable shininess
- Light uniforms passed to fragment shader
- 5 unit tests for lighting (normals, light properties, orthogonality)

✅ **Camera Controls Complete**
- WASD + QE keyboard movement (forward/backward, strafe left/right, up/down)
- Mouse click-and-drag rotation (horizontal yaw, vertical pitch)
- Smooth camera rotation with configurable sensitivity
- Pitch clamping to prevent gimbal lock
- 6 unit tests for camera rotation (horizontal, vertical, clamping, distance preservation)

✅ **Sphere Rendering Complete**
- UV sphere mesh generation with configurable segments/rings
- Mixed cube and sphere rendering in single scene
- Shape-based vertex buffer selection (RenderShape enum)
- Physics integration with sphere colliders
- 5 additional unit tests for sphere rendering

✅ **ECS Architecture Complete**
- Full Entity Component System integration with `hecs`
- Component types: TransformComponent, PhysicsBodyComponent, RenderComponent
- Tag components for filtering: StaticTag, DynamicTag
- Query system for efficient entity iteration
- Complete separation of concerns: rendering, physics, and game logic
- 9 comprehensive ECS unit tests

✅ **Physics-Rendering Integration Complete**
- Real-time physics simulation with 8 dynamic objects + static ground plane
- ECS-driven rendering with per-entity colors and shapes from RenderComponent
- Automatic transform updates for dynamic entities only
- Collision detection and gravity working seamlessly with visuals
- Physics bodies properly synchronized with ECS transforms
- Support for both box and sphere colliders

✅ **Rendering System Complete**
- 3D perspective camera with view/projection matrices
- Cube and sphere geometry rendering with normals
- Shape-based rendering system (RenderShape::Cube, RenderShape::Sphere)
- Per-entity color system driven by ECS components
- Metal compute pipeline with runtime shader compilation
- Vertex attributes (position, normal, color) and uniforms system
- Efficient multi-object rendering with per-object colored vertex buffers
- Phong lighting with ambient, diffuse, and specular components
- 23 comprehensive renderer unit tests

✅ **Foundation Complete**
- Window management with `winit`
- Metal rendering backend with depth buffer
- Modular architecture (lib + bin structure)
- Core math module with `glam` (transforms, vectors, quaternions) - 6 unit tests
- Physics simulation with `rapier3d` (rigid bodies, collisions, gravity) - 5 unit tests
- Camera system with perspective projection and rotation - 16 unit tests
- **Total: 58 passing unit tests**

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
* [x] Vertex descriptor with position, normal, and color attributes
* [x] Camera system with perspective projection
* [x] Depth buffer and depth testing
* [x] Model-View-Projection matrix pipeline
* [x] Uniforms buffer for passing matrices to shaders
* [x] Full 3D cube rendering with 6 colored faces and normals
* [x] Multi-object rendering with ECS-driven colors
* [x] Per-entity colored vertex buffers
* [x] Render method accepting (Transform, color, shape) tuples from ECS
* [x] Sphere geometry rendering with UV sphere algorithm and normals
* [x] Shape-based rendering system (Cube and Sphere support)
* [x] Phong lighting model (ambient + diffuse + specular)
* [x] Directional light with configurable properties
* [x] Light uniforms buffer for shader lighting calculations
* [x] 23 comprehensive unit tests for renderer components
* [ ] Visual debug for forces, joint limits, and sensor rays
* [ ] Wireframe and debug rendering modes
* [ ] Instanced rendering for performance optimization
* [ ] Shadow mapping for realistic shadows
* [ ] Advanced materials system (PBR, metallic/roughness)
* [ ] Additional geometry types (cylinder, capsule, etc.)

### 8. Sensor / Perception Module

* [ ] Implement virtual cameras, LiDAR, and depth sensors
* [ ] Perform raycasting and collision queries with `rapier`
* [ ] Optional ML perception integration

### 9. Event / Input Module ✅ **COMPLETED**

* [x] Integrated `winit` for windowing and input
* [x] Basic event handling (close, resize, redraw)
* [x] Keyboard input tracking with WASD + QE movement
* [x] Mouse button state tracking
* [x] Mouse drag detection and delta calculation
* [x] Camera rotation with mouse click-and-drag
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
8. ✅ **Sphere Rendering** - UV sphere geometry with shape-based rendering
9. ✅ **Camera Controls** - WASD movement + mouse look for interactive navigation
10. ✅ **Phong Lighting** - Vertex normals, ambient/diffuse/specular, directional light
11. **Skeleton & FK/IK layer** - Articulated structures with joints - **NEXT**
12. Control system (PID, trajectory following)
13. Enhanced rendering (instancing, materials, shadows)
14. Motion / Animation integration (keyframes, splines)
15. Sensors & perception (virtual cameras, raycasting)
16. Optional AI / planning

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
├── app.rs          # Application state with ECS World, keyboard/mouse input
├── camera.rs       # Camera system: view/projection matrices, movement, rotation (16 tests)
├── ecs.rs          # ECS components and tags (9 tests)
├── math.rs         # Math module: Transform, vectors, quaternions (6 tests)
├── physics.rs      # Physics module: PhysicsWorld, rigid bodies, colliders (5 tests)
└── renderer.rs     # Metal rendering: shaders, pipeline, normals, lighting (23 tests)
shaders/
└── cube.metal      # Metal shader code with Phong lighting (vertex + fragment)
examples/
├── math_test.rs    # Interactive math module demonstration
└── physics_test.rs # Physics simulation demonstration

Total: 58 unit tests across all modules
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
# Run the main application (shows 5 cubes + 3 spheres falling with physics)
cargo run

# Controls:
# - W/S: Move camera forward/backward
# - A/D: Strafe camera left/right  
# - Q/E: Move camera down/up
# - Left click + drag: Rotate camera (yaw/pitch)

# Run the math test example
cargo run --example math_test

# Run the physics simulation example
cargo run --example physics_test
```

## What's Working Now

- **Phong Lighting**: Realistic lighting with ambient, diffuse, and specular components; directional light from upper left
- **Normal-Based Shading**: Proper vertex normals on cubes (per-face) and spheres (radial); lighting responds to surface orientation
- **Interactive Camera**: Full WASD + QE movement and mouse click-and-drag rotation with pitch clamping
- **Sphere Rendering**: UV sphere mesh generation with 20 segments/rings, smooth geometry with accurate normals
- **Shape-Based Rendering**: RenderShape enum supporting Cube and Sphere with automatic vertex buffer selection
- **Mixed Geometry Scene**: Cubes and spheres rendered in same scene with proper physics and lighting
- **ECS Architecture**: Complete Entity Component System with hecs managing all game objects
- **Component-Based Design**: TransformComponent, PhysicsBodyComponent, RenderComponent with color and shape
- **Entity Filtering**: StaticTag and DynamicTag for separating static/dynamic objects
- **Physics-ECS Integration**: Physics updates only dynamic entities, static ground remains fixed
- **Rendering-ECS Integration**: Renderer reads transforms, colors, and shapes directly from ECS components
- **Multi-Object Scene**: 1 static gray ground plane + 5 colored cubes + 3 colored spheres with realistic lighting
- **Real-Time Simulation**: Physics steps at 60Hz, updates ECS, extracts render data each frame
- **3D Rendering**: Fully functional Metal-based renderer with depth testing and Phong shading
- **Camera System**: Perspective projection with configurable FOV, movement, and rotation
- **Physics**: Complete rapier3d integration with gravity, collisions, box and sphere colliders
- **Math**: Transform types with glam for vectors, quaternions, and matrices
- **Comprehensive Testing**: 58 unit tests covering all core systems including lighting

## Next Steps

### 🤖 Major Feature: Robot Skeleton Module (5-10 hours) - **RECOMMENDED NEXT**
**Start the main robotics work:**
- Define joint types (revolute/hinge, spherical, prismatic)
- Create bone/link structures
- Implement Forward Kinematics (FK) - compute positions from joint angles
- Implement Inverse Kinematics (IK) - solve for joint angles from target position
- Connect to rapier3d with joint constraints
- Add joint limits and actuator interfaces
- Build simple 2-3 link arm as proof of concept

**Benefits:**
- Core robotics functionality
- Visual feedback with existing camera controls
- Foundation for animation and control systems
- Can test with mouse interaction (click to set IK target)

### 🎨 Enhancement: Rendering Improvements (3-5 hours)
**Can be done in parallel or after skeleton:**
- Instanced rendering for performance with many objects
- Basic Phong lighting (directional light + ambient)
- Material system (metallic, roughness properties)
- Wireframe debug mode for skeleton visualization
- Additional geometry (cylinder for bones, capsule for links)

### 🎮 Quality of Life: Additional Features
- **Enhanced Camera** - Scroll wheel zoom, camera smoothing/damping, orbit mode
- **Control Systems** - PID controllers for smooth joint movement
- **Animation System** - Keyframe interpolation for pre-programmed motions
- **Sensors** - Raycasting, virtual cameras for perception
- **UI/Debug Display** - ImGui integration for parameter tweaking
- **Entity Management** - Runtime spawning/despawning, save/load scenes

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
