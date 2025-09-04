use bevy::asset::{Asset, AssetServer, Assets};
use bevy::color::Srgba;
use bevy::math::Vec2;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use bevy::prelude::MeshMaterial3d;
use bevy::prelude::{Component, Handle, Query, Res, ResMut, Time, With};

const COLOR_START_LINE: Srgba = bevy::color::palettes::basic::WHITE;
const SHADER_PATH: &str = "shaders/racing_line.wgsl";

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RacingLineMaterial {
    #[texture(0)]
    #[sampler(1)]
    color_texture: Option<Handle<bevy::image::Image>>,
    #[uniform(2)]
    color: bevy::prelude::LinearRgba,
    #[uniform(3)]
    track_length: f32,
    #[uniform(4)]
    pub middle_line_width: f32,
    #[uniform(5)]
    start_line_width: f32,
    #[uniform(6)]
    time: f32,
    #[uniform(7)]
    pub cursor_position: Vec2,
    #[uniform(8)]
    cursor_radius: f32,
    #[uniform(9)]
    pub lateral_range: Vec2,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl bevy::prelude::Material for RacingLineMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> bevy::prelude::AlphaMode {
        bevy::prelude::AlphaMode::Blend
    }
}

pub fn make(asset_server: &Res<AssetServer>, track_length: f32) -> RacingLineMaterial {
    use bevy::color::LinearRgba;
    use bevy::image::ImageAddressMode;
    use bevy::image::ImageLoaderSettings;
    use bevy::image::ImageSampler;
    use bevy::image::ImageSamplerDescriptor;
    RacingLineMaterial {
        track_length,
        middle_line_width: 0.2,
        start_line_width: 0.2,
        lateral_range: Vec2::new(-0.8, 0.8),
        time: 0.0,
        cursor_position: Vec2::ZERO,
        cursor_radius: 0.4,
        color: LinearRgba::from(COLOR_START_LINE),
        color_texture: Some(asset_server.load_with_settings(
            "textures/slice_square.png",
            |settings: &mut ImageLoaderSettings| {
                *settings = ImageLoaderSettings {
                    sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        ..ImageSamplerDescriptor::default()
                    }),
                    ..ImageLoaderSettings::default()
                }
            },
        )),
    }
}

#[derive(Component)]
pub struct AnimatedRacingLineMarker;

pub fn animate(
    material_handles: Query<&MeshMaterial3d<RacingLineMaterial>, With<AnimatedRacingLineMarker>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<RacingLineMaterial>>,
) {
    for material_handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.time += time.delta_secs();
        }
    }
}
