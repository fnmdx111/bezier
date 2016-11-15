#version 150

uniform u_light {
  vec4 ambient;
  vec4 diffuse;
  vec4 specular;
  vec4 pos;
};

uniform vec4 u_eye;

uniform u_material {
  vec4 ambient;
  vec4 diffuse;
  vec4 specular;
  float shininess;
};

varying vec4 v_eye;
varying vec4 v_lgh;
varying vec4 n_unnormalized;
varying vec4 v_pos;

vec4 pll4(vec4 a, vec4 b)
{
  vec4 r;
  r.r = a.r * b.r;
  r.g = a.g * b.g;
  r.b = a.b * b.b;
  r.a = a.a * b.a;

  return r;
}

void main()
{
  vec4 mamb1 = u_material.ambient;
  vec4 mdiff1 = u_material.diffuse;
  vec4 color = vec4(0.0, 0.0, 0.0, 1.0);

  vec4 n = normalize(n_unnormalized);
  vec4 vLight = normalize(v_lgh);
  vec4 view = normalize(v_eye);
  vec4 hlf = normalize(vLight + view);

  float dddiff = dot(vLight, n);
  float ddspec = dot(hlf, n);

  if (dddiff > 0.0) {
    color += pll4(ldiff, mdiff1) * dddiff;
  }

  if (ddspec > 0.0) {
    color += pll4(u_light.specular, material.specular)
      * exp(u_material.shininess * log(ddspec));
  }

  gl_FragColor = color;
}
