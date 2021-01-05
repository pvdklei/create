#version 330 core

#define MAX_MODELS 128

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec4 aColor;
layout (location = 2) in float aModel;

uniform mat4 uModels[MAX_MODELS];
uniform mat4 uViewProjection;

out vec4 oColor;

void main()
{
    gl_Position = uViewProjection * uModels[int(aModel)] * vec4(aPosition, 1.0); 
    oColor = aColor;
}