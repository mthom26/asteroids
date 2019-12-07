in vec2 pos;
// in vec3 color;

out vec2 v_uv;

void main() {
  gl_Position = vec4(pos, 0.0, 1.0);

  // 1280 (screen width) / 64 (sprite width) = 20.0
  // 720 (screen height) / 64 (sprite height) = 11.25
  v_uv = vec2(pos.x * 20.0 + 0.5, pos.y * 11.25 + 0.5);
}
