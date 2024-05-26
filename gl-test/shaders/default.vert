#version 330 core
layout(location=0) in vec3 in_pos;
layout(location=1) in vec3 in_color;
layout(location=2) in vec2 in_coord;

out vec3 color;
out vec2 coord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

void main()
{
  color = in_color;
  coord = in_coord;

  gl_Position = proj * view * model *  vec4(in_pos, 1.0);
}
