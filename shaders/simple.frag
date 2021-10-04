#version 450 core

uniform layout(location = 4) float time;

out vec4 color;
in vec4 fragmentColor;
in vec3 vertexNormals;


// Logic for deciding checker block
void checkarboard(in float coordinate, in uint range, out bool result) {

    uint x = uint(mod(coordinate, range));

    // range / 2  will be the dimension of each block.
    if(x < range/2)
        result = bool(0);
    else
        result = bool(1);
}
#define PI 3.14


void main()
{

    // bool checker_x;
    // bool checker_y;

    // float pi =  3.14;
    // float oc = 200.0*sin(2*PI*time);

    // checkarboard(gl_FragCoord.x+oc, 50, checker_x);
    // checkarboard(gl_FragCoord.y, 50, checker_y);

    // // Logic for making of the pattern.
    // if(checker_y)
    //     if(checker_x)
    //         color = vec4(1.0f, 1.0f, 1.0f, 1.0f); // white
    //     else
    //         color = vec4(0.0f, 0.0f, 0.0f, 1.0f);   // black
    // else
    //     if(!checker_x)
    //         color = vec4(1.0f, 1.0f, 1.0f, 1.0f); // white
    //     else
    //         color = vec4(0.0f, 0.0f, 0.0f, 1.0f); // black

    //color = fragmentColor;
    vec3 lightDirection = normalize(vec3(0.8, -0.2, 0.6));

    color = vec4(fragmentColor.xyz * max(0, dot(vertexNormals, -lightDirection)), fragmentColor.w);
    // color = vec4(normalize(vertexNormals.xzy), 1.0);

}
