# MD Renderer

A from-scratch engine using the Vulkano crate to safely interact with the Vulkan API.

## Running the example

After cloning the repository, enter the new directory and run the example using:

`cargo run`

## About the example

Presently, the example should display a simple scene containing a red, high-poly version of the [Blender monkey mesh](https://docs.blender.org/manual/en/latest/modeling/meshes/primitives.html#monkey) (her name is Suzanne), a green icosphere, and a grey ground plane.

At present, the scene uses basic directional lighting and diffuse-only shading.
