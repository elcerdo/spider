use bevy::asset::Asset;
use bevy::math::{ops, FloatPow, NormedVectorSpace};
use bevy::math::{Mat2, Vec2, Vec3};
use bevy::reflect::TypePath;
use bevy::render::mesh::Mesh;

use kd_tree::{KdPoint, KdTree};

use std::collections::HashMap;

use std::f32::consts::PI;

pub enum TrackPiece {
    Start,
    Straight(StraightData),
    Corner(CornerData),
    Checkpoint,
    Layer(u8),
    Finish,
}

#[derive(Debug)]
pub struct StraightData {
    left: f32,
    right: f32,
    length: f32,
    num_quads: u32,
}

impl StraightData {
    pub const fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            length: 2.0,
            num_quads: 8,
        }
    }
    pub const fn from_length(length: f32) -> Self {
        Self {
            length,
            num_quads: if 2.0 * length < 1.0 {
                1
            } else {
                (4.0 * length) as u32
            },
            ..StraightData::default()
        }
    }
    pub const fn from_left_right(left: f32, right: f32) -> Self {
        Self {
            left,
            right,
            ..StraightData::default()
        }
    }
    pub const fn from_left_right_length(left: f32, right: f32, length: f32) -> Self {
        Self {
            left,
            right,
            length,
            num_quads: if 2.0 * length < 1.0 {
                1
            } else {
                (4.0 * length) as u32
            },
        }
    }
}

#[derive(Debug)]
pub struct CornerData {
    radius: f32,
    angle: f32,
    num_quads: u32,
}

impl CornerData {
    pub const fn right_turn() -> Self {
        Self {
            radius: 2.0,
            angle: PI / 2.0,
            num_quads: 32,
        }
    }
    pub const fn left_turn() -> Self {
        Self {
            radius: -2.0,
            angle: PI / 2.0,
            num_quads: 32,
        }
    }
}

pub struct TrackData {
    pub pieces: &'static [TrackPiece],
    pub initial_position: Vec3,
    pub initial_forward: Vec3,
    pub initial_up: Vec3,
    pub initial_left: f32,
    pub initial_right: f32,
    pub num_segments: u32,
}

#[derive(PartialEq)]
enum Align {
    Left,
    Collinear,
    Right,
}

impl Align {
    fn from_triplet(xx: &Vec2, yy: &Vec2, zz: &Vec2) -> Self {
        let xy = yy - xx;
        let xz = zz - xx;
        let cross = xy.x * xz.y - xy.y * xz.x;
        if ops::abs(cross) < 1e-7 {
            return Align::Collinear;
        };
        if cross > 0.0 {
            Align::Right
        } else {
            Align::Left
        }
    }
}

pub struct Segment {
    aa: Vec2,
    bb: Vec2,
    pub ii: u8,
}

impl Default for Segment {
    fn default() -> Self {
        Self {
            aa: Vec2::ZERO,
            bb: Vec2::ZERO,
            ii: 255,
        }
    }
}

impl KdPoint for Segment {
    type Scalar = f32;
    type Dim = typenum::U4; // 4 dimensional tree.
    fn at(&self, kk: usize) -> f32 {
        match kk {
            0 => self.aa.x,
            1 => self.bb.x,
            2 => self.aa.y,
            3 => self.bb.y,
            _ => unreachable!(),
        }
    }
}

impl Segment {
    pub fn from_endpoints(aa: Vec2, bb: Vec2) -> Self {
        Self { aa, bb, ii: 255 }
    }

    pub fn intersects(&self, qq: &Self) -> bool {
        let pp = self;
        let qa = Align::from_triplet(&pp.aa, &pp.bb, &qq.aa);
        let qb = Align::from_triplet(&pp.aa, &pp.bb, &qq.bb);
        let pa = Align::from_triplet(&qq.aa, &qq.bb, &pp.aa);
        let pb = Align::from_triplet(&qq.aa, &qq.bb, &pp.bb);
        qa != qb && pa != pb
    }

