#version 450 core

in VS_OUTPUT {
    vec2 TexCoord;
    vec3 Normal;
    vec3 WorldCoord;
} IN;

out vec4 FragColor;

uniform sampler2D texture0;

uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 viewPos;

void main()
{

    float ambientStrength = 0.1;
    vec3 ambient = lightColor * ambientStrength;

    vec3 norm = normalize(IN.Normal);
    vec3 light_direction = normalize(lightPos - IN.WorldCoord);
    float diff = max(dot(norm,light_direction),0.0);
    vec3 diffuse = diff * lightColor;

    float specularStrength = 0.5;
    vec3 view_dirction = normalize(viewPos - IN.WorldCoord);
    vec3 reflect_direction = reflect(-light_direction, norm);
    float spec = pow(max(dot(view_dirction,reflect_direction),0.0),256);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 objectColor = vec3(texture(texture0, IN.TexCoord));

    vec3 result = objectColor * (ambient + diffuse + specular);
    FragColor = min(vec4(result,1.0), vec4(1.0));
}
