#version 330 core
layout (location = 0) in vec3 position;
uniform mat4 scale;
uniform mat4 translate;
void main() {
    gl_Position = translate * scale * vec4(position, 1.0);
}
