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
  // L - Direction from fragment position to light
  vec3 L = normalize(LIGHT.position - v_position.xyz);
  // N - (Normalized) fragment normal
  vec3 N = normalize(v_normal);
  // V - Direction from fragment position to camera
  //vec3 V = normalize;
  // R - Direction of a ray reflected off the surface
  vec3 R = normalize(reflect(-L, N));

  // Phong reflection model
  // Ambient contribution from lights
  vec3 ambient = AMBIENT_LIGHT;
  // Diffuse contribution
  //vec3 diffuse = 
  // Specular contribution
  //vec3 specular = 


  float brightness = dot(N, L);
  vec4 light_color = vec4(material.diffuse_color, material.alpha);
  vec4 dark_color = light_color * vec4(0.6, 0.6, 0.6, 1.0);
  
  f_color = mix(dark_color, light_color, brightness);
}