    pub fn clips(&self, qq: &Self) -> bool {
        let pp = self;
        let qa = Align::from_triplet(&pp.aa, &pp.bb, &qq.aa);
        let qb = Align::from_triplet(&pp.aa, &pp.bb, &qq.bb);
        qa == Align::Left || qb == Align::Left
    }

    pub fn mirror(&self, xx: Vec2) -> Vec2 {
        let ee = (self.bb - self.aa).normalize();
        let ff = Vec2::new(-ee.y, ee.x);
        let mut mm = Mat2::from_cols(ee, ff).transpose();
        mm = mm.transpose() * Mat2::from_diagonal(Vec2::new(1.0, -1.0)) * mm;
        self.aa + mm * (xx - self.aa)
    }
}

#[derive(Default)]
pub struct Collision {
    pub track_kdtree: KdTree<Segment>,
    pub checkpoint_kdtree: KdTree<Segment>,
    pub transition_kdtree: KdTree<Segment>,
}

#[derive(Asset, TypePath)]
pub struct Track {
    pub track: Mesh,
    pub checkpoint: Mesh,
    pub total_length: f32,
    pub is_looping: bool,
    pub layer_to_collisions: HashMap<u8, Collision>,
    pub checkpoint_count: u8,
    pub initial_up: Vec3,
    pub initial_position: Vec3,
    pub initial_forward: Vec3,
    pub initial_left: f32,
    pub initial_right: f32,
}

