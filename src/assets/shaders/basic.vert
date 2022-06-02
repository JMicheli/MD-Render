#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec4 color;

layout(location = 0) out vec4 v_color;
layout(location = 1) out vec3 v_normal;

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

void main() {
  mat4 model_view = ubo.view * ubo.model;

  v_normal = transpose(inverse(mat3(model_view))) * normal;
  gl_Position = ubo.proj * model_view * vec4(position, 1.0);
  v_color = color;
}
