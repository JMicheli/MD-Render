#version 450

// Inputs/Ouputs
////////////////
layout(location = 0) in vec3 v_position;
layout(location = 1) in vec3 v_normal;

layout(location = 0) out vec4 f_color;

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

///////////////////////
//TODO Remove test code
///////////////////////
const vec3 light_position = vec3(1.0, 3.0, 3.0);
const vec3 light_color = vec3(1.0);

const float ambient_strength = 0.1;
const float specular_strength = 0.5;

// Shader Entry Point
/////////////////////
void main() {
  // Calculate normalized directional vectors for lighting
  // Surface normal
  vec3 N = normalize(v_normal);
  // Direction to light
  vec3 L = normalize(light_position - v_position);
  // Direction to viewer
  vec3 V = normalize(camera.position - v_position);
  // Direction of a reflected light ray
  vec3 R = reflect(-L, N);

  // Phong BRDF
  // Ambient contribution
  vec3 ambient = ambient_strength * light_color;
  
  // Diffuse contribution
  float diffusion_coefficient = max(dot(N, L), 0.0);
  vec3 diffuse = diffusion_coefficient * light_color;

  // Specular contribution
  float specular_coefficient = pow(max(dot(V, R), 0.0), material.shininess);
  vec3 specular = specular_strength * specular_coefficient * light_color;

  vec3 result = (ambient + diffuse + specular) * material.diffuse_color;
  f_color = vec4(result, material.alpha);
}
