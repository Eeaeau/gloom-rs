#version 450 core

uniform layout(location = 4) float time;

in vec3 position;

in layout(location=2) vec4 vertexColor;

out vec4 fragmentColor;
// layout(location = 1) in vec3 vertexColor;

#define PI 3.14

void main()
{
    mat4 diagMatrix = mat4(1);

    float oc = 1.0*sin(2*PI*0.5*time);

    mat4 random_matrix = mat4(
        1.0+oc, 0.0, 0.0, 0.0,
        0.0, 1.0-oc, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    // mat4 random_matrix = mat4(1);

    gl_Position = random_matrix * vec4(position, 1.0f);
    // gl_Position = diagMatrix * position.;

    // when mirroring we played with these variables. But when changing position we had to change the rotation of the drawing. this is done in main.rs and with the function gl::FrontFace
    // gl_Position = vec4(
    //     position.x,
    //     position.y, // flip y-position of every vertex to mirror around x-axis
    //     position.z,
    //     1.0f);

    fragmentColor = vertexColor;
}
