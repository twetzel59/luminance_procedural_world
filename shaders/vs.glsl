layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

out vec3 pass_color;

uniform mat4 model_matrix;
uniform mat4 projection_matrix;

void main() {
    gl_Position = projection_matrix * model_matrix * vec4(pos, 1.0);
    
    pass_color = color;
}
