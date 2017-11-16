layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

out vec2 pass_uv;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 projection_matrix;

void main() {
    gl_Position = projection_matrix * view_matrix * model_matrix * vec4(pos, 1.0);
    
    pass_uv = uv;
}
