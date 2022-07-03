#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

layout(location = 0) out vec4 v_position;
layout(location = 1) out vec3 v_normal;

layout(set = 0, binding = 0) uniform CameraUniformData {
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


void main() {
  v_normal = normalize(mat3(camera.view) * normal);
  v_position =  object.transformation_matrix * vec4(position, 1.0);
  gl_Position = camera.proj * camera.view * v_position;
}
