#version 330 core
in vec2 TexCoords;
out vec4 FragColor;
uniform sampler2D textTexture;
void main() {
    float alpha = texture(textTexture, TexCoords).r;
    vec4 debugColor;
    if (alpha > 0.5) {
        debugColor = vec4(1.0, 0.0, 0.0, 1.0);
    } else if (alpha > 0.0) {
        debugColor = vec4(0.0, 1.0, 0.0, 1.0);
    } else {
        debugColor = vec4(0.0, 0.0, 1.0, 0.2);
    }
    FragColor = debugColor;
}