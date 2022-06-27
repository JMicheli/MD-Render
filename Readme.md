# MD Renderer

A from-scratch engine using the Vulkano crate to safely interact with the Vulkan API.

## Running the example

After cloning the repository, enter the new directory and run the example using:

`cargo run`

## About the example

The example should display a simple scene containing a red, high-poly version of the [Blender monkey mesh](https://docs.blender.org/manual/en/latest/modeling/meshes/primitives.html#monkey) (her name is Suzanne), a green icosphere, and a grey ground plane.

At present, the scene uses basic directional lighting and diffuse-only shading, and provides basic spherical camera controls. The camera can be rotated left/right and pitched up/down using the respective arrow keys.
