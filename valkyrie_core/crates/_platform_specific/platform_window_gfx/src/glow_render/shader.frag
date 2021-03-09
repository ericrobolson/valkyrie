uniform vec2 u_screen_size;

uniform vec3 u_view_eye;
uniform vec3 u_view_target;
uniform vec3 u_view_up;
uniform mat4 u_view_matrix;

out vec4 frag_color;

void main()
{
    // Normalize frag coords
    vec2 frag_coord = gl_FragCoord.xy / u_screen_size; // subtract 1 mult 2 to normalize to -1..1

    frag_color = vec4(frag_coord, 0.0, 1.0f);
}