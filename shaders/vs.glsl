layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;
layout (location = 2) in uint facenum;

out vec2 pass_uv;
out vec3 color;

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 projection_matrix;

void main() {
    gl_Position = projection_matrix * view_matrix * model_matrix * vec4(pos, 1.0);
    
    pass_uv = uv;
    
    switch (facenum) {
        case 0u:
            color = vec3(0.8, 0.8, 0.8);
            break;
        case 1u:
            color = vec3(1.0, 1.0, 1.0);
            break;
        case 2u:
            color = vec3(0.9, 0.9, 0.9);
            break;
        case 3u:
            color = vec3(0.7, 0.7, 0.7);
            break;
        case 4u:
            color = vec3(0.8, 0.8, 0.8);
            break;
        case 5u:
            color = vec3(0.8, 0.8, 0.8);
            break;
        default:
            color = vec3(1.0, 0.0, 0.0);
            break;
    }
}
