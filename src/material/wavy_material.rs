use bevy::asset::{AssetServer, Assets};
use bevy::color::Srgba;
use bevy::math::{Affine2, Vec2};
use bevy::pbr::{StandardMaterial, UvChannel};

use bevy::prelude::MeshMaterial3d;
use bevy::prelude::{Component, Query, Res, ResMut, Time, With};

const COLOR_WAVY: Srgba = bevy::color::palettes::css::BLUE;

pub fn make(asset_server: &Res<AssetServer>) -> StandardMaterial {
    use bevy::color::Color;
    use bevy::image::ImageAddressMode;
    use bevy::image::ImageLoaderSettings;
    use bevy::image::ImageSampler;
    use bevy::image::ImageSamplerDescriptor;
    let make_tileable_and_linear = |settings: &mut ImageLoaderSettings| {
        *settings = ImageLoaderSettings {
            is_srgb: false,
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..ImageSamplerDescriptor::default()
            }),
            ..ImageLoaderSettings::default()
        }
    };
    let normal_map =
        asset_server.load_with_settings("textures/wavy_normals.png", make_tileable_and_linear);
    // let depth_map = asset_server.load_with_settings("textures/wavy_depth.png", make_tileable_and_linear);
    StandardMaterial {
        // parallax_depth_scale: 0.02,
        // metallic: 0.0,
        // depth_map: Some(depth_map),
        perceptual_roughness: 0.0,
        base_color: Color::from(COLOR_WAVY),
        normal_map_channel: UvChannel::Uv1,
        normal_map_texture: Some(normal_map),
        uv_transform: Affine2::from_scale(Vec2::new(1.0, 2.0) / 3.0),
        ..StandardMaterial::default()
    }
}

#[derive(Component)]
pub struct AnimatedWavyMarker;

pub fn animate(
    material_handles: Query<&MeshMaterial3d<StandardMaterial>, With<AnimatedWavyMarker>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for material_handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.uv_transform.translation.x += -0.04 * time.delta_secs();
        }
    }
}
