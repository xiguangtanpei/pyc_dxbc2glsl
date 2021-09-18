#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform Constants {
    mat4 Model;
    mat4 View;
    mat4 Projection;
};

layout(location = 0) in vec4 a_Pos;
layout(location = 1) in vec2 a_Uv;

layout(location = 2) out vec2 v_Uv;

void main() {
    gl_Position = Projection * View * Model * vec4(a_Pos, 1.0);
    v_Uv = -abs(a_Uv);
}
