#version 450 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;
layout (location = 2) in vec3 Normal;

uniform mat4 MVPMat;
uniform mat3 NormalMat;

out VS_OUTPUT {
    vec2 TexCoord;
    vec3 Normal;
} OUT;

void main()
{
    // 补充的w分量用于透视除法(Perspective Division)
    gl_Position = MVPMat * vec4(Position, 1.0);
    OUT.TexCoord = TexCoord;
    OUT.Normal = normalize(NormalMat * Normal);
}