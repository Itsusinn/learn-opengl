#version 450 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;
layout (location = 2) in vec3 Normal;

uniform mat4 vp_proj;
uniform mat4 m_proj;
uniform mat3 NormalMat;

out VS_OUTPUT {
    vec2 TexCoord;
    vec3 Normal;
    vec3 WorldCoord;
} OUT;

void main()
{
    // 补充的w分量用于透视除法(Perspective Division)
    gl_Position = m_proj * vp_proj * vec4(Position, 1.0);
    OUT.TexCoord = TexCoord;
    OUT.WorldCoord = vec3( m_proj * vec4(Position, 1.0));
    OUT.Normal = normalize(NormalMat * Normal);
}