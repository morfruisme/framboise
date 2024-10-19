#version 330

out vec4 color;

uniform vec2 dim;
uniform vec4 line_color;
uniform float line_width2;

uniform sampler2D tex;
uniform vec2 p;
uniform vec2 delta;

float dist2_to_line(vec2 p, vec2 a, vec2 b) {
    vec2 v = p - a;
    vec2 line = b - a;
    float l = dot(line, line);
    float t = clamp(dot(v, line)/l, 0, 1);
    vec2 closest = v - line * t;
    return dot(closest, closest);
}

void main() {
    vec2 uv = gl_FragCoord.xy;

    if (dist2_to_line(uv, p - delta, p) <= line_width2) {
        color = line_color;
    }
    else {
        color = texture(tex, uv/dim);
    }
}