#define MAX_STEPS 255
#define MAX_DISTANCE 100.0
#define MIN_DISTANCE 0.0
#define MIN_HIT 0.0001

uniform vec2 u_screen_size;

uniform vec3 u_view_eye;
uniform vec3 u_view_target;
uniform vec3 u_view_up;
uniform mat4 u_view_matrix;
uniform float u_view_fov_degrees;

out vec4 frag_color;

float sdf_sphere(vec3 point) {
    return length(point) - 1.0;
}

float sdf_cube(vec3 p) {
    // If d.x < 0, then -1 < p.x < 1, and same logic applies to p.y, p.z
    // So if all components of d are negative, then p is inside the unit cube
    vec3 d = abs(p) - vec3(1.0, 1.0, 1.0);
    
    // Assuming p is inside the cube, how far is it from the surface?
    // Result will be negative or zero.
    float insideDistance = min(max(d.x, max(d.y, d.z)), 0.0);
    
    // Assuming p is outside the cube, how far is it from the surface?
    // Result will be positive or zero.
    float outsideDistance = length(max(d, 0.0));
    
    return insideDistance + outsideDistance;
}


float sdf_scene(vec3 point){
    return sdf_cube(point);
}

float march(vec3 eye, vec3 ray_direction){
    float depth = MIN_DISTANCE;
    for (int i = 0; i < MAX_STEPS; i++){
        float distance = sdf_scene(eye + depth * ray_direction);

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

// TODO: remove trig ops
vec3 ray_direction(float fov, vec2 normalized_frag_coord){
    vec2 xy = normalized_frag_coord;
    float z = u_screen_size.y - tan(radians(fov) / 2.0);
    return normalize(vec3(xy, -z));    
}

vec2 frag_coord() {
    vec2 frag_coord = gl_FragCoord.xy / u_screen_size; // subtract 1 mult 2 to normalize to -1..1
    frag_coord = gl_FragCoord.xy - u_screen_size / 2.;
    return frag_coord;
}

void main()
{    
    vec3 ray_dir = ray_direction(u_view_fov_degrees, frag_coord());
    vec3 world_dir = (u_view_matrix * vec4(ray_dir, 0.)).xyz;

    float dist = march(u_view_eye, world_dir);

    if (dist > MAX_DISTANCE - MIN_HIT){
        frag_color = vec4(0.0);
        return;
    }

    // lighting jazz using Phong

    frag_color = vec4(1.0, 0.0, 0.0, 1.0);
}