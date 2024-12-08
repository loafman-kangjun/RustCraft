#version 330 core

layout (location = 0) in vec3 aPos;   // 位置 (x, y, z)
layout (location = 1) in vec3 aColor; // 颜色 (r, g, b)

out vec3 vColor; // 输出到片段着色器

uniform mat4 projection;

void main()
{
    gl_Position = projection * vec4(aPos, 1.0);
    vColor = aColor; // 传递颜色到片段着色器
}
