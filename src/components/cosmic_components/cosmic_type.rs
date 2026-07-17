#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum CosmicType {
    // Stars
    NeuronStar,
    DwarfStar,
    MidStar,
    LargeStar,
    // Planets
    DwarfPlanet,
    Planet,
    HabitablePlanet,
    GasPlanet,
    // systems
    LowSolarSystem,
    MidSolarSystem,
    HighSolarsystem,
    // other
    Nebula,
    Galaxy,
    None,
}

use super::cosmic_type::CosmicType::*;
impl CosmicType {
    fn upgrade(&mut self) -> CosmicType {
        match self {
            NeuronStar => DwarfStar,
            DwarfStar => MidStar,
            MidStar => LargeStar,

            DwarfPlanet => Planet,
            Planet => HabitablePlanet,
            HabitablePlanet => GasPlanet,

            _ => None,
        }
    }
}
