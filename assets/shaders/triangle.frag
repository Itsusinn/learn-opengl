#version 450 core
out vec4 fragColor;
in VS_OUTPUT {
    vec4 Color;
} IN;
void main()
{
    fragColor = IN.Color;
}