#version 450 core
out vec4 fragColor;
in VS_OUTPUT {
    vec4 Color;
    vec2 TexCoord;
} IN;
uniform sampler2D texture_0;
void main()
{
    // fragColor = IN.Color;
    // fragColor = texture( texture_0, IN.TexCoord);
    fragColor = texture( texture_0, IN.TexCoord) * IN.Color;
}