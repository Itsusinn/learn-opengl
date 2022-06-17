#version 450 core

in VS_OUTPUT {
    vec2 TexCoord;
    vec3 Normal;
} IN;

out vec4 FragColor;

uniform sampler2D texture0;
uniform sampler2D texture1;
uniform vec4 Ambient;

void main()
{
    vec4 scatteredLight = Ambient;
    vec4 color = texture(texture0, IN.TexCoord);

    FragColor = min(color * scatteredLight, vec4(1.0));
}
