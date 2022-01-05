#version 450 core
out vec4 fragColor;
in VS_OUTPUT {
    vec2 TexCoord;
} IN;
uniform sampler2D texture0;
void main()
{
    // fragColor = IN.Color;
    fragColor = texture(texture0, IN.TexCoord);
    // fragColor = texture( texture0, IN.TexCoord) * IN.Color;
    // fragColor =  texture(texture1, IN.TexCoord) * texture(texture0, IN.TexCoord);
}