#version 100
precision mediump float;

varying vec2 uv;

uniform sampler2D tex;
uniform sampler2D mask;
uniform highp vec2 offset;

void main() {
  vec4 color = texture2D(tex, uv + offset);
  vec4 alpha = texture2D(mask, uv + offset);
  gl_FragColor = vec4(color.rgb, alpha.r);
}
