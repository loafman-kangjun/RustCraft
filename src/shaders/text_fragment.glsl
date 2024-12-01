#version 330 core
in vec2 TexCoords;
out vec4 FragColor;

uniform sampler2D textTexture;

void main() {
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(textTexture, TexCoords).r);
    FragColor = sampled;
} 