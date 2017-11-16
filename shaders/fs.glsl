in vec2 pass_uv;

out vec4 color;

uniform sampler2D terrain_tex;

void main() {
   color = texture(terrain_tex, pass_uv);
}
