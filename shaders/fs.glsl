in vec2 pass_uv;
in vec3 color;

out vec4 out_color;

uniform sampler2D terrain_tex;

void main() {
   out_color = vec4(color, 1.0) * texture(terrain_tex, pass_uv);
}
