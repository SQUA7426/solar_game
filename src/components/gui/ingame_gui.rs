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
    use bevy::prelude::*;

    const NORMAL_TEXT: Color = Color::srgba(50., 50., 50., 0.75);
    const HOVER_TEXT: Color = Color::linear_rgba(10.5, 0., 5., 0.75);
    const PRESSED_TEXT: Color = Color::srgb(100., 0., 0.);

    const NORMAL_BG: Color = Color::srgba(0., 0., 0., 0.5);
    const HOVER_BG: Color = Color::srgb(0.5, 0., 25.);
    const PRESSED_BG: Color = Color::srgb(0., 0., 0.);

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
    pub enum DisplayState {
        DispCombine,
        DispCreateOrbit,
        DispInfo,
        DispUpgrade,
        DispFuse,
        #[default]
        All,
    }

    #[derive(Component, Clone, Copy)]
    enum InGameButtonAction {
        Combine,
        CreateOrbit,
        Fuse,
        Info,
        Upgrade,
    }

    #[derive(Component)]
    struct InGameBTNText;

    pub fn ingame_plugin(app: &mut App) {
        app.init_state::<DisplayState>()
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

    fn create_node(l: f32, r: f32, t: f32, b: Option<f32>, w: f32, h: f32) -> Node {
        Node {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            left: percent(l),
            right: percent(r),
            top: percent(t),
            bottom: match b {
                Some(bot) => percent(bot),
                None => Val::Auto,
            },
            width: percent(w),
            height: percent(h),
            ..default()
        }
    }

    pub fn button_setup(mut cmds: Commands) {
            cmds.spawn((
                DespawnOnExit(GameState::InGame),
                create_node(80., 0., 25., Some(25.), 20., 50.),
                children![(
                    Node {
                        width: percent(100.),
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        ..default()
                    },
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
                            )
                        }),
                    )),
                )],
            ));

            cmds.spawn((
                DespawnOnExit(GameState::InGame),
                create_node(30., 30., 80., None, 40., 20.),
                children![(
                    Node {
                        width: percent(100.),
                        flex_direction: FlexDirection::Row,
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,
                        ..default()
                    },
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
                            )
                        }),
                    )),
                )],
            ));
    }

    use DisplayState::*;

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
        mut disp_state: ResMut<NextState<DisplayState>>,
    ) {
        for (interaction, btn, mut bg_color, mut outline, children) in &mut button_query {
            let mut text_color = text_query.get_mut(children[0]).unwrap();

            if *interaction == Interaction::Pressed {
                match btn {
                    InGameButtonAction::Combine => {
                        disp_state.set(DispCombine);
                    }
                    InGameButtonAction::CreateOrbit => {
                        disp_state.set(DispCreateOrbit);
                    }
                    InGameButtonAction::Fuse => {
                        disp_state.set(DispFuse);
                    }
                    InGameButtonAction::Info => {
                        disp_state.set(DispInfo);
                    }
                    InGameButtonAction::Upgrade => {
                        disp_state.set(DispUpgrade);
                    }
                }
                *text_color = TextColor(PRESSED_TEXT);
                *bg_color = BackgroundColor(PRESSED_BG);
                outline.color = PRESSED_TEXT;
            }
            if *interaction == Interaction::Hovered {
                match btn {
                    InGameButtonAction::Combine => {
                        disp_state.set(DispCombine);
                    }
                    InGameButtonAction::CreateOrbit => {
                        disp_state.set(DispCreateOrbit);
                    }
                    InGameButtonAction::Fuse => {
                        disp_state.set(DispFuse);
                    }
                    InGameButtonAction::Info => {
                        disp_state.set(DispInfo);
                    }
                    InGameButtonAction::Upgrade => {
                        disp_state.set(DispUpgrade);
                    }
                }
                *text_color = TextColor(HOVER_TEXT);
                *bg_color = BackgroundColor(HOVER_BG);
                outline.color = HOVER_TEXT;
            }
            if *interaction == Interaction::None {
                *text_color = TextColor(NORMAL_TEXT);
                *bg_color = BackgroundColor(NORMAL_BG);
                outline.color = NORMAL_TEXT;
                disp_state.reset();
            }
        }
    }
}
