pub mod instructions;

use bevy::prelude::*;

use crate::{
    grid::{cell_bounds, GridPosition, GridRotation, Rotation},
    loading::TextureAssets,
    GameState,
};

use self::instructions::Instruction;

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_robot)
            .add_systems(
                Update,
                (clicked_robot.pipe(edit_robot_code), move_robot)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component, Debug , Clone)]
pub struct Robot {
    pub instructions: Vec<Instruction>,
    index: usize,
}

impl Default for Robot {
    fn default() -> Self {
        Self { 
            instructions: vec![Instruction::TurnRight, Instruction::Walk, Instruction::TurnLeft],
            index: 0 
        }
    }
}

#[derive(Component, Default, Clone, Deref, DerefMut)]
struct MovementTimer(pub Timer);

#[derive(Bundle, Default, Clone)]
pub struct RobotBundle {
    robot: Robot,
    // temp
    movement_timer: MovementTimer,
    grid_position: GridPosition,
    grid_rotation: GridRotation,
    sprite_bundle: SpriteBundle,
}

fn setup_robot(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(RobotBundle {
        sprite_bundle: SpriteBundle {
            texture: textures.robot.clone(),
            ..default()
        },
        movement_timer: MovementTimer(Timer::from_seconds(5., TimerMode::Repeating)),
        ..default()
    });
}

fn move_robot(
    mut robot_query: Query<(&mut MovementTimer, &mut GridPosition, &mut GridRotation, &mut Robot)>,
    time: Res<Time>,
) {
    let (mut timer, mut robot_position, mut grid_rotation, mut robot) = robot_query.single_mut();
    

    timer.tick(time.delta());

    if timer.just_finished() {
        exec_instruction(&robot.instructions, robot_position.as_mut(), grid_rotation.as_mut(), robot.index);
        robot.index = robot.index + 1;
    }
}

fn exec_instruction(instructions: &Vec<Instruction>, grid_position: &mut GridPosition, grid_rotation: &mut GridRotation, mut inedx: usize,) {
    if let Some(instruction) = instructions.get(inedx) {
        match instruction {
            Instruction::Walk => {
                match grid_rotation.rotation {
                    Rotation::North => {grid_position.coords.y += 1},
                    Rotation::East => {grid_position.coords.x -= 1},
                    Rotation::South => {grid_position.coords.y -= 1},
                    Rotation::West => {grid_position.coords.x += 1},
                }
            },
            Instruction::TurnRight => {
                match grid_rotation.rotation {
                    Rotation::North => {grid_rotation.rotation = Rotation::West},
                    Rotation::East => {grid_rotation.rotation = Rotation::North},
                    Rotation::South => {grid_rotation.rotation = Rotation::East},
                    Rotation::West => {grid_rotation.rotation = Rotation::South},
                }
            },
            Instruction::TurnLeft => {
                match grid_rotation.rotation {
                    Rotation::North => {grid_rotation.rotation = Rotation::East},
                    Rotation::East => {grid_rotation.rotation = Rotation::South},
                    Rotation::South => {grid_rotation.rotation = Rotation::West},
                    Rotation::West => {grid_rotation.rotation = Rotation::North},
                }
            },
            Instruction::If { condition, instructions,  mut index} => {
                exec_instruction(instructions, grid_position, grid_rotation, inedx);
                index = index + 1;
            },
        }
    };
}

fn clicked_robot(
    robots: Query<(Entity, &GridPosition), With<Robot>>,
    windows: Query<&Window>,
    camera: Query<&Transform, With<Camera>>,
    mouse: Res<Input<MouseButton>>,
) -> Option<Entity> {
    if !mouse.just_pressed(MouseButton::Left) {
        return None;
    }

    let window = windows.single();
    let camera = camera.single();

    if let Some(cursor_position) = window.cursor_position() {
        let cursor_position = camera.translation.truncate()
            - Vec2::new(window.resolution.width(), window.resolution.height()) / 2.
            + cursor_position;

        let cursor_position = cursor_position * Vec2::new(1., -1.);

        for (robot, position) in robots.iter() {
            let bounds = cell_bounds(&position.coords);

            println!("{:?}", bounds);
            println!("{:?}", cursor_position);

            if (bounds[0].x..bounds[1].x).contains(&cursor_position.x)
                && (bounds[0].y..bounds[1].y).contains(&cursor_position.y)
            {
                return Some(robot);
            }
        }

        None
    } else {
        None
    }
}

fn edit_robot_code(clicked_robot: In<Option<Entity>>, mut robots: Query<&mut Robot>) {
    if let Some(robot) = clicked_robot.0 {
        if let Ok(robot) = robots.get_mut(robot) {
            println!("{:?}", robot.instructions);
        }
    }
}
