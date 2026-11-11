use bevy::asset::AssetServer;
use bevy::prelude::Res;

pub fn make(asset_server: Res<AssetServer>, scale: f32) -> bevy::pbr::StandardMaterial {
    use bevy::image::ImageAddressMode;
    use bevy::image::ImageLoaderSettings;
    use bevy::image::ImageSampler;
    use bevy::image::ImageSamplerDescriptor;
    use bevy::math::Affine2;
    use bevy::math::Vec2;
    use bevy::mesh::UvChannel;
    bevy::pbr::StandardMaterial {
        perceptual_roughness: 0.2,
        base_color_channel: UvChannel::Uv1,
        base_color_texture: Some(
            asset_server
                .load_builder()
                .with_settings(|settings: &mut ImageLoaderSettings| {
                    *settings = ImageLoaderSettings {
                        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                            address_mode_u: ImageAddressMode::Repeat,
                            address_mode_v: ImageAddressMode::Repeat,
                            ..ImageSamplerDescriptor::default()
                        }),
                        ..ImageLoaderSettings::default()
                    }
                })
                .load("textures/parallax_example/cube_color.png"),
        ),
        normal_map_channel: UvChannel::Uv1,
        normal_map_texture: Some(
            asset_server
                .load_builder()
                // The normal map texture is in linear color space. Lighting won't look correct
                // if `is_srgb` is `true`, which is the default.
                .with_settings(|settings: &mut ImageLoaderSettings| {
                    *settings = ImageLoaderSettings {
                        is_srgb: false,
                        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                            address_mode_u: ImageAddressMode::Repeat,
                            address_mode_v: ImageAddressMode::Repeat,
                            ..ImageSamplerDescriptor::default()
                        }),
                        ..ImageLoaderSettings::default()
                    }
                })
                .load("textures/parallax_example/cube_normal.png"),
        ),
        depth_map: Some(
            asset_server
                .load_builder()
                .with_settings(|settings: &mut ImageLoaderSettings| {
                    *settings = ImageLoaderSettings {
                        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                            address_mode_u: ImageAddressMode::Repeat,
                            address_mode_v: ImageAddressMode::Repeat,
                            ..ImageSamplerDescriptor::default()
                        }),
                        ..ImageLoaderSettings::default()
                    }
                })
                .load("textures/parallax_example/cube_depth.png"),
        ),
        parallax_depth_scale: 0.1,
        uv_transform: Affine2::from_scale(Vec2::ONE * scale),
        ..bevy::pbr::StandardMaterial::default()
    }
}
