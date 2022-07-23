#version 450

// Configuration
// /////////////
#define MAX_POINT_LIGHTS 10

// Inputs/Ouputs
// /////////////
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_uv;
layout(location = 3) in vec3 a_tangent;

layout(location = 0) out vec3 v_position;
layout(location = 1) out vec2 v_uv;
layout(location = 2) out mat3 v_TBN;

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

layout(push_constant) uniform MdrPushConstants
{
	mat4 transformation_matrix;
} object;

// Shader Entry Point
// //////////////////
void main() {
  // Calculate world position of input vertex
  vec4 world_position =  object.transformation_matrix * vec4(a_position, 1.0);

  // Calculate surface normal at input vertex
  // TODO fix to use transpose inverse (or probably just remove nonuniform scaling)
  vec3 normal = normalize(mat3(scene_data.camera.view) * a_normal);
  // Write tangent and bitangent vectors for normal mapping
  vec3 tangent = normalize(mat3(scene_data.camera.view) * a_tangent);
  // Gram-Scmidt re-orthogonalization of the tangent with respect to the normal
  tangent = normalize(tangent - dot(tangent, normal) * normal);
  vec3 bitangent = normalize(cross(tangent, normal));
  v_TBN = mat3(tangent, bitangent, normal);
  
  // Write output UVs
  v_uv = a_uv;
  
  // Write output of vertex position
  v_position = world_position.xyz;
  gl_Position = scene_data.camera.proj * scene_data.camera.view * world_position;
}
