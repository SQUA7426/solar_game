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
}

use super::cosmic_type::CosmicType::*;
impl CosmicType {
    pub fn upgrade(&mut self) -> CosmicType {
        match self {
            NeuronStar => DwarfStar,
            DwarfStar => MidStar,
            MidStar => LargeStar,

            LargeStar => LargeStar,

            DwarfPlanet => Planet,
            Planet => HabitablePlanet,
            HabitablePlanet => GasPlanet,

            GasPlanet => GasPlanet,

            LowSolarSystem => MidSolarSystem,
            MidSolarSystem => HighSolarsystem,

            HighSolarsystem => HighSolarsystem,

            Nebula => Nebula,
            Galaxy => Galaxy,
        }
    }
}
