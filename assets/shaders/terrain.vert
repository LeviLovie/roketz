#version 100
attribute vec2 position;
attribute vec2 texcoord;

uniform vec2 offset;

varying vec2 uv;

void main() {
  gl_Position = vec4(position, 0.0, 1.0);
  uv = texcoord + offset;
}
