#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec4 color;

layout(location = 0) out vec4 v_position;
layout(location = 1) out vec3 v_normal;
layout(location = 2) out vec4 v_color;

layout(set = 0, binding = 0) uniform CameraUniformData {
  vec3 camera_pos;

  mat4 view;
  mat4 proj;
} world_data;

layout(set = 1, binding = 0) uniform ObjectUniformData {
  vec3 diffuse_color;
  float alpha;

  vec3 specular_color;
  float shininess;

  mat4 transformation_matrix;
} object_data;

void main() {
  v_normal = normalize(mat3(world_data.view) * normal);
  v_position =  object_data.transformation_matrix * vec4(position, 1.0);
  v_color = vec4(object_data.diffuse_color, object_data.alpha);
  gl_Position = world_data.proj * world_data.view * v_position;
}
