pub mod components {
    pub mod cosmic_components {
        pub mod cosmic_entity;
        pub mod cosmic_resource;
        pub mod cosmic_type;
    }
    pub mod gui {
        pub mod debug_text;
        pub mod game_menu;
        pub mod ingame_gui;
    }
    pub mod mouse;
    pub mod player;
    pub mod tilemap;
}

pub use components::{
    cosmic_components::{cosmic_entity::*, cosmic_resource::*, cosmic_type::CosmicType},
    gui::{debug_text::*, game_menu::*, ingame_gui::*},
    mouse::*,
    player::*,
    tilemap::*,
};
