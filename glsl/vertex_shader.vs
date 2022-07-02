#version 430 core

in vec2 position;
out vec2 my_attr; // our new attribute

uniform mat4 matrix;

void main() {
    my_attr = position;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}



// uniform float t;

// void main() {
//     vec2 pos = position;
//     pos.x += t;
//     gl_Position = vec4(pos, 0.0, 1.0);
// }