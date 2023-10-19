use bevy::prelude::*;

use crate::{util::cleanup, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameUiMarker>()
            .register_type::<InstructionBankUiMarker>()
            .add_systems(OnEnter(GameState::Playing), setup_ui)
            .add_systems(OnExit(GameState::Playing), cleanup::<GameUiMarker>);
    }
}

#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
struct GameUiMarker;

#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub struct InstructionBankUiMarker;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::RowReverse,
                position_type: PositionType::Absolute,
                right: Val::Px(0.),
                top: Val::Px(0.),
                bottom: Val::Px(0.),
                ..default()
            },
            ..default()
        })
        .insert(GameUiMarker)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::axes(Val::Px(8.), Val::Px(32.)),
                        width: Val::Px(248.),
                        ..default()
                    },
                    ..default()
                })
                .insert(InstructionBankUiMarker);

            println!("aaaa");
        });
}
