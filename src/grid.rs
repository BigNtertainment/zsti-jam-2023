use bevy::prelude::*;

use crate::GameState;

pub const GRID_CELL_SIZE: f32 = 64.;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GridPosition>().add_systems(
            Update,
            (update_positions, update_rotation).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct GridPosition {
    pub coords: IVec2,
    pub speed: f32,
}

impl Default for GridPosition {
    fn default() -> Self {
        Self {
            coords: IVec2::ZERO,
            speed: 50.,
        }
    }
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum Rotation {
    North,
    East,
    South,
    West,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct GridRotation {
    pub rotation: Rotation,
    pub speed: f32,
}

impl Default for GridRotation {
    fn default() -> Self {
        Self {
            rotation: Rotation::North,
            speed: 10.,
        }
    }
}

fn update_positions(mut query: Query<(&mut Transform, &GridPosition)>, time: Res<Time>) {
    for (mut transform, grid_position) in query.iter_mut() {
        let target = cell_position(&grid_position.coords);

        let position = transform.translation.truncate();

        if position != target {
            let direction = target - position;

            let movement_vector =
                direction.normalize() * grid_position.speed * time.delta_seconds();

            if movement_vector.length() < direction.length() {
                transform.translation += movement_vector.extend(0.0);
            } else {
                transform.translation = target.extend(transform.translation.z);
            }
        }
    }
}

fn update_rotation(mut query: Query<(&mut Transform, &GridRotation)>, time: Res<Time>) {
    for (mut transform, grid_rotation) in query.iter_mut() {
        let target = get_rotation(grid_rotation.rotation.clone());

        transform.rotation = Quat::from_rotation_z(target);
    }
}

pub fn cell_position(cell_coords: &IVec2) -> Vec2 {
    Vec2::new(cell_coords.x as f32, cell_coords.y as f32) * GRID_CELL_SIZE
}

pub fn get_rotation(rotation: Rotation) -> f32 {

    match rotation {
        Rotation::North => (0. as f32).to_radians(),
        Rotation::East => (90. as f32).to_radians(),
        Rotation::South => (180. as f32).to_radians(),
        Rotation::West => (270. as f32).to_radians(),
    }
}

/// First bottom-left, second upper-right
pub fn cell_bounds(cell_coords: &IVec2) -> [Vec2; 2] {
    let position = cell_position(cell_coords);

    return [
        position - Vec2::splat(GRID_CELL_SIZE / 2.),
        position + Vec2::splat(GRID_CELL_SIZE / 2.),
    ];
}
