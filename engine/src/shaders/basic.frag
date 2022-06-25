#version 450

layout(location = 0) in vec4 v_color;
layout(location = 1) in vec3 v_normal;

layout(location = 0) out vec4 f_color;

const vec3 LIGHT = vec3(0.0, 0.0, 1.0);

void main() {
  float brightness = dot(normalize(v_normal), LIGHT);
  vec4 dark_color = v_color * vec4(0.6, 0.6, 0.6, 1.0);
  
  f_color = mix(dark_color, v_color, brightness);
}
