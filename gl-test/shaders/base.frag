#version 330 core
in vec3 pos;
out vec4 col;

void main()
{
  col = vec4(pos, 1.0f);
}
