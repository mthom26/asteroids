in vec2 pos;
// in vec3 color;
in vec2 tex_coord;

// out vec2 v_uv;
// out vec3 v_color;
out vec2 v_tex_coord;

void main() {
  // v_color = color;
  // 1280 (screen width) / 64 (sprite width) = 20.0
  // 720 (screen height) / 64 (sprite height) = 11.25
  // v_uv = vec2(pos.x * 20.0 + 0.5, pos.y * 11.25 + 0.5);
  v_tex_coord = vec2(tex_coord.x, tex_coord.y);
  gl_Position = vec4(pos, 0.0, 1.0);
}
