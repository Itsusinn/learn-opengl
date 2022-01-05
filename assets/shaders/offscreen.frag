#version 450 core
out vec4 fragColor;
in VS_OUTPUT {
    vec2 TexCoord;
} IN;
uniform sampler2D frame;
void main()
{
    fragColor = texture(frame, IN.TexCoord);
    // fragColor = vec4(0.6,0.3,0.6,1.0);
}