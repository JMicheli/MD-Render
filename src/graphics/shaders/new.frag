#version 450

// Configuration
// /////////////
#define MAX_POINT_LIGHTS 10
#define GAMMA_FACTOR 2.2

// Inputs/Ouputs
// /////////////
layout(location = 0) in vec3 v_position;
layout(location = 1) in vec2 v_uv;
layout(location = 2) in mat3 v_TBN;

layout(location = 0) out vec4 f_color;

// Input buffer objects
// ////////////////////

// Data representing a camera in the scene
struct CameraData {
  // Camera's position in world space
  vec3 position;
  // View transformation matrix
  mat4 view;
  // Perspective projection matrix
  mat4 proj;
};

// Data representing a point light
struct PointLightData {
  // The RGB color of the light
  vec3 color;
  // The position of the light in world space
  vec3 position;
  // The brightness factor of the light
  float brightness;
};

// Data representing the scene
layout(set = 0, binding = 0) buffer MdrSceneData {
  // The camera being used to render the scene
  CameraData camera;
  // Up to MAX_POINT_LIGHTS point light values
  PointLightData point_lights[MAX_POINT_LIGHTS];
  // Maximum point_light index with a valid value
  uint point_light_count;
} scene_data;

// Data representing a material
layout(set = 1, binding = 0) uniform MdrMaterialUniformData {
  // The color of an object's specular highlight
  vec3 specular_color;
  // The exponential specular factor for Blinn-Phong 
  float shininess;
} material;

// Material texture maps
// Base color of material
layout(set = 1, binding = 1) uniform sampler2D diffuse_map;
// Roughness map for material
layout(set = 1, binding = 2) uniform sampler2D roughness_map;
// Normal map for material
layout(set = 1, binding = 3) uniform sampler2D normal_map;

// Shader Entry Point
// //////////////////
void main() {
  f_color = vec4(0.0, 0.0, 0.0, 1.0);

  // ///////////////
  // IGNORE
  // ///////////////
  // These are just to keep it from crashing for now
  uint i = scene_data.point_light_count;
  float s = material.shininess;
  vec4 c1 = texture(diffuse_map, v_uv);
  vec4 c2 = texture(roughness_map, v_uv);
  vec4 c3 = texture(normal_map, v_uv);
}
