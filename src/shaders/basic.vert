#version 450

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_tex_coord;

layout(location = 0) out vec3 v_position;
layout(location = 1) out vec3 v_normal;
layout(location = 2) out vec2 v_uv;

layout(set = 0, binding = 0) uniform CameraUniformData {
  vec3 position;

  mat4 view;
  mat4 proj;
} camera;

layout(set = 1, binding = 0) uniform MaterialUniformData {
  float shininess;
} material;
layout(set = 1, binding = 1) uniform sampler2D albedo;

layout(push_constant) uniform ObjectPushConstants
{
	mat4 transformation_matrix;
} object;


void main() {
  vec4 world_position =  object.transformation_matrix * vec4(a_position, 1.0);

  // TODO fix to use transpose inverse
  v_normal = normalize(mat3(camera.view) * a_normal);
  
  v_position = world_position.xyz;
  gl_Position = camera.proj * camera.view * world_position;

  v_uv = a_tex_coord;
}
