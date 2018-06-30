#version 330 core
uniform vec4 color;
out vec4 fragment_color;
void main() {
    fragment_color = color;
}
