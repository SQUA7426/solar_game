use bevy::prelude::*;

use crate::GameState;

#[derive(Debug)]
pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ingame::ingame_plugin);
    }
}

mod ingame {
    use super::GameState;
    use bevy::{color::palettes::css::*, prelude::*};

    const NORMAL_TEXT: Color = Color::srgba(50., 50., 50., 0.75);
    const HOVER_TEXT: Color = Color::srgb(0., 255., 0.);
    const HOVER_PRESSED_TEXT: Color = Color::srgb(200., 200., 0.);
    const PRESSED_TEXT: Color = Color::srgb(100., 100., 0.);

    const NORMAL_BG: Color = Color::srgba(0., 0., 0., 0.75);
    const HOVER_BG: Color = Color::srgb(250., 250., 250.);
    const HOVER_PRESSED_BG: Color = Color::srgb(100., 100., 255.);
    const PRESSED_BG: Color = Color::srgb(255., 255., 255.);

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
    enum ButtonDisplay {
        DispCombine,
        DispCreateOrbit,
        DispInfo,
        DispUpgrade,
        DispFuse,
        #[default]
        All,
    }

    #[derive(Component)]
    pub enum ButtonType {
        Combine,
        Fuse,
        Upgrade,
        CreateOrbit,
        // SOMETHING ELSE
        Info,
    }

    #[derive(Component)]
    enum InGameButtonAction {
        Combine,
        CreateOrbit,
        Fuse,
        Info,
        Upgrade,
    }

    #[derive(Component)]
    struct InGameBTNText;

    #[derive(Resource)]
    struct ResetButtonTimer(Timer);

    pub fn ingame_plugin(app: &mut App) {
        app.init_state::<ButtonDisplay>()
            .insert_resource(ResetButtonTimer(Timer::from_seconds(
                0.5,
                TimerMode::Repeating,
            )))
            .add_systems(OnEnter(GameState::InGame), button_setup)
            .add_systems(Update, (button_system).run_if(in_state(GameState::InGame)));
    }

    fn create_btn(h: f32, w: f32, m: f32, radius: f32) -> Node {
        Node {
            height: percent(h),
            width: percent(w),
            margin: UiRect::all(percent(m)),
            border_radius: BorderRadius::all(px(radius)),
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            justify_items: JustifyItems::Center,
            align_content: AlignContent::Center,
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
            TextColor(NORMAL_TEXT),
        )
    }

    fn create_bottom_node() -> Node {
        Node {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            left: percent(30),
            right: percent(30),
            top: percent(80),
            width: percent(40),
            height: percent(20),
            ..default()
        }
    }

    fn create_right_node() -> Node {
        Node {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            left: percent(80),
            right: percent(0),
            top: percent(25),
            bottom: percent(25),
            width: percent(20),
            height: percent(50),
            ..default()
        }
    }

    #[derive(Component)]
    struct CreateButton;

    pub fn button_setup(mut cmds: Commands) {
        cmds.spawn((
            DespawnOnExit(GameState::InGame),
            // create_bottom_node(),
            create_right_node(),
            // BackgroundColor(Color::linear_rgb(0., 200., 0.)),
            children![(
                Node {
                    width: percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    ..default()
                },
                // BackgroundColor(Color::linear_rgb(0., 0., 50.)),
                Children::spawn(SpawnIter(
                    [
                        (
                            InGameButtonAction::Info,
                            75.,
                            90.,
                            95.,
                            6.,
                            "INFO",
                            NORMAL_BG
                        ),
                        (
                            InGameButtonAction::CreateOrbit,
                            75.,
                            90.,
                            95.,
                            6.,
                            "Orbit",
                            NORMAL_BG
                        ),
                    ]
                    .into_iter()
                    .map(move |(bt, h, w, r, m, text, c)| {
                        (
                            Button,
                            bt,
                            create_btn(h, w, m, r),
                            children![(create_btn_text(text.into()), InGameBTNText)],
                            BackgroundColor(c),
                            Outline {
                                width: px(6),
                                offset: px(0),
                                color: NORMAL_TEXT,
                            },
                            CreateButton,
                        )
                    }),
                )),
            )],
        ));

        cmds.spawn((
            DespawnOnExit(GameState::InGame),
            create_bottom_node(),
            // BackgroundColor(Color::linear_rgb(0., 200., 0.)),
            children![(
                Node {
                    width: percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    ..default()
                },
                // BackgroundColor(Color::linear_rgb(0., 0., 50.)),
                Children::spawn(SpawnIter(
                    [
                        (
                            InGameButtonAction::Fuse,
                            75.,
                            125.,
                            95.,
                            6.,
                            "Fuse",
                            NORMAL_BG
                        ),
                        (
                            InGameButtonAction::Upgrade,
                            75.,
                            125.,
                            95.,
                            6.,
                            "Upgrade",
                            NORMAL_BG
                        ),
                        (
                            InGameButtonAction::Combine,
                            75.,
                            125.,
                            95.,
                            6.,
                            "Combine",
                            NORMAL_BG
                        ),
                    ]
                    .into_iter()
                    .map(move |(bt, h, w, r, m, text, c)| {
                        (
                            Button,
                            bt,
                            create_btn(h, w, m, r),
                            children![(create_btn_text(text.into()), InGameBTNText)],
                            BackgroundColor(c),
                            Outline {
                                width: px(6),
                                offset: px(0),
                                color: NORMAL_TEXT,
                            },
                            CreateButton,
                        )
                    }),
                )),
            )],
        ));
    }

    fn button_system(
        mut button_query: Query<
            (
                &Interaction,
                &InGameButtonAction,
                &mut BackgroundColor,
                &mut Outline,
                &Children,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        mut text_query: Query<&mut TextColor>,
        mut btn_timer: ResMut<ResetButtonTimer>,
        time: Res<Time>,
    ) {
        for (interaction, btn, mut bg_color, mut outline, children) in &mut button_query {
            let mut text_color = text_query.get_mut(children[0]).unwrap();

            if *interaction == Interaction::Pressed {
                // match btn {
                //     InGameButtonAction::Combine => {
                //         println!("Pressed!");
                //     }
                //     InGameButtonAction::CreateOrbit => {
                //         println!("Pressed!");
                //     }
                //     InGameButtonAction::Fuse => {
                //         println!("Pressed!");
                //     }
                //     InGameButtonAction::Info => {
                //         println!("Pressed!");
                //     }
                //     InGameButtonAction::Upgrade => {
                //         println!("Pressed!");
                //     }
                // }
                *text_color = TextColor(PRESSED_TEXT);
                *bg_color = BackgroundColor(PRESSED_BG);
                outline.color = PRESSED_TEXT;

                if btn_timer.0.tick(time.delta()).just_finished() {
                    *text_color = TextColor(NORMAL_TEXT);
                    *bg_color = BackgroundColor(NORMAL_BG);
                    outline.color = NORMAL_TEXT;
                }
            }
            if *interaction == Interaction::Hovered {
                println!("Hovered!");
                // *text_color = TextColor(HOVER_TEXT);
                // *bg_color = BackgroundColor(HOVER_BG);
                // outline.color = HOVER_TEXT;
            }
            if *interaction == Interaction::None {
                *text_color = TextColor(NORMAL_TEXT);
                *bg_color = BackgroundColor(NORMAL_BG);
                outline.color = NORMAL_TEXT;
            }
        }
    }
}
