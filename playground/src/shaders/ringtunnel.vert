#version 330 core

layout (location = 0) in vec3 aPosition;

uniform mat4 uMVP;
uniform vec4 uColor;

out vec4 oColor;

void main() {
    gl_Position = uMVP * vec4(aPosition, 1.0);
    oColor = uMVP * uColor;
}