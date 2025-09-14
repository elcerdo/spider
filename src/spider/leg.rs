use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

use std::collections::BTreeMap;

pub const SPIDER_LEG_LENGTH: f32 = 3.5;
const SPIDER_STEP_LENGTH: f32 = 1.0;
const SPIDER_STEP_LEAD: f32 = 0.25;

#[derive(Clone)]
pub struct SpiderLeg {
    pub parent: Entity,
    marker: Entity,
    entity: Entity,
}

#[derive(Component)]
pub struct SpiderAnimation {
    pub graph: Handle<AnimationGraph>,
    pub index: AnimationNodeIndex,
    pub legs: BTreeMap<(String, String), SpiderLeg>,
}

impl SpiderAnimation {
    pub fn from_anim(graph: Handle<AnimationGraph>, index: AnimationNodeIndex) -> Self {
        Self {
            graph,
            index,
            legs: BTreeMap::new(),
        }
    }
}

pub fn populate_legs(
    trigger: Trigger<SceneInstanceReady>,
    mut animation: Single<&mut SpiderAnimation>,
    children: Query<&Children>,
    names: Query<&Name>,
    parents: Query<&ChildOf>,
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("** populate legs **");

    let re = regex::Regex::new(r"^leg_(left|right)_(front|mid|back)$").unwrap();
    let target = trigger.target();

    #[cfg(feature = "debug_gizmos")]
    let block = {
        let mesh = Cuboid::new(0.5, 0.5, SPIDER_LEG_LENGTH);
        let material = StandardMaterial {
            base_color: RED.into(),
            emissive: RED.into(),
            ..default()
        };
        let mesh = _meshes.add(mesh);
        let material = _materials.add(material);

        (
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(0.0, 0.0, SPIDER_LEG_LENGTH / 2.0),
        )
    };

    for entity in children.iter_descendants(target) {
        if let Ok(entity_name) = names.get(entity) {
            if let Some(groups) = re.captures(entity_name) {
                let key: (String, String) = (groups[1].into(), groups[2].into());

                #[cfg(feature = "debug_gizmos")]
                let marker = {
                    let mut marker = commands.spawn((Visibility::Visible, Transform::IDENTITY));
                    marker.with_child(block.clone());
                    marker.id()
                };

                #[cfg(not(feature = "debug_gizmos"))]
                let marker = commands
                    .spawn((Visibility::Visible, Transform::IDENTITY))
                    .id();

                let ChildOf(parent) = parents.get(entity).unwrap();
                let parent = *parent;

                let value = SpiderLeg {
                    parent,
                    marker,
                    entity,
                };

                let parent_name = names.get(parent).unwrap();
                info!(
                    "{:?} -> ({}, {:?}, {})",
                    key.clone(),
                    parent_name,
                    marker,
                    entity_name,
                );

                animation.legs.insert(key.clone(), value.clone());
            }
        }
    }

    assert!(animation.legs.len() == 6);

    for leg in animation.legs.values() {
        let mut leg_commands = commands.entity(leg.entity);
        leg_commands.remove_parent_in_place();
        leg_commands.set_parent_in_place(leg.marker);
    }
}

pub fn update_legs(
    animations: Query<&SpiderAnimation>,
    global_transforms: Query<&GlobalTransform>,
    mut transforms: Query<&mut Transform>,
) {
    assert!(SPIDER_STEP_LEAD < SPIDER_STEP_LENGTH);
    for animation in animations.iter() {
        for leg in animation.legs.values() {
            let transform = global_transforms.get(leg.parent).unwrap();
            let pos = transform.transform_point(Vec3::Y * SPIDER_LEG_LENGTH);
            let pos__ = transform.transform_point(Vec3::ZERO);
            assert!((pos__ - transform.translation()).norm() < 1e-5);

            let mut transform_ = transforms.get_mut(leg.marker).unwrap();
            let pos_ = transform_.transform_point(Vec3::ZERO);
            assert!((pos_ - transform_.translation).norm() < 1e-5);

            let delta = pos - pos_;
            if delta.norm() > SPIDER_STEP_LENGTH {
                let lead = delta.normalize() * SPIDER_STEP_LEAD;
                transform_.translation = pos + lead;
            }

            let delta = pos__ - pos_;
            let angle = delta.zx().to_angle();
            transform_.rotation = Quat::from_axis_angle(Vec3::Y, angle);

            let mut transform__ = transforms.get_mut(leg.entity).unwrap();
            transform__.translation.y = 0.5;
            transform__.translation.z = 3.0;
        }
    }
}
