pub mod instructions;

use bevy::{prelude::*, utils::HashMap};

use crate::{
    grid::{cell_bounds, GridPosition},
    loading::TextureAssets,
    GameState,
};

use self::instructions::{
    update_instruction_bank_ui, Instruction, InstructionBank, InstructionQuantity,
};

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InstructionBank {
            instructions: {
                let mut instructions = HashMap::new();

                instructions.insert(Instruction::Walk, InstructionQuantity::Value(3));
                instructions.insert(Instruction::TurnLeft, InstructionQuantity::Infinite);
                instructions.insert(
                    Instruction::If {
                        condition: instructions::Condition::True,
                        instructions: Vec::new(),
                    },
                    InstructionQuantity::Infinite,
                );

                instructions
            },
        })
        .add_systems(OnEnter(GameState::Playing), setup_robot)
        .add_systems(
            Update,
            (
                clicked_robot.pipe(edit_robot_code),
                move_robot,
                update_instruction_bank_ui,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct Robot {
    pub instructions: Vec<Instruction>,
}

#[derive(Component, Default, Clone, Deref, DerefMut)]
struct MovementTimer(pub Timer);

#[derive(Bundle, Default, Clone)]
pub struct RobotBundle {
    robot: Robot,
    // temp
    movement_timer: MovementTimer,
    grid_position: GridPosition,
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
    mut robot_query: Query<(&mut MovementTimer, &mut GridPosition), With<Robot>>,
    time: Res<Time>,
) {
    let (mut timer, mut robot_position) = robot_query.single_mut();

    timer.tick(time.delta());

    if timer.just_finished() {
        robot_position.coords.y += 1;
    }
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
