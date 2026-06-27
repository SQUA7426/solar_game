use bevy::prelude::*;

use crate::GameState;

#[derive(Debug)]
pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), ingame::ingame_setup);
    }
}

mod ingame {
    use bevy::prelude::*;

    pub fn ingame_setup(mut cmds: Commands) {

    }

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
        NotCombine,
        Combine,
        Fuse,
        Upgrade,
        CreateOrbit,
        // SOMETHING ELSE
        INFO,
    }

    #[derive(Component)]
    struct InGameBTNText;

    fn create_btn(w: f32, h:f32, m: f32, color: Color, pos: JustifySelf) -> Node {
        Node {
            width: px(w),
            height: px(h),
            // margin: UiRect { left: (), right: (), top: (), bottom: () }
            margin: UiRect::all(px(m)),
            justify_self: pos,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }
    fn create_btn_text(text: String, color: Color) -> (Text, TextFont, TextColor) {
        (
            Text::new(text),
            TextFont {
                font_size: 22.,
                ..default()
            },
            TextColor(color)
        )
    }

    fn button_setup(mut cmds: Commands) {

    }
}
