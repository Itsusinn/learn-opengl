#version 450 core
layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

uniform mat4 vp_proj;
uniform mat4 m_proj;

out VS_OUTPUT {
    vec2 TexCoord;
} OUT;
void main()
{
    // 补充的w分量用于透视除法(Perspective Division)
    gl_Position = vp_proj * m_proj * vec4(Position, 1.0);
    OUT.TexCoord = TexCoord;
}