pub fn prepare_track(track_data: &TrackData) -> Track {
    use bevy::prelude::*;

    assert!(ops::abs(track_data.initial_forward.norm() - 1.0) < 1e-5);
    assert!(ops::abs(track_data.initial_up.norm() - 1.0) < 1e-5);
    assert!(track_data.initial_left < track_data.initial_right);
    assert!(track_data.num_segments > 0);
    assert!(track_data.pieces.len() >= 2);
    match &track_data.pieces[0] {
        TrackPiece::Start => {}
        _ => panic!("!!! first piece should be a start !!!"),
    }
    match &track_data.pieces[track_data.pieces.len() - 1] {
        TrackPiece::Finish => {}
        _ => panic!("!!! last piece should be a finish !!!"),
    }

    let initial_righthand = track_data.initial_forward.cross(track_data.initial_up);

    let mut checkpoint_positions: Vec<Vec3> = vec![];
    let mut checkpoint_normals: Vec<Vec3> = vec![];
    let mut checkpoint_triangles: Vec<u32> = vec![];
    let mut push_checkpoint_gate =
        |position: &Vec3, forward: &Vec3, left: f32, right: f32| -> u32 {
            const WIDTH: f32 = 0.2;
            const EPSILON: f32 = 1e-3;
            let righthand = forward.cross(track_data.initial_up);
            let aa = position + righthand * left - WIDTH * forward / 2.0
                + EPSILON * track_data.initial_up;
            let bb = position + righthand * right - WIDTH * forward / 2.0
                + EPSILON * track_data.initial_up;
            let cc = aa + WIDTH * forward;
            let dd = bb + WIDTH * forward;
            let next_vertex = checkpoint_positions.len() as u32;
            checkpoint_positions.push(aa);
            checkpoint_positions.push(bb);
            checkpoint_positions.push(cc);
            checkpoint_positions.push(dd);
            checkpoint_normals.push(track_data.initial_up);
            checkpoint_normals.push(track_data.initial_up);
            checkpoint_normals.push(track_data.initial_up);
            checkpoint_normals.push(track_data.initial_up);
            let mut tri_aa = vec![next_vertex, next_vertex + 1, next_vertex + 2];
            let mut tri_bb = vec![next_vertex + 2, next_vertex + 1, next_vertex + 3];
            checkpoint_triangles.append(&mut tri_aa);
            checkpoint_triangles.append(&mut tri_bb);
            next_vertex
        };

    let mut checkpoint_layer_to_segments: HashMap<u8, Vec<Segment>> = HashMap::new();
    let mut checkpoint_count: u8 = 0;
    let mut push_checkpoint_segment =
        |position: &Vec3, forward: &Vec3, left: f32, right: f32, layer: u8| -> u8 {
            let righthand = forward.cross(track_data.initial_up);
            let aa = position + righthand * left;
            let bb = position + righthand * right;
            let ii = checkpoint_count;
            checkpoint_layer_to_segments.entry(layer).or_default();
            let checkpoint_segments = checkpoint_layer_to_segments.get_mut(&layer).unwrap();
            checkpoint_segments.push(Segment {
                aa: aa.xz(),
                bb: bb.xz(),
                ii,
            });
            checkpoint_count += 1;
            ii
        };

    let mut transition_layer_to_segments: HashMap<u8, Vec<Segment>> = HashMap::new();
    let mut transition_count: u8 = 0;
    let mut push_transition_segment = |position: &Vec3,
                                       forward: &Vec3,
                                       left: f32,
                                       right: f32,
                                       from_layer: u8,
                                       to_layer: u8|
     -> u8 {
        assert!(from_layer != to_layer);
        let righthand = forward.cross(track_data.initial_up);
        let aa = position + righthand * left;
        let bb = position + righthand * right;
        let ii = transition_count;
        transition_layer_to_segments.entry(from_layer).or_default();
        transition_layer_to_segments.entry(to_layer).or_default();
        {
            let from_transition_segments =
                transition_layer_to_segments.get_mut(&from_layer).unwrap();
            from_transition_segments.push(Segment {
                aa: aa.xz(),
                bb: bb.xz(),
                ii: to_layer,
            });
        }
        {
            let to_transition_segments = transition_layer_to_segments.get_mut(&to_layer).unwrap();
            to_transition_segments.push(Segment {
                aa: aa.xz(),
                bb: bb.xz(),
                ii: from_layer,
            });
        }
        transition_count += 1;
        ii
    };

    let mut track_positions: Vec<Vec3> = vec![];
    let mut track_normals: Vec<Vec3> = vec![];
    let mut track_triangles: Vec<u32> = vec![];
    let mut track_uvs: Vec<Vec2> = vec![];
    let mut track_pqs: Vec<Vec2> = vec![];
    let mut track_layer_to_segments: HashMap<u8, Vec<Segment>> = HashMap::new();
    let mut push_track_section =
        |position: &Vec3, forward: &Vec3, left: f32, right: f32, length: f32, layer: u8| -> u32 {
            let left_pos = position + forward.cross(track_data.initial_up) * left;
            let right_pos = position + forward.cross(track_data.initial_up) * right;
            let next_vertex = track_positions.len() as u32;
            let num_segments = track_data.num_segments;
            assert!(next_vertex % (num_segments + 1) == 0);
            for kk in 0..=num_segments {
                let aa = kk as f32 / num_segments as f32;
                assert!(aa >= 0.0);
                assert!(aa <= 1.0);
                let pos = aa * right_pos + (1.0 - aa) * left_pos;
                let uv = Vec2::new(aa * right + (1.0 - aa) * left, length);
                let proj =
                    Mat3::from_cols(initial_righthand, track_data.initial_forward, Vec3::ZERO)
                        .transpose();
                let pq = proj * (pos - track_data.initial_position);
                assert!(ops::abs(pq.z) < 1e-5);
                track_positions.push(pos);
                track_normals.push(track_data.initial_up);
                track_uvs.push(uv);
                track_pqs.push(pq.xy());
            }
            if next_vertex != 0 {
                assert!(next_vertex >= (num_segments + 1));
                for kk in 0..num_segments {
                    let mut tri_aa = vec![
                        next_vertex + kk - num_segments - 1,
                        next_vertex + kk - num_segments,
                        next_vertex + kk,
                    ];
                    // let mut tri_bb = vec![next_vertex - 1, next_vertex + 1, next_vertex];
                    let mut tri_bb = vec![
                        next_vertex + kk - num_segments,
                        next_vertex + kk + 1,
                        next_vertex + kk,
                    ];
                    track_triangles.append(&mut tri_aa);
                    track_triangles.append(&mut tri_bb);
                }
                let left_index_ = (next_vertex - num_segments - 1) as usize;
                let right_index_ = (next_vertex - 1) as usize;
                let left_pos_ = track_positions[left_index_];
                let right_pos_ = track_positions[right_index_];
                track_layer_to_segments.entry(layer).or_default();
                let track_segments = track_layer_to_segments.get_mut(&layer).unwrap();
                track_segments.push(Segment {
                    aa: left_pos_.xz(),
                    bb: left_pos.xz(),
                    ii: 0,
                });
                track_segments.push(Segment {
                    aa: right_pos.xz(),
                    bb: right_pos_.xz(),
                    ii: 1,
                });
            }
            next_vertex
        };

    let mut current_position = track_data.initial_position;
    let mut current_forward = track_data.initial_forward;
    let mut current_length: f32 = 0.0;
    let mut is_looping: bool = false;
    let mut current_left: f32 = track_data.initial_left;
    let mut current_right: f32 = track_data.initial_right;
    let mut current_layer: u8 = 0;
    for piece in track_data.pieces {
        match piece {
            TrackPiece::Start => {
                debug!("Start {:?}", current_position.clone());
                assert!(current_length == 0.0);
                assert!(current_left == track_data.initial_left);
                assert!(current_right == track_data.initial_right);
                assert!(current_left < current_right);
                let section_index = push_track_section(
                    &current_position,
                    &current_forward,
                    current_left,
                    current_right,
                    current_length,
                    current_layer,
                );
                assert!(section_index == 0);
                let section_index_ = push_checkpoint_segment(
                    &current_position,
                    &current_forward,
                    current_left,
                    current_right,
                    current_layer,
                );
                assert!(section_index_ == 0);
            }
            TrackPiece::Straight(data) => {
                debug!("Straight {:?} {:?}", current_position.clone(), data);
                assert!(current_left < current_right);
                assert!(data.num_quads > 0);
                for kk in 0..data.num_quads {
                    let aa = (kk + 1) as f32 / data.num_quads as f32;
                    assert!(aa > 0.0);
                    assert!(aa <= 1.0);
                    let bb = 3.0 * aa.squared() - 2.0 * aa.cubed();
                    assert!(bb > 0.0);
                    assert!(bb <= 1.0);
                    let pos = current_position + current_forward * aa * data.length;
                    let len = current_length + aa * data.length;
                    let section_index = push_track_section(
                        &pos,
                        &current_forward,
                        current_left * (1.0 - bb) + data.left * bb,
                        current_right * (1.0 - bb) + data.right * bb,
                        len,
                        current_layer,
                    );
                    assert!(section_index > 0);
                }
                current_position += current_forward * data.length;
                current_length += data.length;
                assert!(current_length != 0.0);
                current_left = data.left;
                current_right = data.right;
                assert!(current_left < current_right);
            }
            TrackPiece::Corner(data) => {
                debug!("Corner {:?} {:?}", current_position.clone(), data);
                assert!(current_left < current_right);
                assert!(data.num_quads > 0);
                let current_righthand = current_forward.cross(track_data.initial_up);
                let center = current_position + current_righthand * data.radius;
                let sign: f32 = if data.radius < 0.0 { 1.0 } else { -1.0 };
                for kk in 0..data.num_quads {
                    let angle = (kk + 1) as f32 / data.num_quads as f32 * data.angle;
                    let pos = center + current_forward * ops::abs(data.radius) * ops::sin(angle)
                        - current_righthand * data.radius * ops::cos(angle);
                    let quat = Quat::from_axis_angle(track_data.initial_up, sign * angle);
                    let fwd = quat * current_forward;
                    let len = ops::abs(data.radius) * angle + current_length;
                    let section_index = push_track_section(
                        &pos,
                        &fwd,
                        current_left,
                        current_right,
                        len,
                        current_layer,
                    );
                    assert!(section_index > 0);
                }
                current_position = center
                    + current_forward * ops::abs(data.radius) * ops::sin(data.angle)
                    - current_righthand * data.radius * ops::cos(data.angle);
                let quat = Quat::from_axis_angle(track_data.initial_up, sign * data.angle);
                current_forward = quat * current_forward;
                current_length += ops::abs(data.radius) * data.angle;
                assert!(current_length != 0.0);
            }
            TrackPiece::Finish => {
                let pos_error = (current_position - track_data.initial_position).norm();
                let dir_error = (current_forward - track_data.initial_forward).norm();
                let left_error = ops::abs(current_left - track_data.initial_left);
                let right_error = ops::abs(current_right - track_data.initial_right);
                let layer_ok = current_layer == 0;
                let eps: f32 = 1e-3;
                is_looping = pos_error < eps
                    && dir_error < eps
                    && left_error < eps
                    && right_error < eps
                    && current_length > 0.0
                    && layer_ok;
                debug!(
                    "Finish {:?} pos_err {:0.3e} dir_err {:0.3e} total_length {} layer {} loop {}",
                    current_position.clone(),
                    pos_error,
                    dir_error,
                    current_length,
                    current_layer,
                    is_looping,
                );
            }
            TrackPiece::Checkpoint => {
                push_checkpoint_gate(
                    &current_position,
                    &current_forward,
                    current_left,
                    current_right,
                );
                let section_index_ = push_checkpoint_segment(
                    &current_position,
                    &current_forward,
                    current_left,
                    current_right,
                    current_layer,
                );
                debug!("Checkpoint {}", section_index_);
            }
            TrackPiece::Layer(layer) => {
                debug!("Layer {} -> {}", current_layer, *layer);
                push_transition_segment(
                    &current_position,
                    &current_forward,
                    current_left,
                    current_right,
                    current_layer,
                    *layer,
                );
                current_layer = *layer;
            }
        }
    }

    assert!(checkpoint_triangles.len() % 3 == 0);
    assert!(track_triangles.len() % 3 == 0);
    debug!("num_vertices {}", track_positions.len());
    debug!("num_triangles {}", track_triangles.len() / 3);
    debug!("total_length {}", current_length);
    if !is_looping {
        warn!("!!! road is not looping !!!");
    }

    use bevy::render::mesh::Indices;
    use bevy::render::mesh::Mesh;
    use bevy::render::render_asset::RenderAssetUsages;
    use bevy::render::render_resource::PrimitiveTopology;

    let mut track = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    track = track.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, track_positions);
    track = track.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, track_normals);
    track = track.with_inserted_indices(Indices::U32(track_triangles));
    track = track.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, track_uvs);
    track = track.with_inserted_attribute(Mesh::ATTRIBUTE_UV_1, track_pqs);
    track = track.with_generated_tangents().unwrap();

    let mut checkpoint = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    checkpoint = checkpoint.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, checkpoint_positions);
    checkpoint = checkpoint.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, checkpoint_normals);
    checkpoint = checkpoint.with_inserted_indices(Indices::U32(checkpoint_triangles));
    // checkpoint = checkpoint.with_generated_tangents().unwrap();

    let mut layer_to_collisions = HashMap::new();
    for (section, track_segments) in track_layer_to_segments {
        layer_to_collisions
            .entry(section)
            .or_insert_with(Collision::default);
        let collision = layer_to_collisions.get_mut(&section).unwrap();
        collision.track_kdtree = KdTree::build_by_ordered_float(track_segments);
    }
    for (section, checkpoint_segments) in checkpoint_layer_to_segments {
        layer_to_collisions
            .entry(section)
            .or_insert_with(Collision::default);
        let collision = layer_to_collisions.get_mut(&section).unwrap();
        collision.checkpoint_kdtree = KdTree::build_by_ordered_float(checkpoint_segments);
    }
    for (section, transition_segments) in transition_layer_to_segments {
        layer_to_collisions
            .entry(section)
            .or_insert_with(Collision::default);
        let collision = layer_to_collisions.get_mut(&section).unwrap();
        collision.transition_kdtree = KdTree::build_by_ordered_float(transition_segments);
    }

    Track {
        track,
        checkpoint,
        total_length: current_length,
        is_looping,
        checkpoint_count,
        layer_to_collisions,
        initial_up: track_data.initial_up,
        initial_position: track_data.initial_position,
        initial_forward: track_data.initial_forward,
        initial_left: track_data.initial_left,
        initial_right: track_data.initial_right,
    }
}
