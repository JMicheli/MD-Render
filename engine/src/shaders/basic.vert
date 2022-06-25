#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec4 color;

layout(location = 0) out vec4 v_color;
layout(location = 1) out vec3 v_normal;

layout(set = 0, binding = 0) uniform WorldUniformData {
    mat4 world;
    mat4 view;
    mat4 proj;
} world_data;

layout(set = 1, binding = 0) uniform TransformUniformData {
  mat4 transformation_matrix;
} transform_data;

void main() {
  mat4 model_view = world_data.view * world_data.world * transform_data.transformation_matrix;

  v_normal = normalize(mat3(model_view) * normal);
  gl_Position = world_data.proj * model_view * vec4(position, 1.0);
  v_color = color;
}
