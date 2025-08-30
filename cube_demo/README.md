# 3D Cube Demo with Diffuse Lighting

This is a simple 3D cube renderer using raycasting techniques with diffuse lighting effects. The cube rotates automatically and can be viewed from different angles.

## Controls

- **W/S**: Move camera forward/backward
- **A/D**: Rotate camera left/right
- **ESC**: Exit the application

## Features

- 3D cube rendering with perspective projection
- Diffuse lighting with a dynamic directional light source that changes with cube rotation
- Backface culling for performance optimization
- Filled polygon rendering
- Automatic cube rotation
- Camera movement and rotation
- FPS counter

## Implementation Details

This demo implements a simple 3D renderer from scratch using the following techniques:

1. **Vector Math**: Basic 3D vector operations for positions, directions, and lighting calculations
2. **Perspective Projection**: Converting 3D coordinates to 2D screen space
3. **Polygon Rendering**: Drawing filled polygons using a point-in-polygon algorithm
4. **Diffuse Lighting**: Calculating light intensity based on surface normals
5. **Backface Culling**: Skipping faces that aren't visible to the camera

## Running the Demo

To run the demo, navigate to the cube_demo directory and execute:

```
cargo run
```