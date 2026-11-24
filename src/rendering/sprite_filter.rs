use macroquad::{miniquad::*, prelude::*};

const VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
uniform mat4 Model;
uniform mat4 Projection;
varying highp vec2 uv;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
"#;

const FRAGMENT: &str = r#"#version 100
precision highp float;
uniform sampler2D Texture;
uniform vec2 AtlasSize;
uniform vec4 SpriteBounds;
varying highp vec2 uv;

vec4 sample_sprite(vec2 texel) {
    vec2 center = clamp(texel, SpriteBounds.xy, SpriteBounds.zw) + 0.5;
    vec4 color = texture2D(Texture, center / AtlasSize);
    return vec4(color.rgb * color.a, color.a);
}

void main() {
    vec2 texel = uv * AtlasSize - 0.5;
    vec2 base = floor(texel);
    vec2 weight = fract(texel);
    vec4 top = mix(sample_sprite(base), sample_sprite(base + vec2(1.0, 0.0)), weight.x);
    vec4 bottom = mix(sample_sprite(base + vec2(0.0, 1.0)),
                      sample_sprite(base + vec2(1.0, 1.0)), weight.x);
    gl_FragColor = mix(top, bottom, weight.y);
}
"#;

pub(super) fn load() -> Result<Material, macroquad::Error> {
    load_material(
        ShaderSource::Glsl {
            vertex: VERTEX,
            fragment: FRAGMENT,
        },
        MaterialParams {
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
            uniforms: vec![
                UniformDesc::new("AtlasSize", UniformType::Float2),
                UniformDesc::new("SpriteBounds", UniformType::Float4),
            ],
            ..Default::default()
        },
    )
}

pub(super) fn begin(material: &Material, atlas: &Texture2D, source: Rect) {
    material.set_uniform("AtlasSize", atlas.size());
    material.set_uniform(
        "SpriteBounds",
        vec4(
            source.x,
            source.y,
            source.x + source.w - 1.,
            source.y + source.h - 1.,
        ),
    );
    gl_use_material(material);
}
