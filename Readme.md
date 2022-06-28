# MD Renderer

A from-scratch engine using the Vulkano crate to safely interact with the Vulkan API.

## Running the example

Download the appropriate example for your platform from the [releases page](https://github.com/JMicheli/MD-Render/releases).

Unzip the folder and run the binary inside to view the example.

### About the example

The example should display a simple scene containing a red, high-poly version of the [Blender monkey mesh](https://docs.blender.org/manual/en/latest/modeling/meshes/primitives.html#monkey) (her name is Suzanne), a green icosphere, and a grey ground plane.

At present, the scene uses basic directional lighting and diffuse-only shading, and provides basic spherical camera controls. The camera can be rotated left/right and pitched up/down using the respective arrow keys.

## Compiling from source

Requires a working Rust installation with cargo.

Clone the repository, recursing over submodules.

`git clone --recurse-submodules https://github.com/JMicheli/MD-Render.git`

Navigate into the directory and run the `basic` example with cargo. This will cause cargo to download the dependencies and build the project.

`cargo run --example basic`

© Joseph W. Micheli 2022, all rights reserved. See `license.txt` for further information.
