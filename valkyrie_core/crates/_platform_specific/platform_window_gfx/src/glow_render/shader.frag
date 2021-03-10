#define MAX_STEPS 1000
#define MAX_DISTANCE 2000.0
#define MIN_DISTANCE 0.0
#define MIN_HIT 0.0001

uniform vec2 u_screen_size;

uniform vec3 u_view_eye;
uniform vec3 u_view_target;
uniform vec3 u_view_up;
uniform mat4 u_view_matrix;

out vec4 frag_color;

float sdf_sphere(vec3 point) {
    return length(point) - 1.0;
}



float march(vec3 eye, vec3 ray_direction){
    float depth = MIN_DISTANCE;
    for (int i = 0; i < MAX_STEPS; i++){
        float distance = sdf_sphere(eye + depth * ray_direction);

        if (distance < MIN_HIT) {
            return depth;
        }

        depth += distance;

        if (distance >= MAX_DISTANCE){
            return MAX_DISTANCE;
        }
    }

    return depth;
}

void main()
{
    // Normalize frag coords
    vec2 frag_coord = gl_FragCoord.xy / u_screen_size; // subtract 1 mult 2 to normalize to -1..1

    frag_color = vec4(frag_coord, 0.0, 1.0f);
}