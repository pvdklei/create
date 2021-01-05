#version 330 core

out vec4 color;

in vec2 oTexCoord;
uniform sampler2D uTexture;

void main() {
    color = texture(uTexture, oTexCoord);
    // color = vec4(1.0, 1.0, 1.0, 1.0);
}