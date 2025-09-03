use crate::global_state::{GlobalState, TrackNickname};
use crate::track::{Segment, Track, TRACK_HANDLES};
use crate::vehicle::data::{BoatData, LapStat};

use bevy::prelude::warn;
use bevy::prelude::{Assets, Query, Res, State, Time};

pub fn bounce_and_resolve_checkpoints(
    mut boats: Query<&mut BoatData>,
    tracks: Res<Assets<Track>>,
    state: Res<State<GlobalState>>,
    time: Res<Time>,
) {
    let track = match state.get() {
        GlobalState::InGame(TrackNickname::Beginner) => tracks.get(&TRACK_HANDLES[0]),
        GlobalState::InGame(TrackNickname::Vertical) => tracks.get(&TRACK_HANDLES[1]),
        GlobalState::InGame(TrackNickname::Advanced) => tracks.get(&TRACK_HANDLES[2]),
        _ => unreachable!(),
    }
    .unwrap();

    assert!(track.is_looping);
    assert!(!track.layer_to_collisions.is_empty());

    let top_now = time.elapsed();

    // bounce track boundary
    for mut boat in &mut boats {
        assert!(track.layer_to_collisions.contains_key(&boat.layer));
        let collision = track.layer_to_collisions.get(&boat.layer).unwrap();
        assert!(!collision.track_kdtree.is_empty());
        let query_segment = Segment::from_endpoints(boat.position_current, boat.position_previous);
        let closest_segment = collision.track_kdtree.nearest(&query_segment).unwrap();
        assert!(query_segment.ii == 255);
        assert!(closest_segment.item.ii == 0 || closest_segment.item.ii == 1);
        if closest_segment.item.clips(&query_segment) {
            boat.position_previous = closest_segment.item.mirror(boat.position_previous);
            boat.position_current = closest_segment.item.mirror(boat.position_current);
        }
    }

    // resolve crossed checkpoints
    for mut boat in &mut boats {
        assert!(track.layer_to_collisions.contains_key(&boat.layer));
        let collision = track.layer_to_collisions.get(&boat.layer).unwrap();
        if collision.checkpoint_kdtree.is_empty() {
            continue;
        }
        assert!(!collision.checkpoint_kdtree.is_empty());
        let query_segment = Segment::from_endpoints(boat.position_current, boat.position_previous);
        let closest_segment = collision.checkpoint_kdtree.nearest(&query_segment).unwrap();
        assert!(query_segment.ii == 255);
        assert!(closest_segment.item.ii != 255);
        boat.current_stat.update(top_now);
        if closest_segment.item.intersects(&query_segment)
            && boat.current_stat.completed_lap(
                closest_segment.item.ii,
                track.checkpoint_count,
                top_now,
            )
        {
            assert!(boat.current_stat.is_valid());
            boat.last_stat = boat.current_stat.clone();
            boat.best_stat = match boat.best_stat.is_valid() {
                false => boat.current_stat.clone(),
                true => {
                    if boat.current_stat.lap_duration() < boat.best_stat.lap_duration() {
                        boat.current_stat.clone()
                    } else {
                        boat.best_stat.clone()
                    }
                }
            };
            boat.lap_count += 1;
            let is_new_best: bool = boat.best_stat.clone() == boat.last_stat.clone();
            warn!(
                "player {} completed lap {} in {:.3}{}",
                boat.player,
                boat.lap_count,
                boat.current_stat.lap_duration().as_secs_f32(),
                if is_new_best { " NEW BEST LAP !!!" } else { "" },
            );
            boat.current_stat = LapStat::from(top_now);
        }
    }

    // resolve layer transition
    for mut boat in &mut boats {
        assert!(track.layer_to_collisions.contains_key(&boat.layer));
        let collision = track.layer_to_collisions.get(&boat.layer).unwrap();
        if collision.transition_kdtree.is_empty() {
            continue;
        }
        assert!(!collision.transition_kdtree.is_empty());
        let query_segment = Segment::from_endpoints(boat.position_current, boat.position_previous);
        let closest_segment = collision.transition_kdtree.nearest(&query_segment).unwrap();
        assert!(query_segment.ii == 255);
        assert!(closest_segment.item.ii < 4, "max four layers");
        if closest_segment.item.intersects(&query_segment) {
            warn!(
                "player {} moved to layer {} from layer {}",
                boat.player, closest_segment.item.ii, boat.layer,
            );
            boat.layer = closest_segment.item.ii;
        }
    }
}
