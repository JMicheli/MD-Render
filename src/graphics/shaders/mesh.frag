#version 450

// Configuration
// /////////////
#define MAX_POINT_LIGHTS 10
#define GAMMA_FACTOR 2.2

// Inputs/Ouputs
// /////////////
layout(location = 0) in vec3 v_position;
layout(location = 1) in vec3 v_normal;
layout(location = 2) in vec2 v_uv;

layout(location = 0) out vec4 f_color;

// Input buffer objects
// ////////////////////
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

layout(set = 0, binding = 0) buffer MdrSceneData {
  CameraData camera;
  PointLightData point_lights[MAX_POINT_LIGHTS];
  uint point_light_count;
} scene_data;

layout(set = 1, binding = 0) uniform MdrMaterialUniformData {
  vec3 specular_color;
  float shininess;
} material;
// Base color of material
layout(set = 1, binding = 1) uniform sampler2D diffuse_map;
// Roughness map for material
layout(set = 1, binding = 2) uniform sampler2D roughness_map;


///////////////////////
//TODO Remove test code
///////////////////////
const float ambient_strength = 0.1;

// Lighting functions
// //////////////////
vec3 calculate_point_light_contribution(PointLightData light, vec3 specular_strength, vec3 N, vec3 V);

// Shader Entry Point
// //////////////////
void main() {
  // Calculate normalized directional vectors for lighting
  // Surface normal
  vec3 N = normalize(v_normal);
  // Direction to viewer
  vec3 V = normalize(scene_data.camera.position - v_position);

  // Sample diffuse map to get color
  vec4 diffuse_color = texture(diffuse_map, v_uv);
  // Sample specular map to get specular strength
  vec3 specular_strength = vec3(1.0) - texture(roughness_map, v_uv).xyx;

  vec3 result = vec3(0.0);
  for (int i = 0; i < scene_data.point_light_count; i++) {
    result += calculate_point_light_contribution(scene_data.point_lights[i], specular_strength, N, V) * diffuse_color.xyz;
  }

  // Perform gamma correction
  vec3 gamma_corrected_result = pow(result, vec3(1.0/GAMMA_FACTOR));

  f_color = vec4(gamma_corrected_result, diffuse_color.w);
}

// Impl lighting functions
// ///////////////////////

vec3 calculate_point_light_contribution(PointLightData light, vec3 specular_strength, vec3 N, vec3 V) {
  // Light-specific direction vectors
  // Direction to light
  vec3 L = normalize(light.position - v_position);
  // Blinn-Phong halfway vector
  vec3 H = normalize(L + V);

  // Light color adjusted by brightness
  vec3 light_color = light.color * light.brightness;

  // Blinn-Phong BRDF
  // Ambient contribution
  vec3 ambient = ambient_strength * light_color;
  
  // Diffuse contribution
  float diffusion_coefficient = max(dot(N, L), 0.0);
  vec3 diffuse = diffusion_coefficient * light_color;

  // Specular contribution
  float specular_coefficient = pow(max(dot(N, H), 0.0), material.shininess);
  vec3 specular = specular_strength * light_color * specular_coefficient ;

  return (ambient + diffuse + specular);
}