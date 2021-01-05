#version 330 core

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec4 aColor;

uniform mat4 uVP = mat4(1.0);
uniform mat4 uM = mat4(1.0);

out vec4 oColor;

void main() {
    gl_Position = uVP * uM * vec4(aPosition, 1.0);
    oColor = aColor;
}