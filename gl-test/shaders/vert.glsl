#version 330 core
in vec3 in_pos;
out vec3 pos;


void main()
{
  pos = in_pos;
  gl_Position = vec4(in_pos, 1.0);
}
