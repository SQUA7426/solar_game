use bevy::prelude::*;

use crate::GameState;

#[derive(Debug)]
pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), ingame::button_setup);
    }
}

mod ingame {
    use super::GameState;
    use bevy::prelude::*;

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
    enum ButtonDisplay {
        DispInfo,
        DispUpgrade,
        DispFuse,
        #[default]
        All,
    }

    #[derive(Component)]
    pub enum Button {
        Combine,
        Fuse,
        Upgrade,
        CreateOrbit,
        // SOMETHING ELSE
        INFO,
    }

    #[derive(Component)]
    struct InGameBTNText;

    fn create_btn(w: f32, h: f32, m: f32) -> Node {
        Node {
            width: percent(w),
            height: percent(h),
            // margin: UiRect { left: (), right: (), top: (), bottom: () }
            margin: UiRect::all(px(m)),
            justify_content: JustifyContent::Center,
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }
    fn create_btn_text(text: String) -> (Text, TextFont, TextColor) {
        (
            Text::new(text),
            TextFont {
                font_size: 22.,
                ..default()
            },
            TextColor(Color::linear_rgb(0., 0., 0.)),
        )
    }

    fn create_bot_node() -> Node {
        Node {
            align_items: AlignItems::Center,
            left: percent(20),
            right: percent(20),
            top: percent(80),
            width: percent(60),
            height: percent(20),
            ..default()
        }
    }

    #[derive(Component)]
    struct CreateButton;

    pub fn button_setup(mut cmds: Commands) {
        cmds.spawn((
            DespawnOnExit(GameState::InGame),
            create_bot_node(),
            CreateButton,
            children![(
                Node {
                    flex_direction: FlexDirection::Row,
                    align_content: AlignContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::linear_rgb(0., 0., 50.)),
                Children::spawn(SpawnIter(
                    [
                        (75., 45., 30., "Fuse"),
                        (75., 45., 30., "Upgrade"),
                        (75., 45., 30., "Combine"),
                    ]
                    .into_iter()
                    .map(move |(w, h, m, text)| {
                        (
                            Button,
                            create_btn(w, h, m),
                            create_btn_text(text.into()),
                        )
                    }),
                )),
            )],
        ));
    }
}
