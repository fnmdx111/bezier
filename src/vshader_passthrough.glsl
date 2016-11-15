#version 150
attribute vec4 a_position;
attribute vec4 a_norm;

uniform mat4 u_persp;

varying vec4 v_eye;
varying vec4 v_lgh;
varying vec4 n_unnormalized;
varying vec4 v_pos;

uniform u_prj {
  mat4 ctm;
};

uniform u_light {
  vec4 ambient;
  vec4 diffuse;
  vec4 specular;
  vec4 pos
};

void main()
{
  gl_Position = u_persp * u_prj.ctm * a_position;

  n_unnormalized = a_norm;
  v_eye = u_eye - a_position;
  v_lgh = u_light.pos - a_position;
  v_pos = a_position;
}

