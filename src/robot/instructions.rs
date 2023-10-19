use bevy::{prelude::*, utils::HashMap};

use crate::ui::InstructionBankUiMarker;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Walk,
    TurnRight,
    TurnLeft,
    If {
        condition: Condition,
        instructions: Vec<Instruction>,
    },
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    // TODO
    True,
}

pub enum InstructionQuantity {
    Value(u32),
    Infinite,
}

#[derive(Resource)]
pub struct InstructionBank {
    pub instructions: HashMap<Instruction, InstructionQuantity>,
}

pub fn update_instruction_bank_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<InstructionBankUiMarker>>,
    ui_children_query: Query<&Children, With<InstructionBankUiMarker>>,
    instruction_bank: Res<InstructionBank>,
) {
    if let Ok(children) = ui_children_query.get_single() {
        for child in children.iter() {
            commands.entity(*child).despawn_recursive();
        }
    }

    let ui = ui_query.single();

    for (instruction, quantity) in instruction_bank.instructions.iter() {
        commands.entity(ui).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::axes(Val::Px(16.), Val::Px(8.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    instruction_ui(parent, instruction);

                    let quantity_str = match quantity {
                        InstructionQuantity::Value(num) => {
                            format!("{}", num)
                        }
                        InstructionQuantity::Infinite => "inf".to_string(),
                    };

                    parent.spawn(TextBundle::from_section(
                        quantity_str.as_str(),
                        TextStyle {
                            font_size: 27.0,
                            color: Color::rgb(1., 1., 1.),
                            ..default()
                        },
                    ));
                });
        });
    }
}

pub fn instruction_ui(commands: &mut ChildBuilder, instruction: &Instruction) -> Entity {
    let instruction_height_px = 32.;

    match instruction {
        Instruction::Walk => commands
            .spawn(ButtonBundle {
                background_color: Color::hex("037bfc").unwrap().into(),
                style: Style {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    width: Val::Px(128.),
                    height: Val::Px(instruction_height_px),
                    padding: UiRect::horizontal(Val::Px(8.)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "walk",
                    TextStyle {
                        font_size: 21.0,
                        color: Color::rgb(1., 1., 1.),
                        ..default()
                    },
                ));
            })
            .id(),
        Instruction::TurnLeft => commands
            .spawn(ButtonBundle {
                background_color: Color::hex("037bfc").unwrap().into(),
                style: Style {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    width: Val::Px(128.),
                    height: Val::Px(instruction_height_px),
                    padding: UiRect::horizontal(Val::Px(8.)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "turn left",
                    TextStyle {
                        font_size: 21.0,
                        color: Color::rgb(1., 1., 1.),
                        ..default()
                    },
                ));
            })
            .id(),
        Instruction::TurnRight => commands
            .spawn(ButtonBundle {
                background_color: Color::hex("037bfc").unwrap().into(),
                style: Style {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    width: Val::Px(128.),
                    height: Val::Px(instruction_height_px),
                    padding: UiRect::horizontal(Val::Px(8.)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "turn right",
                    TextStyle {
                        font_size: 21.0,
                        color: Color::rgb(1., 1., 1.),
                        ..default()
                    },
                ));
            })
            .id(),
        Instruction::If {
            condition: _,
            instructions,
        } => commands
            .spawn(ButtonBundle {
                background_color: Color::hex("fcc203").unwrap().into(),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    min_width: Val::Px(128.),
                    height: Val::Px(instruction_height_px),
                    padding: UiRect::horizontal(Val::Px(8.)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.),
                            height: Val::Px(instruction_height_px),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "if",
                            TextStyle {
                                font_size: 21.0,
                                color: Color::rgb(1., 1., 1.),
                                ..default()
                            },
                        ));
                    });

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::left(Val::Px(16.)),
                            min_height: Val::Px(instruction_height_px),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        if instructions.is_empty() {
                            parent.spawn(NodeBundle {
                                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                                style: Style {
                                    width: Val::Percent(100.),
                                    height: Val::Px(instruction_height_px),
                                    ..default()
                                },
                                ..default()
                            });
                        } else {
                            for instruction in instructions {
                                instruction_ui(parent, &instruction);
                            }
                        }
                    });
            })
            .id(),
    }
}
