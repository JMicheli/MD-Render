#version 450

// Inputs/Ouputs
////////////////
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;

layout(location = 0) out vec3 v_position;
layout(location = 1) out vec3 v_normal;

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
  PointLightData point_lights[10];
  uint point_light_count;
} scene_data;

layout(push_constant) uniform ObjectPushConstants
{
	mat4 transformation_matrix;
} object;

// Shader Entry Point
/////////////////////
void main() {
  // Calculate world position of input vertex
  vec4 world_position =  object.transformation_matrix * vec4(a_position, 1.0);

  // Calculate surface normal at input vertex
  // TODO fix to use transpose inverse
  v_normal = normalize(mat3(scene_data.camera.view) * a_normal);
  
  // Write output of vertex position
  v_position = world_position.xyz;
  gl_Position = scene_data.camera.proj * scene_data.camera.view * world_position;
}
