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
    use crate::{CosmicCombine, CosmicEntity};
    use rust_decimal::prelude::*;

    use super::GameState;
    use bevy::prelude::ops::sqrt;
    use bevy::prelude::*;
    use itertools::Itertools;

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
        DispFuse,
        DispInfo,
        DispUpgrade,
        #[default]
        All,
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
    pub enum IngameState {
        Combine,
        Fuse,
        Info,
        Orbit,
        Upgrade,
        #[default]
        Default,
    }

    #[derive(Component, Clone, Copy, Default)]
    enum InGameButtonAction {
        Combine,
        CreateOrbit,
        Fuse,
        Info,
        Upgrade,
        #[default]
        Standby,
    }
    use DisplayState::*;
    use IngameState::*;

    #[derive(Component)]
    struct InGameBTNText;

    pub fn ingame_plugin(app: &mut App) {
        app.init_state::<DisplayState>()
            .init_state::<IngameState>()
            .add_systems(OnEnter(GameState::InGame), button_setup)
            .add_systems(
                Update,
                (button_system, button_action).run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                create_combine.run_if(in_state(IngameState::Combine)),
            )
            .add_systems(Update, display_combine.run_if(in_state(DispCombine)))
            .add_systems(OnEnter(IngameState::Upgrade), upgrade_entities)
            .add_systems(OnEnter(DispInfo), show_info);
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

    fn button_system(
        mut btn_query: Query<
            (&Interaction, &mut BackgroundColor, &mut Outline, &Children),
            (Changed<Interaction>, With<Button>),
        >,
        mut text_query: Query<&mut TextColor>,
    ) {
        for (interaction, mut bg_color, mut outline, children) in &mut btn_query {
            let mut text_color = text_query.get_mut(children[0]).unwrap();

            if *interaction == Interaction::Pressed {
                *text_color = TextColor(PRESSED_TEXT);
                *bg_color = BackgroundColor(PRESSED_BG);
                outline.color = PRESSED_TEXT;
            }
            if *interaction == Interaction::Hovered {
                *text_color = TextColor(HOVER_TEXT);
                *bg_color = BackgroundColor(HOVER_BG);
                outline.color = HOVER_TEXT;
            }
            if *interaction == Interaction::None {
                *text_color = TextColor(NORMAL_TEXT);
                *bg_color = BackgroundColor(NORMAL_BG);
                outline.color = NORMAL_TEXT;
            }
        }
    }

    fn button_action(
        mut btn_query: Query<
            (&Interaction, &InGameButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut disp_state: ResMut<NextState<DisplayState>>,
        mut ingame_state: ResMut<NextState<IngameState>>,
    ) {
        for (interaction, btn) in &mut btn_query {
            if *interaction == Interaction::Pressed {
                match btn {
                    InGameButtonAction::Combine => ingame_state.set(Combine),
                    InGameButtonAction::CreateOrbit => ingame_state.set(Orbit),
                    InGameButtonAction::Fuse => ingame_state.set(Fuse),
                    InGameButtonAction::Info => ingame_state.set(Info),
                    InGameButtonAction::Upgrade => ingame_state.set(Upgrade),
                    _ => {}
                }
            }

            if *interaction == Interaction::Hovered {
                match btn {
                    InGameButtonAction::Combine => disp_state.set(DispCombine),
                    InGameButtonAction::CreateOrbit => disp_state.set(DispCreateOrbit),
                    InGameButtonAction::Fuse => disp_state.set(DispFuse),
                    InGameButtonAction::Info => disp_state.set(DispInfo),
                    InGameButtonAction::Upgrade => disp_state.set(DispUpgrade),
                    _ => {}
                }
            }

            if *interaction == Interaction::None {
                disp_state.reset();
                ingame_state.reset();
            }
        }
    }

    fn selected_entities_production_rate(
        selected_query: Option<Query<&CosmicEntity>>,
    ) -> Option<String> {
        let Some(selected_query) = selected_query else {
            return None;
        };

        let mut sum = 0.;
        for entity in &selected_query {
            if entity.selected {
                sum += entity.production_rate;
            }
        }
        Some(sum.to_string())
    }

    fn display_combine(
        mut cmds: Commands,
        // mut disp_state: ResMut<NextState<DisplayState>>,
        selected_query: Option<Query<&CosmicEntity>>,
    ) {
        let mut s = None;
        if selected_query.clone().is_some() {
            s = selected_entities_production_rate(selected_query);
        }
        let sub_s = match s {
            Some(sel) => sel,
            None => "Combine".into(),
        };

        cmds.spawn((
            DespawnOnExit(DispCombine),
            create_node(30., 30., 73., Some(7.), 40., 20.),
            // BackgroundColor(Color::srgb(0., 255., 0.)),
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
                    [(InGameButtonAction::Standby, 75., 125., 95., 6., sub_s,),]
                        .into_iter()
                        .map(move |(bt, h, w, r, m, text)| {
                            (
                                Button,
                                bt,
                                create_btn(h, w, m, r),
                                children![(
                                    Text::new(text),
                                    TextFont {
                                        font_size: 22.,
                                        ..default()
                                    },
                                    TextColor(Color::srgba(0., 0., 0., 0.),),
                                    InGameBTNText
                                )],
                                BackgroundColor(Color::srgba(0., 0., 0., 0.)),
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

    fn list_selected_entities_pos(
        selected_query: &Option<
            Query<(
                Entity,
                &mut CosmicEntity,
                &mut Transform,
                &MeshMaterial2d<ColorMaterial>,
            )>,
        >,
    ) -> Option<String> {
        let Some(selected_query) = selected_query else {
            return None;
        };

        let mut positions = String::new();
        for (_e, entity, _, _) in selected_query {
            if entity.selected {
                positions += &format!("{} {} ", entity.pos.x, entity.pos.y);
            }
        }
        Some(positions)
    }

    // MATH / PHYSICS TypeShit
    fn calc_centroid(positions: Vec<Vec2>) -> Vec2 {
        let n = positions.len();
        positions.iter().fold(Vec2::ZERO, |pre, pos| pre + pos) / n as f32
    }

    fn calc_avg_radius(positions: Vec<Vec2>, centroid: Vec2) -> f32 {
        let n = positions.len();
        positions
            .iter()
            .map(|point| point.distance(centroid))
            .sum::<f32>()
            / n as f32
            * 1.25
    }

    fn calc_intersect_2_circle(
        x1: f32,
        y1: f32,
        r1: f32,
        x2: f32,
        y2: f32,
        r2: f32,
        d: f32,
    ) -> (Vec2, Vec2) {
        let a = (r1.powi(2) - r2.powi(2) + d.powi(2)) / (2. * d);
        // println!("a: {}", a);
        // let b = (r2.powi(2) - r1.powi(2) + d.powi(2)) / 2. * d; TRIANGLE
        let h1 = Decimal::new(r1.powi(2) as i64, 1).to_f32().unwrap();
        let h2 = Decimal::new(a.powi(2) as i64, 1).to_f32().unwrap();
        // println!("h1: {}, h2: {}", h1, h2);
        // FAILS
        let h: f32 = sqrt(h1 - h2);
        // println!("h: {:#?}", h);

        let x5 = x1 + a / d * (x2 - x1);
        // println!("x5: {}", x5);
        let y5 = y1 + a / d * (y2 - y1);
        // println!("y5: {}", y5);
        let p5: Vec2 = Vec2::new(x5, y5);
        // println!("p5: {}", p5);

        let p3: Vec2 = Vec2::new(
            p5.x - ((h as f32 * (y2 - y1)) / d),
            p5.y + ((h as f32 * (x2 - x1)) / d),
        );
        let p4: Vec2 = Vec2::new(
            p5.x + ((h as f32 * (y2 - y1)) / d),
            p5.y - ((h as f32 * (x2 - x1)) / d),
        );
        // println!("p3: {}, {}", p3.x, p3.y);
        // println!("p4: {}, {}", p4.x, p4.y);
        (p3, p4)
    }

    fn move_cosmic_entity_to_outer_circle(ce: &mut CosmicEntity, centroid: Vec2, avg_radius: f32) {
        let dist = centroid.distance(ce.pos).abs();

        let (new_pos1, _new_pos2) = calc_intersect_2_circle(
            centroid.x, centroid.y, avg_radius, ce.pos.x, ce.pos.y, ce.radius, dist,
        );

        ce.pos = new_pos1;
    }

    fn create_combine(
        mut cmds: Commands,
        selected_query: Option<
            Query<(
                Entity,
                &mut CosmicEntity,
                &mut Transform,
                &MeshMaterial2d<ColorMaterial>,
            )>,
        >,
        mut ingame_state: ResMut<NextState<IngameState>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let sel = list_selected_entities_pos(&selected_query);

        let mut cc = CosmicCombine::new();

        match sel {
            Some(positions) => {
                let mut pos_s: Vec<Vec2> = Vec::new();
                let pos: Vec<String> = positions
                    .split_whitespace()
                    .map(|ws| String::from(ws))
                    .collect::<Vec<String>>();
                for (p1, p2) in pos.into_iter().tuples() {
                    // println!("position: {}, {}", p1, p2);
                    pos_s.push(Vec2::new(
                        p1.parse::<f32>().unwrap(),
                        p2.parse::<f32>().unwrap(),
                    ));
                }

                let n = pos_s.len();

                if n > 1 {
                    // println!("Len > 1...");
                    let centroid = calc_centroid(pos_s.clone());
                    cc.set_midpoint(centroid);
                    let avg_radius = calc_avg_radius(pos_s, centroid);

                    let Some(selected_query) = selected_query else {
                        return;
                    };

                    let cc_entities = cmds
                        .spawn((
                            Mesh2d(meshes.add(Circle::new(avg_radius))),
                            // MeshMaterial2d(materials.add(Color::srgba(0., 255., 0., 0.25))),
                            Transform::from_translation(Vec3::new(centroid.x, centroid.y, 20.)),
                        ))
                        .id();

                    for (e_num, (entity, mut ce, mut transform, meshes)) in
                        &mut selected_query.into_iter().enumerate()
                    {
                        move_cosmic_entity_to_outer_circle(&mut ce, centroid, avg_radius);
                        // println!("CE pos: {}, {}", ce.clone().pos.x, ce.clone().pos.y);
                        if ce.selected {
                            ce.selected = false;
                            if let Some(material) = materials.get_mut(&meshes.0) {
                                let idx = i32::from_usize(e_num);
                                // println!("{:?}", idx);
                                match idx {
                                    None => material.color = Color::linear_rgb(0., 0., 0.),
                                    Some(i) => {
                                        if i % 2 == 0 {
                                            material.color = Color::linear_rgb(50., 50., 0.); // YELLOW
                                        } else {
                                            material.color = Color::linear_rgb(50., 50., 50.); // WHITE
                                        }
                                    }
                                }
                            }

                            let local_offset = ce.pos - centroid;

                            transform.translation.x = local_offset.x;
                            transform.translation.y = local_offset.y;

                            cc.push(ce.clone());
                            cmds.entity(cc_entities).add_child(entity);
                        }
                    }

                    cmds.entity(cc_entities).insert(cc);
                    println!("Orbit Circle created!");
                }
            }
            None => {}
        }

        ingame_state.reset();
    }

    fn upgrade_entities(
        mut ingame_state: ResMut<NextState<IngameState>>,
        ce_query: Option<Query<&mut CosmicEntity>>,
    ) {
        let Some(mut ce_query) = ce_query else { return };

        ce_query.iter_mut().for_each(|mut ce| {
            if ce.selected {
                ce.entity_type.upgrade();
            }
            println!("CE.entity_type after Update: {:?}", ce.entity_type);
        });
        ingame_state.reset();
    }

    fn show_info(
        mut cmds: Commands,
        ce_query: Option<Query<&CosmicEntity>>,
        mut disp_state: ResMut<NextState<DisplayState>>,
    ) {
        let Some(ce_query) = ce_query else { return };

        let len = ce_query
            .iter()
            .filter(|ce| ce.selected == true)
            .collect_vec()
            .len();
        if len > 1 {
            return;
        }
        ce_query.iter().for_each(|ce| {
            if ce.clone().selected {
                cmds.spawn((
                    DespawnOnExit(DisplayState::DispInfo),
                    create_node(80., 20., 0., Some(60.), 20., 40.),
                    BackgroundColor(Color::linear_rgb(0., 0., 0.)),
                    create_btn_text(format!("{}", ce)),
                ));
            }
        });
    }
}
