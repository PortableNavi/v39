#version 330 core
in vec3 vert_color;
out vec4 out_color;

uniform float br;

void main()
{
  out_color = vec4(vert_color*br, 1.0f);
}
