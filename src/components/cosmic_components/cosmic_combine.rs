use bevy::prelude::*;

use crate::{CosmicCombineSpinningTimer, CosmicEntity, GameState};

#[derive(Component)]
pub struct CosmicCombine {
    pub entities: Vec<CosmicEntity>,
    pub midpoint: Vec2,
}

impl CosmicCombine {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            midpoint: Vec2::ZERO,
        }
    }

    pub fn push(&mut self, ce: CosmicEntity) {
        self.entities.push(ce);
    }

    pub fn set_midpoint(&mut self, centroid: Vec2) {
        self.midpoint = centroid;
    }

}

fn spinning(
    cc_query: Option<
    Query<(
    &mut CosmicCombine,
    &mut Transform)>>,
    mut timer: ResMut<CosmicCombineSpinningTimer>,
    time: Res<Time>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let Some(cc_query) = cc_query else { return };

    for (_cc, mut transform) in cc_query {
        transform.rotate_z(1.);
    }
}

pub struct CosmicCombinePlugin;

impl Plugin for CosmicCombinePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CosmicCombineSpinningTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
            .add_systems(Update, spinning.run_if(in_state(GameState::InGame)));
    }
}
