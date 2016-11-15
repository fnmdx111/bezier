#version 150 core

in vec4 a_position;
in vec4 a_norm;

uniform mat4 u_persp;
uniform vec4 u_eye;

out vec4 v_eye;
out vec4 v_lgh;
out vec4 n_unnormalized;
out vec4 v_pos;

struct ProjectionObject {
  mat4 ctm;
  mat4 persp;
  vec4 eye;
};
uniform ProjectionObject u_prj;

struct LightObject {
  vec4 ambient;
  vec4 diffuse;
  vec4 specular;
  vec4 pos;
};
uniform LightObject u_light;

void main()
{
  gl_Position = u_prj.persp * u_prj.ctm * a_position;

  n_unnormalized = a_norm;
  v_eye = u_prj.eye - a_position;
  v_lgh = u_light.pos - a_position;
  v_pos = a_position;
}

