use bevy::prelude::*;

// #[derive(Component, Clone, Debug, Copy, PartialEq, Eq, Hash)]
// pub enum TrackNickname {
//     Beginner,
//     Vertical,
//     Advanced,
// }

// pub const TRACK_NICKNAMES: &[TrackNickname] = &[
//     TrackNickname::Beginner,
//     TrackNickname::Vertical,
//     TrackNickname::Advanced,
// ];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GlobalState {
    #[default]
    Init,
    Ready,
    // TrackSelectionInit,
    // TrackSelectionIdle,
    // TrackSelectionHoovered(TrackNickname),
    // TrackSelected(TrackNickname),
    // InGame(TrackNickname),
    // GameDone,
}

pub struct GlobalStatePlugin;

impl Plugin for GlobalStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GlobalState>();
        app.add_systems(
            Update,
            dump_global_state.run_if(state_changed::<GlobalState>),
        );
    }
    fn finish(&self, app: &mut App) {
        app.insert_state(GlobalState::Ready);
    }
}

fn dump_global_state(state: Res<State<GlobalState>>) {
    info!("!!!! {:?} !!!!", state.get());
}
