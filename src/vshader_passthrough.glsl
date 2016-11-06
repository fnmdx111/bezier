#version 150
attribute vec4 vPosition;
attribute vec4 vNorm;

uniform mat4 ctm;
uniform mat4 persp;
uniform vec4 light_pos;
uniform vec4 eye;

varying vec4 v_eye;
varying vec4 v_lgh;
varying vec4 n_unnormalized;
varying vec4 v_pos;

void main()
{
  gl_Position = persp * ctm * vPosition;

  n_unnormalized = vNorm;
  v_eye = eye - vPosition;
  v_lgh = light_pos - vPosition;
  v_pos = vPosition;
} 
