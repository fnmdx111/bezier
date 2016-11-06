#version 150
uniform vec4 light_pos;
uniform vec4 eye;

uniform vec4 lamb;
uniform vec4 ldiff;
uniform vec4 lspec;

uniform vec4 mamb;
uniform vec4 mdiff;
uniform vec4 mspec;
uniform float mshine;

varying vec4 v_eye;
varying vec4 v_lgh;
varying vec4 n_unnormalized;
varying vec4 v_pos;

uniform int checkerboard;
uniform float chb_size;

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
  vec4 mamb1 = mamb;
  vec4 mdiff1 = mdiff;
  vec4 color = vec4(0.0, 0.0, 0.0, 1.0);
  if (checkerboard == 1) {
    float mx = mod(float(int(v_pos.x / chb_size)), 2.0);
    float my = mod(float(int(v_pos.y / chb_size)), 2.0);
    float mz = mod(float(int(v_pos.z / chb_size)), 2.0);
    int black = 0;
    /**
     * y 1 0
     * y 0 1
     * + x x
     */

    if (mx < 1.0) {
      if (my < 1.0) {
        if (mz < 1.0) {
          black = 0;
        } else {
          black = 1;
        }
      } else {
        if (mz < 1.0) {
          black = 1;
        } else {
          black = 0;
        }
      }
    } else {
      if (my < 1.0) {
        if (mz < 1.0) {
          black = 1;
        } else {
          black = 0;
        }
      } else {
        if (mz < 1.0) {
          black = 0;
        } else {
          black = 1;
        }
      }
    }
    if (black == 0) {
      mamb1 = vec4(0.0, 0.0, 0.0, 1.0);
      mdiff1 = vec4(1.0, 1.0, 1.0, 1.0);
    } else {
      mamb1 = vec4(0.0, 0.0, 0.0, 1.0);
      mdiff1 = vec4(0.0, 0.0, 0.0, 1.0);
    }
    color = vec4(0.0, 0.0, 0.0, 1.0);
  } else {
    color = pll4(lamb, mamb1);
  }

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
    color += pll4(lspec, mspec) * exp(mshine * log(ddspec));
  }

  gl_FragColor = color;
}
