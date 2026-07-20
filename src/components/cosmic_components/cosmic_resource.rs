use bevy::prelude::{Resource, Timer};

#[allow(unused)]
// ESSENCE
#[derive(Resource, Clone, Debug)]
pub struct CosmicEssence {
    pub value: f32,
}

impl CosmicEssence {
    pub fn new(val: f32) -> Self {
        Self { value: val }
    }
}

#[derive(Resource)]
pub struct CeCreationTimer(pub Timer);

#[derive(Resource)]
pub struct CosmicCombineSpinningTimer(pub Timer);

#[derive(Resource)]
pub struct ChangeResource;
