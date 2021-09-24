#version 450 core

uniform layout(location = 4) float time;
uniform layout(location = 5) mat4 transform_matrix;

in vec3 position;

in layout(location=2) vec4 vertexColor;

out vec4 fragmentColor;

#define PI 3.14

void main()
{
    mat4 diagMatrix = mat4(1);

    float oc = 0.5*sin(2*PI*0.5*time);

    float a = 1.0;
    float b = 0.0;
    float c = 0.0;
    float d = 0.0;
    float f = 0.0;
    float e = 1.0;

    mat4 random_matrix = mat4(
        a, b, 0.0, oc,
        d, e, 0.0, f,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    // mat4 random_matrix = mat4(1);

    gl_Position = transform_matrix * vec4(position, 1.0f);
    // gl_Position = diagMatrix * position.;

    // when mirroring we played with these variables. But when changing position we had to change the rotation of the drawing. this is done in main.rs and with the function gl::FrontFace
    // gl_Position = vec4(
    //     position.x,
    //     position.y, // flip y-position of every vertex to mirror around x-axis
    //     position.z,
    //     1.0f);

    fragmentColor = vertexColor;
}
