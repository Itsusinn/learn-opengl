#version 450 core
layout (location = 0) in vec3 Position;
layout (location = 1) in vec4 Color;
layout (location = 2) in vec2 TexCoord;
out VS_OUTPUT {
    vec4 Color;
    vec2 TexCoord;
} OUT;
void main()
{
    // 补充的w分量用于透视除法(Perspective Division)
    gl_Position = vec4(Position, 1.0);
    OUT.Color = Color;
    OUT.TexCoord = TexCoord;
}