#version 450

// Configuration
////////////////
#define MAX_POINT_LIGHTS 10

// Inputs/Ouputs
////////////////
layout(location = 0) in vec3 v_position;
layout(location = 1) in vec3 v_normal;

layout(location = 0) out vec4 f_color;

// Input buffer objects
///////////////////////
struct CameraData {
  vec3 position;

  mat4 view;
  mat4 proj;
};

struct PointLightData {
  vec3 color;
  vec3 position;
  float brightness;
};

layout(set = 0, binding = 0) buffer SceneDataObject {
  CameraData camera;
  PointLightData point_lights[MAX_POINT_LIGHTS];
  uint point_light_count;
} scene_data;

layout(set = 1, binding = 0) uniform MaterialUniformData {
  vec3 diffuse_color;
  float alpha;

  vec3 specular_color;
  float shininess;
} material;

///////////////////////
//TODO Remove test code
///////////////////////
const float ambient_strength = 0.1;
const float specular_strength = 0.5;

// Lighting functions
/////////////////////
vec3 calculate_point_light_contribution(PointLightData light, vec3 N, vec3 V);

// Shader Entry Point
/////////////////////
void main() {
  // Calculate normalized directional vectors for lighting
  // Surface normal
  vec3 N = normalize(v_normal);
  // Direction to viewer
  vec3 V = normalize(scene_data.camera.position - v_position);

  vec3 result = vec3(0.0);
  for (int i = 0; i < scene_data.point_light_count; i++) {
    result += calculate_point_light_contribution(scene_data.point_lights[i], N, V) * material.diffuse_color;
  }

  f_color = vec4(result, material.alpha);
}

// Impl lighting functions
//////////////////////////

vec3 calculate_point_light_contribution(PointLightData light, vec3 N, vec3 V) {
  // Light-specific direction vectors
  // Direction to light
  vec3 L = normalize(light.position - v_position);
  // Direction of a reflected light ray
  vec3 R = reflect(-L, N);

  // Phong BRDF
  // Ambient contribution
  vec3 ambient = ambient_strength * light.color;
  
  // Diffuse contribution
  float diffusion_coefficient = max(dot(N, L), 0.0);
  vec3 diffuse = diffusion_coefficient * light.color;

  // Specular contribution
  float specular_coefficient = pow(max(dot(V, R), 0.0), material.shininess);
  vec3 specular = specular_strength * specular_coefficient * light.color;

  return (ambient + diffuse + specular);
}