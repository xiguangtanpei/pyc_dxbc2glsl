/*cbuffer Constants {
    float4x4 Model;
    float4x4 View;
    float4x4 Projection;
}

struct VInput {
    float4 pos : POSITION;
    float2 uv : TEXCOORD;
};

struct VOutput {
    float4 pos : SV_Position;
    uint3 col : COLOR;
    float2 uv : TEXCOORD;
};

Texture2D<float4> Texture;
SamplerState Sampler;

VOutput vs(VInput input) {
    VOutput output;
    output.pos = mul(Projection, mul(View, mul(Model, float4(input.pos.xyz, 1))));
    output.uv = -abs(input.uv);
    output.col = saturate(Texture.SampleLevel(Sampler, float2(0,0), 0).rgb);
    return output;
}*/

struct PSInput
{
	float4 color : COLOR;
    float2 uv : UV;
};

cbuffer CB : register(b0) {
    float4 A[10];
}

Texture2D T : register(t0);
SamplerState S : register(s0);

float2 vs(PSInput input) : SV_TARGET0
{
    int a = input.uv.y * int(input.uv.x);
    int b = input.uv.y * int(input.uv.x);
	float4 c = T.SampleLevel(S, A[a + b + 2].yx, 0);
    c *= b;
    return c;
}
