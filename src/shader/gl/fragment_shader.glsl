#version 330

in vec2 v_tex_coords;

uniform sampler2D tex;

out vec4 color;

void main() {
    color = texture(tex, v_tex_coords);
}
