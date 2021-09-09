#version 430 core

out vec4 color;

// Logic for deciding checker block
void checkarboard(in float coordinate, in uint range, out bool result) {

    uint x = uint(mod(coordinate, range));

    // range / 2  will be the dimension of each block. 
    if(x < range/2)
        result = bool(0);
    else
        result = bool(1);
}

void main()
{
    bool checker_x;
    bool checker_y;

    checkarboard(gl_FragCoord.x, 50, checker_x);
    checkarboard(gl_FragCoord.y, 50, checker_y);

    // Logic for making of the pattern.
    if(checker_y)
        if(checker_x)
            color = vec4(1.0f, 1.0f, 1.0f, 1.0f); // white
        else
            color = vec4(0.0f, 0.0f, 0.0f, 1.0f);   // black
    else
        if(!checker_x)
            color = vec4(1.0f, 1.0f, 1.0f, 1.0f); // white 
        else
            color = vec4(0.0f, 0.0f, 0.0f, 1.0f); // black
            
}
