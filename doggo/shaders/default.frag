#version 330 core

in vec3 color;
in vec2 coord;

out vec4 out_color;

uniform sampler2D tex;

void main()
{
  out_color = texture(tex, vec2(1.0, 1.0) - coord);
}
