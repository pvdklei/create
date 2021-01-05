#version 330 core

#define MAX_MODELS 128

layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec4 aColor;
layout (location = 2) in float aModel;

uniform mat3 uModels[MAX_MODELS];
uniform mat4 uViewProjection;
uniform bool doForeground;

out vec4 oColor;

void main()
{
    float z;
    if (doForeground) { z = 1.0; } else { z = -1.0; }
    vec3 modelSpacePos = uModels[int(aModel)] * vec3(aPosition, 1.0);
    gl_Position = uViewProjection * vec4(modelSpacePos.xy, z, modelSpacePos.z);
    oColor = aColor;
}
