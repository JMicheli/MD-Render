#version 450

// Inputs/Ouputs
////////////////
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;

layout(location = 0) out vec3 v_position;
layout(location = 1) out vec3 v_normal;

// Uniform buffer objects
/////////////////////////
layout(set = 0, binding = 0) uniform CameraUniformData {
  vec3 position;

  mat4 view;
  mat4 proj;
} camera;

layout(set = 1, binding = 0) uniform MaterialUniformData {
  vec3 diffuse_color;
  float alpha;

  vec3 specular_color;
  float shininess;
} material;

layout(push_constant) uniform ObjectPushConstants
{
	mat4 transformation_matrix;
} object;

// Shader Entry Point
/////////////////////
void main() {
  // Calculate world position of input vertex
  vec4 world_position =  object.transformation_matrix * vec4(a_position, 1.0);

  // Calculate surface normal at input vertex
  // TODO fix to use transpose inverse
  v_normal = normalize(mat3(camera.view) * a_normal);
  
  // Write output of vertex position
  v_position = world_position.xyz;
  gl_Position = camera.proj * camera.view * world_position;
}
