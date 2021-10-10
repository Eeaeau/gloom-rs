#version 450 core

//uniform layout(location = 4) float time;
uniform layout(location = 5) mat4 modelMatrix;
uniform layout(location = 6) mat4 transform_matrix;
//uniform layout(location = 5) mat4 viewProjection;
//layout(location = 2) in vec4 color;
in vec3 position;
in layout(location=2) vec4 vertexColor;
in layout(location=3) vec3 normals;

out vec3 vertexNormals;
out vec4 fragmentColor; //mb change name for this
//out vec4 color_vert;
#define PI 3.14

void main()
{
    /* mat4 diagMatrix = mat4(1);

    float oc = 0.5*sin(2*PI*0.5*time);

    float a = 1.0;
    float b = 0.0;
    float c = 0.0;
    float d = 0.0;
    float f = 0.0;
    float e = 1.0; */

    gl_Position = transform_matrix * vec4(position, 1.0f);

    // when mirroring we played with these variables. But when changing position we had to change the rotation of the drawing. this is done in main.rs and with the function gl::FrontFace
    // gl_Position = vec4(
    //     position.x,
    //     position.y, // flip y-position of every vertex to mirror around x-axis
    //     position.z,
    //     1.0f);

    fragmentColor = vertexColor;
    //vertexNormals = normals;
    vertexNormals = normalize(mat3(modelMatrix) * normals);

}
