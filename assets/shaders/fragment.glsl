// in vec2 v_uv;
// in vec3 v_color;
in vec2 v_tex_coord;

out vec4 frag;

uniform sampler2D tex;

void main() {
  vec4 texel = texture(tex, v_tex_coord);
  // Check if the texel alpha is less than zero, if so don't render it
  // This allows for images with areas of full opacty and full transprency
  if (texel.a < 0.5) {
    discard;
  }
  frag = texel;
}
