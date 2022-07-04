#version 450

layout(location = 0) in vec4 v_position;
layout(location = 1) in vec3 v_normal;

layout(location = 0) out vec4 f_color;

// Create a light for testing
// TODO Externalize
struct PositionalLight {
  vec3 diffuse_color;
  vec3 specular_color;
  vec3 position;
};
const vec3 AMBIENT_LIGHT = vec3(1.0, 1.0, 1.0);
const PositionalLight LIGHT = PositionalLight(
  vec3(1.0, 1.0, 1.0),
  vec3(1.0, 1.0, 1.0),
  vec3(2.0, -2.0, 2.0)
);
// Test constants not included in material
// TODO Externalize?
const float AMBIENT_REFLECTANCE = 0.2;
const float DIFFUSE_REFLECTANCE = 0.6;
const float SPECULAR_REFLECTANCE = 0.2;

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

void main() {
  // Camera position can be obtained from view matrix
  vec3 camera_position = -camera.view[3].xyz;

  // L - Direction from fragment position to light
  vec3 L = normalize(LIGHT.position - v_position.xyz);
  // N - (Normalized) fragment normal
  vec3 N = normalize(v_normal);
  // V - Direction from fragment position to camera
  vec3 V = normalize(camera_position - v_normal.xyz);
  // R - Direction of a ray reflected off the surface
  vec3 R = normalize(reflect(-L, N));

  // Phong reflection model
  // Ambient contribution from lights
  vec3 ambient = AMBIENT_REFLECTANCE * AMBIENT_LIGHT;
  // Diffuse contribution
  vec3 diffuse = DIFFUSE_REFLECTANCE * dot(L, N) * material.diffuse_color;
  // Specular contribution
  vec3 specular = SPECULAR_REFLECTANCE * pow(dot(R, V), material.shininess) * material.specular_color;
  vec3 phong_illumination = ambient + diffuse + specular;

  f_color = vec4(phong_illumination, material.alpha);
}
