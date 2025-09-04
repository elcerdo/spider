use crate::track::piece::prepare_track;
use crate::track::piece::{CornerData, StraightData, Track, TrackData, TrackPiece};

use bevy::asset::{Assets, weak_handle};
use bevy::math::Vec3;

use bevy::prelude::info;
use bevy::prelude::{Handle, ResMut};

//////////////////////////////////////////////////////////////////////

const TRACK_ADVANCED_WW: f32 = 1.8;

const TRACK_ADVANCED_STRAIGHT: StraightData =
    StraightData::from_left_right_length(-TRACK_ADVANCED_WW, TRACK_ADVANCED_WW, 4.0);
const TRACK_ADVANCED_STRAIGHT_DOUBLE: StraightData =
    StraightData::from_left_right_length(-TRACK_ADVANCED_WW, TRACK_ADVANCED_WW, 8.0);
const TRACK_ADVANCED_STRAIGHT_TRIPLE: StraightData =
    StraightData::from_left_right_length(-TRACK_ADVANCED_WW, TRACK_ADVANCED_WW, 12.0);

static TRACK_ADVANCED_PIECES: &[TrackPiece] = &[
    TrackPiece::Start,
    TrackPiece::Straight(TRACK_ADVANCED_STRAIGHT_DOUBLE),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(TRACK_ADVANCED_STRAIGHT),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(TRACK_ADVANCED_STRAIGHT),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(TRACK_ADVANCED_STRAIGHT_DOUBLE),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(TRACK_ADVANCED_STRAIGHT_TRIPLE),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Straight(TRACK_ADVANCED_STRAIGHT),
    TrackPiece::Finish,
];

static TRACK_ADVANCED_DATA: TrackData = TrackData {
    pieces: TRACK_ADVANCED_PIECES,
    initial_position: Vec3::new(-8.0, 0.0, 0.0),
    initial_forward: Vec3::Z,
    initial_up: Vec3::Y,
    initial_left: -TRACK_ADVANCED_WW,
    initial_right: TRACK_ADVANCED_WW,
    num_segments: 10,
};

pub const TRACK_ADVANCED_HANDLE: Handle<Track> =
    weak_handle!("1347c9b7-c46a-48e7-0000-023a354b7cac");

//////////////////////////////////////////////////////////////////////

static TRACK_BEGINNER_PIECES: &[TrackPiece] = &[
    TrackPiece::Start,
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_length(8.0)),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_length(14.0)),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(11.0)),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(3.0)),
    TrackPiece::Finish,
];

static TRACK_BEGINNER_DATA: TrackData = TrackData {
    pieces: TRACK_BEGINNER_PIECES,
    initial_position: Vec3::new(-3.0, 0.0, -2.0),
    initial_forward: Vec3::Z,
    initial_up: Vec3::Y,
    initial_left: -1.0,
    initial_right: 1.0,
    num_segments: 4,
};

pub const TRACK_BEGINNER_HANDLE: Handle<Track> =
    weak_handle!("1347c9b7-c46a-48e7-1111-023a354b7cac");

//////////////////////////////////////////////////////////////////////

static TRACK_VERTICAL_PIECES: &[TrackPiece] = &[
    TrackPiece::Start,
    TrackPiece::Straight(StraightData::from_length(5.0)),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_left_right(-1.0, 0.5)),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::default()),
    TrackPiece::Checkpoint,
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Straight(StraightData::from_length(2.0)),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_left_right(-2.0, 1.0)),
    TrackPiece::Straight(StraightData::from_left_right_length(-2.0, 1.0, 4.0)),
    TrackPiece::Layer(2),
    TrackPiece::Corner(CornerData::right_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_left_right(-1.0, 1.0)),
    TrackPiece::Straight(StraightData::from_length(8.0)),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_left_right(-1.0, 2.0)),
    TrackPiece::Layer(0),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Straight(StraightData::from_left_right_length(-1.0, 2.0, 1.0)),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_left_right_length(-1.0, 2.0, 6.0)),
    TrackPiece::Corner(CornerData::left_turn()),
    TrackPiece::Checkpoint,
    TrackPiece::Straight(StraightData::from_left_right_length(-2.0, 1.0, 6.0)),
    TrackPiece::Finish,
];

static TRACK_VERTICAL_DATA: TrackData = TrackData {
    pieces: TRACK_VERTICAL_PIECES,
    initial_position: Vec3::new(0.0, 0.0, 2.0),
    initial_forward: Vec3::X,
    initial_up: Vec3::Y,
    initial_left: -2.0,
    initial_right: 1.0,
    num_segments: 10,
};

pub const TRACK_VERTICAL_HANDLE: Handle<Track> =
    weak_handle!("1347c9b7-c46a-48e7-2222-023a354b7cac");

//////////////////////////////////////////////////////////////////////

pub const TRACK_HANDLES: &[Handle<Track>] = &[
    TRACK_BEGINNER_HANDLE,
    TRACK_VERTICAL_HANDLE,
    TRACK_ADVANCED_HANDLE,
];

pub fn prepare_tracks(mut tracks: ResMut<Assets<Track>>) {
    info!("** prepare_tracks **");
    tracks.insert(&TRACK_BEGINNER_HANDLE, prepare_track(&TRACK_BEGINNER_DATA));
    tracks.insert(&TRACK_VERTICAL_HANDLE, prepare_track(&TRACK_VERTICAL_DATA));
    tracks.insert(&TRACK_ADVANCED_HANDLE, prepare_track(&TRACK_ADVANCED_DATA));
}
