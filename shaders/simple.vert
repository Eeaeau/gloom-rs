#version 430 core

in vec3 position;

void main()
{
    // when mirroring we played with these variables. But when changing position we had to change the rotation of the drawing. this is done in main.rs and with the function gl::FrontFace
    gl_Position = vec4(
        position.x, 
        -position.y, // flip y-position of every vertex to mirror around x-axis
        position.z, 
        1.0f); 
}