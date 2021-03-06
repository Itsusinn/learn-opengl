#version 450 core
out vec4 fragColor;
in VS_OUTPUT {
    vec3 Color;
} IN;

void main()
{
    fragColor = vec4(IN.Color, 1.0f);
}