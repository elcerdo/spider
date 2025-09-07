use bevy::prelude::*;

const MODEL_SPIDER_PATH: &str = "models/tachikoma.glb";
const MODEL_SPIDER_SCALE: f32 = 1.0;

pub struct SpiderPlugin;

impl Plugin for SpiderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, populate_spider);
    }
}

//////////////////////////////////////////////////////////////////////

#[derive(Component)]
struct SpiderMarker;

fn populate_spider(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
    // tracks: Res<Assets<Track>>,
    // state: Res<State<GlobalState>>,
) {
    let scene: Handle<Scene> = server.load(GltfAssetLabel::Scene(0).from_asset(MODEL_SPIDER_PATH));

    commands.spawn((
        SpiderMarker,
        SceneRoot(scene.clone()),
        // BoatData::from_player_position_and_forward(
        //     Player::One,
        //     pos_p1.xz(),
        //     track.initial_forward.xz(),
        // ),
        Transform::from_translation(Vec3::Z * 0.0),
    ));

    // info!("scene\n{:?}", scene);
}
